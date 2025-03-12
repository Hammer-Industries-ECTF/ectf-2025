//! Decrypter

use core::iter::zip;

extern crate alloc;
use alloc::vec::Vec;

use hal::aes::Aes;

use hal::aes::{AesBlock, AesError};

use crate::message::packet::PacketError;
use crate::message::packet::extract_decoder_id;

use super::secure_memory::Secret;
use super::secure_memory::{retrieve_master_secret, retrieve_channel_secret};

#[derive(Debug, Clone, Copy)]
pub enum DecryptError {
    InvalidSecretChannel(u32),
    PacketError(PacketError),
    AesError(AesError)
}

pub fn decrypt_message(aes: &Aes, message: Vec<AesBlock>) -> Result<Vec<AesBlock>, DecryptError> {
    let secret = retrieve_master_secret();
    decrypt_blocks(aes, secret, message)
}

pub fn decrypt_decoder_id(aes: &Aes, channel_id: u32, block: AesBlock) -> Result<u32, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    let decoded_block = decrypt_block(aes, secret, block)?;
    let decoder_id = extract_decoder_id(decoded_block);
    if decoder_id.is_err() { return Err(DecryptError::PacketError(decoder_id.unwrap_err())); }
    Ok(decoder_id.unwrap())
}

pub fn decrypt_company_stamp(aes: &Aes, channel_id: u32, block: AesBlock) -> Result<AesBlock, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    decrypt_block(aes, secret, block)
}

pub fn decrypt_frame(aes: &Aes, channel_id: u32, blocks: Vec<AesBlock>) -> Result<Vec<u8>, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    let mut decrypted_blocks = decrypt_blocks(aes, secret, blocks)?;
    decrypted_blocks.remove(0);
    let decrypted_blocks: Vec<u8> = decrypted_blocks.iter().flat_map(|x| *x).collect();
    Ok(decrypted_blocks)
}

fn decrypt_blocks(aes: &Aes, secret: &Secret, blocks: Vec<AesBlock>) -> Result<Vec<AesBlock>, DecryptError> {
    let mut key = secret.aes_key;
    key.reverse();
    aes.set_key(&key);
    let mut decrypted_blocks: Vec<AesBlock> = Vec::with_capacity(blocks.len());
    let mut cbc= secret.aes_iv;
    for mut block in blocks {
        block.reverse();
        let aes_out = aes.decrypt_block(block);
        if aes_out.is_err() { return Err(DecryptError::AesError(aes_out.unwrap_err())); }
        let mut aes_out = aes_out.unwrap();
        aes_out.reverse();
        let cbc_intermediate: Vec<u8> = zip(aes_out, cbc).map(|(x, y)| x ^ y).collect();
        let mut aes_out: AesBlock = [0; 16];
        aes_out.copy_from_slice(cbc_intermediate.as_slice());
        decrypted_blocks.push(aes_out);
        block.reverse();
        cbc = block;
    }
    Ok(decrypted_blocks)
}

fn decrypt_block(aes: &Aes, secret: &Secret, block: AesBlock) -> Result<AesBlock, DecryptError> {
    let mut key = secret.aes_key;
    key.reverse();
    aes.set_key(&key);
    let mut block = block;
    block.reverse();
    let aes_out = aes.decrypt_block(block);
    if aes_out.is_err() { return Err(DecryptError::AesError(aes_out.unwrap_err())); }
    let mut aes_out = aes_out.unwrap();
    aes_out.reverse();
    let cbc_intermediate: Vec<u8> = zip(aes_out, secret.aes_iv).map(|(x, y)| x ^ y).collect();
    let mut aes_out: AesBlock = [0; 16];
    aes_out.copy_from_slice(cbc_intermediate.as_slice());
    Ok(aes_out)
}