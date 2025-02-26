extern crate alloc;
use alloc::vec::Vec;

use hal::aes::{AesKey, AesBlock};

#[derive(Debug, Clone)]
pub enum SecureMemoryError {
    InvalidSubscriptionChannel(u32),
    SubscriptionNotValid(u32)
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub channel_id: u32,
    pub valid: bool,
    pub end: u64,
    pub start: u64
}

#[derive(Debug, Clone)]
pub struct Secret {
    pub channel_id: u32,
    pub aes_key: AesKey,
    pub aes_iv: AesBlock
}

pub fn retrieve_subscription(channel_id: u32) -> Option<Subscription> {
    todo!();
}

pub fn retrieve_subscriptions() -> Vec<Subscription> {
    todo!();
}

pub fn retrieve_channel_secret(channel_id: u32) -> Option<Secret> {
    todo!();
}

pub fn retrieve_master_secret() -> Secret {
    todo!();
}

pub fn overwrite_subscription(subscription: Subscription) -> Result<(), SecureMemoryError> {
    if retrieve_subscription(subscription.channel_id).is_none() { return Err(SecureMemoryError::InvalidSubscriptionChannel(subscription.channel_id)); }
    if !subscription.valid { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    if subscription.end < subscription.start { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    todo!();
}

pub fn verify_decoder_id(decoder_id: u32) -> bool {
    todo!();
}