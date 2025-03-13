extern crate alloc;

use core::slice::Iter;

use hal::aes::{AesBlock, AesKey};
use hal::flc::Flc;
use hal::flc::FlashError;

#[derive(Debug, Clone, Copy)]
pub enum SecureMemoryError {
    InvalidSubscriptionChannel(u32),
    SubscriptionNotValid(u32),
    SubscriptionMemoryFull,
    NoMasterSecret,
    FlashError(FlashError)
}

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct Subscription {
    pub channel_id: u32,
    pub valid: bool,
    pub end: u64,
    pub start: u64
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecretType {
    Channel(u32),
    Master
}

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct Secret {
    pub secret_type: SecretType,
    pub valid: bool,
    pub aes_key: AesKey,
    pub aes_iv: AesBlock
}

extern "C" {
    static subscriptions_address: u32;
    static decoder_id_address: u32;
    static secrets_address: u32;
}

const SUBSCRIPTIONS_CAPACITY: usize = 8;
const SECRETS_CAPACITY: usize = 128;

fn find_const_time<P, T>(mut it: Iter<T>, predicate: P) -> Option<T>
where
    T: Sized + Copy,
    P: Fn(&T) -> bool,
{
    let mut item = None;
    while let Some(x) = it.next() {
        if predicate(x) {
            item = Some(*x);
        }
    }
    item
}

fn position_const_time<P, T>(it: Iter<T>, predicate: P) -> Option<usize>
where
    T: Sized,
    P: Fn(&T) -> bool,
{
    let mut index = None;
    let mut enumit = it.enumerate();
    while let Some((i, x)) = enumit.next() {
        if predicate(x) {
            index = Some(i);
        }
    }
    index
}

fn retrieve_subscriptions_array(flc: &Flc) -> Result<[Subscription; SUBSCRIPTIONS_CAPACITY], SecureMemoryError> {
    unsafe {
        let subscriptions = flc.read_t::<[Subscription; SUBSCRIPTIONS_CAPACITY]>(subscriptions_address);
        if subscriptions.is_err() { return Err(SecureMemoryError::FlashError(subscriptions.unwrap_err())); }
        Ok(subscriptions.unwrap())
    }
}

pub fn retrieve_subscription(flc: &Flc, channel_id: u32) -> Result<Option<Subscription>, SecureMemoryError> {
    let subscriptions = retrieve_subscriptions(flc);
    if subscriptions.is_err() { return Err(subscriptions.unwrap_err()); }
    Ok(find_const_time(subscriptions.unwrap().iter(), |sub| sub.valid && sub.channel_id == channel_id))
}

pub fn retrieve_subscriptions(flc: &Flc) -> Result<alloc::vec::Vec<Subscription>, SecureMemoryError> {
    let subscriptions = retrieve_subscriptions_array(flc);
    if subscriptions.is_err() { return Err(subscriptions.unwrap_err()); }
    Ok(subscriptions.unwrap().to_vec())
}

pub fn overwrite_subscription(flc: &Flc, subscription: Subscription) -> Result<(), SecureMemoryError> {
    if subscription.channel_id == 0 { return Err(SecureMemoryError::InvalidSubscriptionChannel(subscription.channel_id)); }
    if !subscription.valid { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    if subscription.end < subscription.start { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    let subscriptions = retrieve_subscriptions_array(flc);
    if subscriptions.is_err() { return Err(subscriptions.unwrap_err()); }
    let mut subscriptions = subscriptions.unwrap();
    unsafe {
        let open_slot = SUBSCRIPTIONS.iter().position(|x| !x.valid);
        if open_slot.is_none() { return Err(SecureMemoryError::SubscriptionMemoryFull); }
        let open_slot = open_slot.unwrap();
        SUBSCRIPTIONS[open_slot] = subscription;
        Ok(())
    }
}

fn retrieve_secrets_array(flc: &Flc) -> Result<[Secret; SECRETS_CAPACITY], SecureMemoryError> {
    unsafe {
        let secrets = flc.read_t::<[Secret; SECRETS_CAPACITY]>(secrets_address);
        if secrets.is_err() { return Err(SecureMemoryError::FlashError(secrets.unwrap_err())); }
        Ok(secrets.unwrap())
    }
}

pub fn retrieve_channel_secret(flc: &Flc, channel_id: u32) -> Result<Option<Secret>, SecureMemoryError> {
    let secrets = retrieve_secrets_array(flc);
    if secrets.is_err() { return Err(secrets.unwrap_err()); }
    Ok(find_const_time(secrets.unwrap().iter(), |sec| sec.valid && sec.secret_type == SecretType::Channel(channel_id)))
}

pub fn retrieve_master_secret(flc: &Flc) -> Result<Secret, SecureMemoryError> {
    let secrets = retrieve_secrets_array(flc);
    if secrets.is_err() { return Err(secrets.unwrap_err()); }
    let master_secret = find_const_time(secrets.unwrap().iter(), |sec| sec.valid && sec.secret_type == SecretType::Master);
    match master_secret {
        Some(master_secret) => Ok(master_secret),
        None => Err(SecureMemoryError::NoMasterSecret)
    }
}

pub fn verify_decoder_id(flc: &Flc, decoder_id: u32) -> Result<bool, SecureMemoryError> {
    unsafe {
        let saved_decoder_id = flc.read_32(decoder_id_address);
        if saved_decoder_id.is_err() { return Err(SecureMemoryError::FlashError(saved_decoder_id.unwrap_err())); }
        Ok(saved_decoder_id.unwrap() == decoder_id)
    }
}