//! Message module
//! Contains:
//! - Information about message structure, encrypted and decrypted
//! - Functions for transmitting and receiving data

pub mod receive;
pub mod transmit;

extern crate alloc;
use alloc::vec::Vec;

use crate::sys::Subscription;

#[derive(Debug, Clone)]
pub struct HostDebugMessage {}

#[derive(Debug, Clone)]
pub struct HostListMessage {}

#[derive(Debug, Clone)]
pub struct HostUpdateMessage {
    channel_id: u32, 
    end: u64,
    start: u64, 
    encrypted_decoder_id: u128, 
}

#[derive(Debug, Clone)]
pub struct HostDecodeMessage {
    timestamp: u64, 
    channel_id: u32, 
    frame_length: u32, 
    encrypted_frame: Vec<u128>,
}

#[derive(Debug, Clone)]
pub enum HostMessage {
    Debug  (HostDebugMessage),
    List   (HostListMessage),
    Update (HostUpdateMessage),
    Decode (HostDecodeMessage), 
}

#[derive(Debug, Clone)]
pub struct ResponseDebugMessage {}

#[derive(Debug, Clone)]
pub struct ResponseListMessage {
    subscriptions: Vec<Subscription>
}

#[derive(Debug, Clone)]
pub struct ResponseUpdateMessage {}

#[derive(Debug, Clone)]
pub struct ResponseDecodeMessage {
    frame: Vec<u8>
}

#[derive(Debug, Clone)]
pub enum ResponseMessage {
    Debug  (ResponseDebugMessage),
    List   (ResponseListMessage),
    Update (ResponseUpdateMessage),
    Decode (ResponseDecodeMessage), 
}
