#[derive(Debug, Clone)]
pub enum PacketError {
    ZeroPaddingNotIntact(usize)
}

pub fn extract_channel_number(decoded_block: u128) -> Result<u32, PacketError> {
    if decoded_block > 2u128.pow(32) - 1 { return Err(PacketError::ZeroPaddingNotIntact(96)); }
    Ok(decoded_block as u32)
}

pub fn extract_timestamps(decoded_block: u128) -> (u64, u64) {
    (decoded_block as u64, (decoded_block >> 64) as u64)
}

pub fn extract_decoder_id(decoded_block: u128) -> Result<u32, PacketError> {
    if decoded_block > 2u128.pow(32) - 1 { return Err(PacketError::ZeroPaddingNotIntact(96)); }
    Ok(decoded_block as u32)
}

pub fn extract_frame_metadata(decoded_block: u128) -> (u64, u32, u32) {
    (decoded_block as u64, (decoded_block >> 64) as u32, (decoded_block >> 96) as u32)
}

pub fn verify_company_stamp(decoded_block: u128) -> bool {
    decoded_block != u128::from_le_bytes(*b"HammerIndustries")
}
