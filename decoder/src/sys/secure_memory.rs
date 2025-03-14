extern crate alloc;

use alloc::vec::Vec;

use hal::aes::{AesBlock, AesKey};
use hal::flc::Flc;
use hal::flc::FlashError;

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum SecureMemoryError {
    InvalidSubscriptionChannel(u32),
    SubscriptionNotValid(u32),
    SubscriptionMemoryFull,
    NoMasterSecret,
    FlashError(FlashError)
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4))]
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
#[repr(C, align(4))]
pub struct Secret {
    pub secret_type: SecretType,
    pub valid: bool,
    pub aes_key: AesKey,
    pub aes_iv: AesBlock
}

// extern "C" {
//     static subscriptions_address: u32;
//     static decoder_id_address: u32;
//     static secrets_address: u32;
// }

static subscriptions_address: u32 = 0x1007a000;
static decoder_id_address: u32 = 0x1007c000;
static secrets_address: u32 = 0x1007c004;

const SUBSCRIPTIONS_CAPACITY: usize = 8;
const SECRETS_CAPACITY: usize = 128;

pub fn retrieve_subscription(flc: &Flc, channel_id: u32) -> Result<Option<Subscription>, SecureMemoryError> {
    let mut subscription: Option<Subscription> = None;
    for i in 0..SUBSCRIPTIONS_CAPACITY {
        unsafe {
            let sub = flc.read_t::<Subscription>(subscriptions_address + (i * size_of::<Subscription>()) as u32);
            if sub.is_err() { return Err(SecureMemoryError::FlashError(sub.unwrap_err())); }
            let sub = sub.unwrap();
            if sub.valid && sub.channel_id == channel_id {
                subscription = Some(sub)
            }
        }
    }
    Ok(subscription)
}

pub fn retrieve_subscriptions(flc: &Flc) -> Result<Vec<Subscription>, SecureMemoryError> {
    unsafe {
        let subscriptions = flc.read_t::<[Subscription; SUBSCRIPTIONS_CAPACITY]>(subscriptions_address);
        if subscriptions.is_err() { return Err(SecureMemoryError::FlashError(subscriptions.unwrap_err())); }
        Ok(subscriptions.unwrap().to_vec())
    }
}

pub fn overwrite_subscription(flc: &Flc, subscription: Subscription) -> Result<(), SecureMemoryError> {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Slot {
        Empty(usize),
        Existing(usize),
        Full
    }

    if subscription.channel_id == 0 { return Err(SecureMemoryError::InvalidSubscriptionChannel(subscription.channel_id)); }
    if !subscription.valid { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    if subscription.end < subscription.start { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    let mut subscriptions = retrieve_subscriptions(flc)?;
    let mut slot = Slot::Full;
    for (i, sub) in subscriptions.iter().enumerate() {
        let mut this_slot = Slot::Full;
        if !sub.valid { this_slot = Slot::Empty(i); }
        if sub.valid && sub.channel_id == subscription.channel_id { this_slot = Slot::Existing(i); }
        match slot {
            Slot::Full => {
                if this_slot == Slot::Full {
                    slot = this_slot;
                } else {
                    slot = this_slot;
                }
            },
            Slot::Empty(_) => {
                if this_slot == Slot::Full {
                    slot = slot;
                } else {
                    slot = this_slot;
                }
            },
            Slot::Existing(_) => {
                if this_slot == Slot::Full {
                    slot = slot;
                } else {
                    slot = slot;
                }
            }
        }
    }

    match slot {
        Slot::Full => {
            Err(SecureMemoryError::SubscriptionMemoryFull)
        },
        Slot::Empty(i) => {
            subscriptions[i] = subscription;
            let mut subscription_array: [Subscription; SUBSCRIPTIONS_CAPACITY] = [
                Subscription{
                    channel_id: 0,
                    valid: false,
                    end: 0,
                    start: 0
                }; 8
            ];
            subscription_array.copy_from_slice(subscriptions.as_slice());
            unsafe {
                let data: [u32; SUBSCRIPTIONS_CAPACITY * size_of::<Subscription>() / 4] = core::mem::transmute(subscription_array);
                let ret = flc.erase_page(subscriptions_address);
                if ret.is_err() { return Err(SecureMemoryError::FlashError(ret.unwrap_err())); }
                match flc.write_u32_slice(subscriptions_address, &data) {
                    Ok(()) => Ok(()),
                    Err(flash_error) => Err(SecureMemoryError::FlashError(flash_error))
                }
            }
        },
        Slot::Existing(i) => {
            subscriptions[i] = subscription;
            let mut subscription_array: [Subscription; SUBSCRIPTIONS_CAPACITY] = [
                Subscription{
                    channel_id: 0,
                    valid: false,
                    end: 0,
                    start: 0
                }; 8
            ];
            subscription_array.copy_from_slice(subscriptions.as_slice());
            unsafe {
                let data: [u32; SUBSCRIPTIONS_CAPACITY * size_of::<Subscription>() / 4] = core::mem::transmute(subscription_array);
                let ret = flc.erase_page(subscriptions_address);
                if ret.is_err() { return Err(SecureMemoryError::FlashError(ret.unwrap_err())); }
                match flc.write_u32_slice(subscriptions_address, &data) {
                    Ok(()) => Ok(()),
                    Err(flash_error) => Err(SecureMemoryError::FlashError(flash_error))
                }
            }
        }
    }
}

pub fn retrieve_channel_secret(flc: &Flc, channel_id: u32) -> Result<Option<Secret>, SecureMemoryError> {
    let mut secret: Option<Secret> = None;
    for i in 0..SECRETS_CAPACITY {
        unsafe {
            let sec = flc.read_t::<Secret>(secrets_address + (i * size_of::<Secret>()) as u32);
            if sec.is_err() { return Err(SecureMemoryError::FlashError(sec.unwrap_err())); }
            let sec = sec.unwrap();
            if sec.valid && sec.secret_type == SecretType::Channel(channel_id) {
                secret = Some(sec)
            }
        }
    }
    Ok(secret)
}

pub fn retrieve_master_secret(flc: &Flc) -> Result<Secret, SecureMemoryError> {
    let mut secret: Option<Secret> = None;
    for i in 0..SECRETS_CAPACITY {
        unsafe {
            let sec = flc.read_t::<Secret>(secrets_address + (i * size_of::<Secret>()) as u32);
            if sec.is_err() { return Err(SecureMemoryError::FlashError(sec.unwrap_err())); }
            let sec = sec.unwrap();
            if sec.valid && sec.secret_type == SecretType::Master {
                secret = Some(sec)
            }
        }
    }
    match secret {
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