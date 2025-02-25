//! Decrypter

extern crate alloc;
use alloc::vec::Vec;

use crate::message::packet::extract_decoder_id;
use crate::message::packet::PacketError;

use super::secure_memory::Secret;


pub fn decrypt_message(message: Vec<u128>) -> Vec<u128> {
    // Retrieve master secret
    // Interact with AES
    todo!();
}

pub fn decrypt_decoder_id(channel_id: u32, block: u128) -> Result<u32, PacketError> {
    // Retrieve channel secret
    // Interact with AES
    todo!();
    let decoded_block: u128 = 0;
    extract_decoder_id(decoded_block)
}

pub fn decrypt_company_stamp(channel_id: u32, block: u128) -> u128 {
    // Retrieve channel secret
    // Interact with AES
    todo!();
}

pub fn decrypt_frame(channel_id: u32, blocks: Vec<u128>) -> Vec<u8> {
    // Retrieve channel secret
    // Interact with AES
    // Chop off first block (company stamp)
    // Vec<u128> as Vec<u8>
    todo!();
}

fn decrypt_blocks(secret: &Secret, blocks: Vec<u128>) -> Vec<u128> {
    todo!();
}

fn decrypt_block(secret: &Secret, block: u128) -> u128 {
    todo!();
}