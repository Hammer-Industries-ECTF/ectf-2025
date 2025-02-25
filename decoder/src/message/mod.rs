//! Message module
//! Contains:
//! - Information about message structure, encrypted and decrypted
//! - Functions for transmitting and receiving data

pub mod receive;
pub mod transmit;
pub mod packet;

extern crate alloc;
use alloc::vec::Vec;

use crate::sys::secure_memory::Subscription;

#[derive(Debug, Clone)]
pub struct HostUpdateMessage {
    pub channel_id: u32, 
    pub end: u64,
    pub start: u64, 
    pub encrypted_decoder_id: u128, 
}

#[derive(Debug, Clone)]
pub struct HostDecodeMessage {
    pub timestamp: u64, 
    pub channel_id: u32, 
    pub frame_length: u32, 
    pub encrypted_frame: Vec<u128>,
}

#[derive(Debug, Clone)]
pub enum HostMessage {
    Debug,
    List,
    Update (HostUpdateMessage),
    Decode (HostDecodeMessage), 
}

#[derive(Debug, Clone)]
pub struct ResponseDebugMessage {}

#[derive(Debug, Clone)]
pub struct ResponseListMessage {
    pub subscriptions: Vec<Subscription>
}

#[derive(Debug, Clone)]
pub struct ResponseDecodeMessage {
    pub frame: Vec<u8>
}

#[derive(Debug, Clone)]
pub enum ResponseMessage {
    Debug  (ResponseDebugMessage),
    List   (ResponseListMessage),
    Update (()),
    Decode (ResponseDecodeMessage), 
}
