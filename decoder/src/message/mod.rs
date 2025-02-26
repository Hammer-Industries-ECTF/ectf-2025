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

#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub magic: u8,
    pub opcode: u8,
    pub length: u16
}

const MAGIC_BYTE: u8 = 37;

const DEBUG_OPCODE: u8 = 71;
const LIST_OPCODE: u8 = 76;
const UPDATE_OPCODE: u8 = 83;
const DECODE_OPCODE: u8 = 68;
const ACK_OPCODE: u8 = 65;
const ERR_OPCODE: u8 = 69;
