extern crate alloc;
use alloc::vec::Vec;

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
    pub aes_key: [u128; 2],
    pub aes_iv: u128
}

pub fn retrieve_subscription(channel_id: u32) -> Option<Subscription> {
    unimplemented!();
}

pub fn retrieve_subscriptions() -> Vec<Subscription> {
    unimplemented!();
}

pub fn retrieve_secret(channel_id: u32) -> Option<Secret> {
    unimplemented!();
}
