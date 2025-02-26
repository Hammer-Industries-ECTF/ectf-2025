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

#[derive(Debug, Clone)]
pub enum DecryptError {
    InvalidSecretChannel(u32),
    PacketError(PacketError),
    AesError(AesError)
}

pub fn decrypt_message(aes: &Aes, message: Vec<AesBlock>) -> Result<Vec<AesBlock>, DecryptError> {
    let secret = retrieve_master_secret();
    decrypt_blocks(&aes, &secret, message)
}

pub fn decrypt_decoder_id(aes: &Aes, channel_id: u32, block: AesBlock) -> Result<u32, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    let decoded_block = decrypt_block(&aes, &secret, block)?;
    let decoder_id = extract_decoder_id(
(decoded_block[0] as u128) |
            ((decoded_block[1] as u128) << 32) |
            ((decoded_block[2] as u128) << 64) |
            ((decoded_block[3] as u128) << 96)
    );
    if decoder_id.is_err() { return Err(DecryptError::PacketError(decoder_id.unwrap_err())); }
    Ok(decoder_id.unwrap())
}

pub fn decrypt_company_stamp(aes: &Aes, channel_id: u32, block: AesBlock) -> Result<u128, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    match decrypt_block(&aes, &secret, block) {
        Ok(block) => { Ok(
            (block[0] as u128) |
            ((block[1] as u128) << 32) |
            ((block[2] as u128) << 64) |
            ((block[3] as u128) << 96)
        ) },
        Err(err) => { Err(err) }
    }
}

pub fn decrypt_frame(aes: &Aes, channel_id: u32, blocks: Vec<AesBlock>) -> Result<Vec<u8>, DecryptError> {
    let secret = retrieve_channel_secret(channel_id);
    if secret.is_none() { return Err(DecryptError::InvalidSecretChannel(channel_id)); }
    let secret = secret.unwrap();
    let mut decrypted_blocks = decrypt_blocks(&aes, &secret, blocks)?;
    decrypted_blocks.remove(0);
    let decrypted_blocks: Vec<u8> = decrypted_blocks.iter().flat_map(|x| x.iter().flat_map(|y| y.to_le_bytes())).collect();
    Ok(decrypted_blocks)
}

fn decrypt_blocks(aes: &Aes, secret: &Secret, blocks: Vec<AesBlock>) -> Result<Vec<AesBlock>, DecryptError> {
    let mut decrypted_blocks: Vec<AesBlock> = Vec::with_capacity(blocks.len());
    aes.set_key(&secret.aes_key);
    let mut cbc= secret.aes_iv;
    for block in blocks {
        let cbc_intermediate: Vec<u32> = zip(block, cbc).map(|(x, y)| x ^ y).collect();
        let mut aes_in: AesBlock = [0; 4];
        aes_in.copy_from_slice(cbc_intermediate.as_slice());
        let aes_out = aes.decrypt_block(aes_in);
        if aes_out.is_err() { return Err(DecryptError::AesError(aes_out.unwrap_err())); }
        let aes_out = aes_out.unwrap();
        decrypted_blocks.push(aes_out);
        cbc = aes_out;
    }
    Ok(decrypted_blocks)
}

fn decrypt_block(aes: &Aes, secret: &Secret, block: AesBlock) -> Result<AesBlock, DecryptError> {
    aes.set_key(&secret.aes_key);
    let cbc_intermediate: Vec<u32> = zip(block, secret.aes_iv).map(|(x, y)| x ^ y).collect();
    let mut aes_in: AesBlock = [0; 4];
    aes_in.copy_from_slice(cbc_intermediate.as_slice());
    match aes.decrypt_block(aes_in) {
        Ok(block) => Ok(block),
        Err(err) => Err(DecryptError::AesError(err))
    }
}