//! Decrypter

extern crate alloc;
use alloc::vec::Vec;

use crate::message::packet::PacketError;
use crate::message::packet::extract_decoder_id;

use super::secure_memory::Secret;
use super::secure_memory::{retrieve_master_secret, retrieve_channel_secret};

#[derive(Debug, Clone)]
pub enum DecryptError {
    InvalidSecretChannel(u32),
    PacketError(PacketError)
}

pub fn decrypt_message(message: Vec<u128>) -> Vec<u128> {
    let secret = retrieve_master_secret();
    decrypt_blocks(&secret, message)
}

pub fn decrypt_decoder_id(channel_id: u32, block: u128) -> Result<u32, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    let decoded_block = decrypt_block(&secret, block);
    let decoder_id = extract_decoder_id(decoded_block);
    if decoder_id.is_err() { return Err(DecryptError::PacketError(decoder_id.unwrap_err())); }
    Ok(decoder_id.unwrap())
}

pub fn decrypt_company_stamp(channel_id: u32, block: u128) -> Result<u128, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    Ok(decrypt_block(&secret, block))
}

pub fn decrypt_frame(channel_id: u32, blocks: Vec<u128>) -> Result<Vec<u8>, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    let mut decrypted_blocks = decrypt_blocks(&secret, blocks);
    decrypted_blocks.remove(0);
    let decrypted_blocks: Vec<u8> = decrypted_blocks.iter().flat_map(|x| x.to_le_bytes()).collect();
    Ok(decrypted_blocks)
}

fn decrypt_blocks(secret: &Secret, blocks: Vec<u128>) -> Vec<u128> {
    todo!();
}

fn decrypt_block(secret: &Secret, block: u128) -> u128 {
    todo!();
}