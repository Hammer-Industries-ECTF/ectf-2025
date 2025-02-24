//! Keys
pub extern crate max7800x_hal as hal;

const KEY_SIZE: usize = 0x20;

#[link_section = ".aes_keys"]
#[repr(align(4096))]
#[no_mangle]
static FLASH_DATA: [[u8; 32]; 9] = *[
    b"MESSAGE_KEY",
    b"CHANNEL0_KEY",
    b"CHANNEL1_KEY",
    b"CHANNEL2_KEY",
    b"CHANNEL3_KEY",
    b"CHANNEL4_KEY",
    b"CHANNEL5_KEY",
    b"CHANNEL6_KEY",
    b"CHANNEL7_KEY"
];

pub enum KeyType {
    Message,
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7
}

macro_rules! generate_key_handler {
    ($name:ident, $id:expr) => {
        paste! {
            pub fn [<aes_key_set_ $name:lower>]() {
                !heprintln("TODO: Implement key set");
            }
        } 
    }
}

generate_key_handler!(MESSAGE_KEY, 0);
generate_key_handler!(CHANNEL0_KEY, 1);
generate_key_handler!(CHANNEL1_KEY, 2);
generate_key_handler!(CHANNEL2_KEY, 3);
generate_key_handler!(CHANNEL3_KEY, 4);
generate_key_handler!(CHANNEL4_KEY, 5);
generate_key_handler!(CHANNEL5_KEY, 6);
generate_key_handler!(CHANNEL6_KEY, 7);
generate_key_handler!(CHANNEL7_KEY, 8);
