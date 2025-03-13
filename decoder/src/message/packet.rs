use hal::aes::AesBlock;

#[derive(Debug, Clone, Copy)]
pub enum PacketError {
    ZeroPaddingNotIntact(usize)
}

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
    todo!("make const time");
    decoded_block == *b"HammerIndustries"
}
