use core::iter::zip;

use hal::aes::AesBlock;

#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum PacketError {
    ZeroPaddingNotIntact(usize)
}

static COMPANY_STAMP: AesBlock = *b"HammerIndustries";

pub fn extract_channel_id(decoded_block: AesBlock) -> Result<u32, PacketError> {
    if !decoded_block[4..].iter().all(|x| *x == 0) {
        Err(PacketError::ZeroPaddingNotIntact(96))
    } else {
        Ok(u32::from_le_bytes(*decoded_block.first_chunk::<4>().unwrap()))
    }
}

pub fn extract_timestamps(decoded_block: AesBlock) -> (u64, u64) {
        (u64::from_le_bytes(*decoded_block.first_chunk::<8>().unwrap()),
        u64::from_le_bytes(*decoded_block.last_chunk::<8>().unwrap()))
}

pub fn extract_decoder_id(decoded_block: AesBlock) -> Result<u32, PacketError> {
    if !decoded_block[4..].iter().all(|x| *x == 0) {
        Err(PacketError::ZeroPaddingNotIntact(96))
    } else {
        Ok(u32::from_le_bytes(*decoded_block.first_chunk::<4>().unwrap()))
    }
}

pub fn extract_frame_metadata(decoded_block: AesBlock) -> (u64, u32, u32) {
    (u64::from_le_bytes(*decoded_block.first_chunk::<8>().unwrap()),
    u32::from_le_bytes(*decoded_block.last_chunk::<8>().unwrap().first_chunk::<4>().unwrap()),
    u32::from_le_bytes(*decoded_block.last_chunk::<4>().unwrap()))
}

pub fn verify_company_stamp(decoded_block: AesBlock) -> bool {
    let mut c: u8 = 0;
    for (block_byte, stamp_byte) in zip(decoded_block, COMPANY_STAMP) {
        c |= block_byte ^ stamp_byte
    }
    c == 0
}
