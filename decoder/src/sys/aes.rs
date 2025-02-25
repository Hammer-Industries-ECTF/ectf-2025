//! AES Support for MAX78000

pub extern crate max7800x_hal as hal;
use hal::pac;

/// AES struct for handling encryption and decryption using the AES hardware module.
pub struct AesBackend<const KEY_SIZE: usize> {
    aes: pac::Aes, // Shared mutable access
}
