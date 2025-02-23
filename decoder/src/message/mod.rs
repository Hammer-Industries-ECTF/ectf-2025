//! Message module
//! Contains:
//! - Information about message structure, encrypted and decrypted
//! - Functions for transmitting and receiving data

pub mod receive;
pub mod transmit;

extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum HostRawMessage {}

#[derive(Debug, Clone)]
pub struct HostDebugMessage {}

#[derive(Debug, Clone)]
pub struct HostListMessage {}

#[derive(Debug, Clone)]
pub struct HostUpdateMessage {
    decoder_id: u128, 
    channel_id: u32, 
    start: u64, 
    end: u64,
}

#[derive(Debug, Clone)]
pub struct HostDecodeMessage {
    timestamp: u64, 
    decoder_id: u32, 
    channel_id: u32, 
    body: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum HostMessageType {
    Debug  (HostDebugMessage),
    List   (HostListMessage),
    Update (HostUpdateMessage),
    Decode (HostDecodeMessage), 
}

#[derive(Debug, Clone)]
pub struct HostMessage {
    pub msg_type: HostMessageType,
}

impl Message {
    /// Create a new message
    pub fn new(msg_type: MessageType) -> Self {
        Self { msg_type }
    }
}

#[derive(Debug, Clone)]
pub enum ResponseMessageType {
    // Debug message with a string
    Debug  {},                             
    // List message with a vector of bytes
    List   { subscriptions: Vec<Subscription> },                                         
    // Update with ID and fixed-size data
    Update {}, 
    // Decode message with raw byte data
    Decode { timestamp: u64, body: Vec<u8>}, 
}
