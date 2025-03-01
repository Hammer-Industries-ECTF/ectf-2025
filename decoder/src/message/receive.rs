//! Reciever functions

extern crate alloc;
use alloc::vec::Vec;

use hal::aes::{Aes, AesBlock};

use hal::{gpio::{Af1, Pin}, pac::Uart0, uart::BuiltUartPeripheral};

use crate::sys::decrypt::DecryptError;
use crate::sys::decrypt::decrypt_message;

use super::{HostDecodeMessage, HostMessage, HostUpdateMessage, MessageHeader};
use super::{MAGIC_BYTE, LIST_OPCODE, UPDATE_OPCODE, DECODE_OPCODE, DEBUG_OPCODE, ACK_OPCODE, ERR_OPCODE};

use super::packet::PacketError;
use super::packet::{extract_channel_id, extract_timestamps, extract_frame_metadata};

use super::transmit::transmit_ack;

#[derive(Debug, Clone)]
pub enum RXError {
    IncorrectMagic(u8),
    InvalidOpcode(u8),
    InvalidLength(u16),
    UnexpectedDebug,
    UnexpectedACK,
    UnexpectedERR,
    PacketError(PacketError),
    DecryptError(DecryptError)
}

pub fn receive_message(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, aes: &Aes) -> Result<HostMessage, RXError> {
    let message_header = receive_header(uart);
    if message_header.magic != MAGIC_BYTE { return Err(RXError::IncorrectMagic(message_header.magic)); }
    match message_header.opcode {
        LIST_OPCODE => {
            if message_header.length != 0 { return Err(RXError::InvalidLength(message_header.length)); }
            transmit_ack(uart);
            Ok(HostMessage::List)
        },
        UPDATE_OPCODE => { Ok(HostMessage::Update(receive_update_body(uart, aes, message_header)?)) },
        DECODE_OPCODE => { Ok(HostMessage::Decode(receive_decode_body(uart, aes, message_header)?)) },
        DEBUG_OPCODE => { Err(RXError::UnexpectedDebug) },
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

fn receive_update_body(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, aes: &Aes, header: MessageHeader) -> Result<HostUpdateMessage, RXError> {
    if header.length != 48 { return Err(RXError::InvalidLength(header.length)); }
    let mut body_buf: [u8; 48] = [0; 48];
    transmit_ack(uart);
    uart.read_bytes(&mut body_buf);
    let mut encrypted_blocks: Vec<AesBlock> = Vec::with_capacity(12);
    encrypted_blocks.push([
        u32::from_le_bytes(*body_buf[0..4].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[4..8].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[8..12].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[12..16].first_chunk::<4>().unwrap())
    ]);
    encrypted_blocks.push([
        u32::from_le_bytes(*body_buf[16..20].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[20..24].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[24..28].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[28..32].first_chunk::<4>().unwrap())
    ]);
    encrypted_blocks.push([
        u32::from_le_bytes(*body_buf[32..36].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[36..40].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[40..44].first_chunk::<4>().unwrap()),
        u32::from_le_bytes(*body_buf[44..48].first_chunk::<4>().unwrap())
    ]);
    let decrypted_blocks = decrypt_message(aes, encrypted_blocks);
    if decrypted_blocks.is_err() { return Err(RXError::DecryptError(decrypted_blocks.unwrap_err())); }
    let decrypted_blocks = decrypted_blocks.unwrap();
    let channel_id = extract_channel_id(
        (decrypted_blocks[0][0] as u128) |
            ((decrypted_blocks[0][1] as u128) << 32) |
            ((decrypted_blocks[0][2] as u128) << 64) |
            ((decrypted_blocks[0][3] as u128) << 96)
    );
    if channel_id.is_err() { return Err(RXError::PacketError(channel_id.unwrap_err())); }
    let channel_id = channel_id.unwrap();
    let (end, start) = extract_timestamps(
        (decrypted_blocks[1][0] as u128) |
            ((decrypted_blocks[1][1] as u128) << 32) |
            ((decrypted_blocks[1][2] as u128) << 64) |
            ((decrypted_blocks[1][3] as u128) << 96)
    );
    transmit_ack(uart);
    Ok(HostUpdateMessage{ channel_id, end, start, encrypted_decoder_id: decrypted_blocks[2] })
}

fn receive_decode_body(uart: &BuiltUartPeripheral<Uart0, Pin<0, 0, Af1>, Pin<0, 1, Af1>, (), ()>, aes: &Aes, header: MessageHeader) -> Result<HostDecodeMessage, RXError> {
    let mut encrypted_blocks: Vec<AesBlock> = Vec::with_capacity(24);
    match header.length {
        48 => {
            let mut body_buf: [u8; 48] = [0; 48];
            transmit_ack(uart);
            uart.read_bytes(&mut body_buf);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[0..4].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[4..8].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[8..12].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[12..16].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[16..20].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[20..24].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[24..28].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[28..32].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[32..36].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[36..40].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[40..44].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[44..48].first_chunk::<4>().unwrap())
            ]);
        },
        64 => {
            let mut body_buf: [u8; 64] = [0; 64];
            transmit_ack(uart);
            uart.read_bytes(&mut body_buf);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[0..4].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[4..8].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[8..12].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[12..16].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[16..20].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[20..24].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[24..28].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[28..32].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[32..36].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[36..40].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[40..44].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[44..48].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[48..52].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[52..56].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[56..60].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[60..64].first_chunk::<4>().unwrap())
            ]);
        },
        80 => {
            let mut body_buf: [u8; 80] = [0; 80];
            transmit_ack(uart);
            uart.read_bytes(&mut body_buf);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[0..4].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[4..8].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[8..12].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[12..16].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[16..20].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[20..24].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[24..28].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[28..32].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[32..36].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[36..40].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[40..44].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[44..48].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[48..52].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[52..56].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[56..60].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[60..64].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[64..68].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[68..72].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[72..76].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[76..80].first_chunk::<4>().unwrap())
            ]);
        },
        96 => {
            let mut body_buf: [u8; 96] = [0; 96];
            transmit_ack(uart);
            uart.read_bytes(&mut body_buf);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[0..4].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[4..8].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[8..12].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[12..16].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[16..20].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[20..24].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[24..28].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[28..32].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[32..36].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[36..40].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[40..44].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[44..48].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[48..52].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[52..56].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[56..60].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[60..64].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[64..68].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[68..72].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[72..76].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[76..80].first_chunk::<4>().unwrap())
            ]);
            encrypted_blocks.push([
                u32::from_le_bytes(*body_buf[80..84].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[84..88].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[88..92].first_chunk::<4>().unwrap()),
                u32::from_le_bytes(*body_buf[92..96].first_chunk::<4>().unwrap())
            ]);
        },
        other => { return Err(RXError::InvalidLength(other)); }
    }
    let decrypted_blocks = decrypt_message(aes, encrypted_blocks);
    if decrypted_blocks.is_err() { return Err(RXError::DecryptError(decrypted_blocks.unwrap_err())); }
    let mut decrypted_blocks = decrypted_blocks.unwrap();
    let (timestamp, channel_id, frame_length) = extract_frame_metadata(
        (decrypted_blocks[0][0] as u128) |
            ((decrypted_blocks[0][1] as u128) << 32) |
            ((decrypted_blocks[0][2] as u128) << 64) |
            ((decrypted_blocks[0][3] as u128) << 96)
    );
    decrypted_blocks.remove(0);
    transmit_ack(uart);
    Ok(HostDecodeMessage{ timestamp, channel_id, frame_length, encrypted_frame: decrypted_blocks })
}