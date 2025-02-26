//! Reciever functions

extern crate alloc;
use alloc::vec::Vec;

use hal::{gpio::{Af1, Pin}, pac::Uart0, uart::BuiltUartPeripheral};

use crate::sys::decrypt::decrypt_message;

use super::{HostDecodeMessage, HostMessage, HostUpdateMessage, MessageHeader};
use super::{MAGIC_BYTE, DEBUG_OPCODE, LIST_OPCODE, UPDATE_OPCODE, DECODE_OPCODE, ACK_OPCODE, ERR_OPCODE};

use super::packet::PacketError;
use super::packet::{extract_channel_id, extract_timestamps, extract_frame_metadata};

use super::transmit::transmit_ack;

#[derive(Debug, Clone)]
pub enum RXError {
    IncorrectMagic(u8),
    InvalidOpcode(u8),
    InvalidLength(u16),
    UnexpectedACK,
    UnexpectedERR,
    PacketError(PacketError)
}

pub fn receive_message(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>) -> Result<HostMessage, RXError> {
    let message_header = receive_header(&uart);
    if message_header.magic != MAGIC_BYTE { return Err(RXError::IncorrectMagic(message_header.magic)); }
    match message_header.opcode {
        DEBUG_OPCODE => {
            if message_header.length != 0 { return Err(RXError::InvalidLength(message_header.length)); }
            transmit_ack(&uart);
            Ok(HostMessage::Debug)
        },
        LIST_OPCODE => {
            if message_header.length != 0 { return Err(RXError::InvalidLength(message_header.length)); }
            transmit_ack(&uart);
            Ok(HostMessage::List)
        },
        UPDATE_OPCODE => { Ok(HostMessage::Update(receive_update_body(&uart, message_header)?)) },
        DECODE_OPCODE => { Ok(HostMessage::Decode(receive_decode_body(&uart, message_header)?)) },
        ACK_OPCODE => { Err(RXError::UnexpectedACK) },
        ERR_OPCODE => { Err(RXError::UnexpectedERR) },
        other => { Err(RXError::InvalidOpcode(other)) }
    }
}

pub fn receive_ack(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>) -> Result<(), RXError> {
    let mut header_buf: [u8; 4] = [0; 4];
    let mut length_buf: [u8; 2] = [0; 2];
    uart.read_bytes(&mut header_buf);
    length_buf.clone_from_slice(&header_buf[2..=3]);
    let length = u16::from_le_bytes(length_buf);
    if header_buf[0] != MAGIC_BYTE { return Err(RXError::IncorrectMagic(header_buf[0])); }
    if header_buf[1] != ACK_OPCODE { return Err(RXError::InvalidOpcode(header_buf[1])); }
    if length != 0 { return Err(RXError::InvalidLength(length)); }
    Ok(())
}

fn receive_header(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>) -> MessageHeader {
    let mut header_buf: [u8; 4] = [0; 4];
    let mut length_buf: [u8; 2] = [0; 2];
    uart.read_bytes(&mut header_buf);
    length_buf.clone_from_slice(&header_buf[2..=3]);
    MessageHeader{ magic: header_buf[0], opcode: header_buf[1], length: u16::from_le_bytes(length_buf) }
}

fn receive_update_body(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, header: MessageHeader) -> Result<HostUpdateMessage, RXError> {
    if header.length != 48 { return Err(RXError::InvalidLength(header.length)); }
    let mut body_buf: [u8; 48] = [0; 48];
    transmit_ack(&uart);
    uart.read_bytes(&mut body_buf);
    let mut encrypted_blocks: Vec<u128> = Vec::with_capacity(3);
    encrypted_blocks.push(u128::from_le_bytes(*body_buf[0..16].first_chunk::<16>().unwrap()));
    encrypted_blocks.push(u128::from_le_bytes(*body_buf[16..32].first_chunk::<16>().unwrap()));
    encrypted_blocks.push(u128::from_le_bytes(*body_buf[32..48].first_chunk::<16>().unwrap()));
    let decrypted_blocks = decrypt_message(encrypted_blocks);
    let channel_id = extract_channel_id(decrypted_blocks[0]);
    if channel_id.is_err() { return Err(RXError::PacketError(channel_id.unwrap_err())); }
    let channel_id = channel_id.unwrap();
    let (end, start) = extract_timestamps(decrypted_blocks[1]);
    Ok(HostUpdateMessage{ channel_id, end, start, encrypted_decoder_id: decrypted_blocks[2] })
}

fn receive_decode_body(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, header: MessageHeader) -> Result<HostDecodeMessage, RXError> {
    let mut encrypted_blocks: Vec<u128> = Vec::with_capacity(6);
    match header.length {
        48 => {
            let mut body_buf: [u8; 48] = [0; 48];
            transmit_ack(&uart);
            uart.read_bytes(&mut body_buf); 
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[0..16].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[16..32].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[32..48].first_chunk::<16>().unwrap()));
        },
        64 => {
            let mut body_buf: [u8; 64] = [0; 64];
            transmit_ack(&uart);
            uart.read_bytes(&mut body_buf); 
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[0..16].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[16..32].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[32..48].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[48..64].first_chunk::<16>().unwrap()));
        },
        80 => {
            let mut body_buf: [u8; 80] = [0; 80];
            transmit_ack(&uart);
            uart.read_bytes(&mut body_buf); 
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[0..16].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[16..32].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[32..48].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[48..64].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[64..80].first_chunk::<16>().unwrap()));
        },
        96 => {
            let mut body_buf: [u8; 96] = [0; 96];
            transmit_ack(&uart);
            uart.read_bytes(&mut body_buf); 
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[0..16].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[16..32].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[32..48].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[48..64].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[64..80].first_chunk::<16>().unwrap()));
            encrypted_blocks.push(u128::from_le_bytes(*body_buf[80..96].first_chunk::<16>().unwrap()));
        },
        other => { return Err(RXError::InvalidLength(other)); }
    }
    let mut decrypted_blocks = decrypt_message(encrypted_blocks);
    let (timestamp, channel_id, frame_length) = extract_frame_metadata(decrypted_blocks[0]);
    decrypted_blocks.remove(0);
    Ok(HostDecodeMessage{ timestamp, channel_id, frame_length, encrypted_frame: decrypted_blocks })
}