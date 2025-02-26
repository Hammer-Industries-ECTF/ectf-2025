//! Keys
pub extern crate max7800x_hal as hal;

use super::secure_memory::{Secret, Subscription};

const SECRET_SIZE: usize = size_of::<Secret>();

#[link_section = ".secrets"]
#[no_mangle]
static SECRET_FLASH_DATA: [&[u8; SECRET_SIZE]; 10] = [
    b"MASTER_SECRET_FILLER_DATA______MASTER_SECRET_FILLER_DATA",
    b"CHANNEL0_SECRET_FILLER_DATA__CHANNEL0_SECRET_FILLER_DATA",
    b"CHANNEL1_SECRET_FILLER_DATA__CHANNEL1_SECRET_FILLER_DATA",
    b"CHANNEL2_SECRET_FILLER_DATA__CHANNEL2_SECRET_FILLER_DATA",
    b"CHANNEL3_SECRET_FILLER_DATA__CHANNEL3_SECRET_FILLER_DATA",
    b"CHANNEL4_SECRET_FILLER_DATA__CHANNEL4_SECRET_FILLER_DATA",
    b"CHANNEL5_SECRET_FILLER_DATA__CHANNEL5_SECRET_FILLER_DATA",
    b"CHANNEL6_SECRET_FILLER_DATA__CHANNEL6_SECRET_FILLER_DATA",
    b"CHANNEL7_SECRET_FILLER_DATA__CHANNEL7_SECRET_FILLER_DATA",
    b"CHANNEL8_SECRET_FILLER_DATA__CHANNEL8_SECRET_FILLER_DATA"
    // CHANNELN SECRET SPACE
];

pub enum SecretSlot {
    Master,
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8
}

macro_rules! generate_secret_handler {
    ($name:ident, $id:expr) => {
        paste! {
            pub fn [<secret_set_ $name:lower>]() {
                !heprintln("TODO: Implement secret set");
            }
        } 
    }
}

generate_secret_handler!(MASTER_SECRET, 0);
generate_secret_handler!(CHANNEL0_SECRET, 1);
generate_secret_handler!(CHANNEL1_SECRET, 2);
generate_secret_handler!(CHANNEL2_SECRET, 3);
generate_secret_handler!(CHANNEL3_SECRET, 4);
generate_secret_handler!(CHANNEL4_SECRET, 5);
generate_secret_handler!(CHANNEL5_SECRET, 6);
generate_secret_handler!(CHANNEL6_SECRET, 7);
generate_secret_handler!(CHANNEL7_SECRET, 8);
generate_secret_handler!(CHANNEL8_SECRET, 8);

const SUBSCRIPTION_SIZE: usize = size_of::<Subscription>();

#[link_section = ".subscriptions"]
#[no_mangle]
static SUBSCRIPTION_FLASH_DATA: [&[u8; SUBSCRIPTION_SIZE]; 8] = [
    b"CHANNEL1_SUBSCRIPTION___",
    b"CHANNEL2_SUBSCRIPTION___",
    b"CHANNEL3_SUBSCRIPTION___",
    b"CHANNEL4_SUBSCRIPTION___",
    b"CHANNEL5_SUBSCRIPTION___",
    b"CHANNEL6_SUBSCRIPTION___",
    b"CHANNEL7_SUBSCRIPTION___",
    b"CHANNEL8_SUBSCRIPTION___"
];

pub enum SubscriptionSlot {
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8
}

macro_rules! generate_subscription_handler {
    ($name:ident, $id:expr) => {
        paste! {
            pub fn [<subscription_set_ $name:lower>]() {
                todo!();
                !heprintln("TODO: Implement subscription set");
            }
        } 
    }
}

generate_subscription_handler!(CHANNEL1_SUBSCRIPTION, 2);
generate_subscription_handler!(CHANNEL2_SUBSCRIPTION, 3);
generate_subscription_handler!(CHANNEL3_SUBSCRIPTION, 4);
generate_subscription_handler!(CHANNEL4_SUBSCRIPTION, 5);
generate_subscription_handler!(CHANNEL5_SUBSCRIPTION, 6);
generate_subscription_handler!(CHANNEL6_SUBSCRIPTION, 7);
generate_subscription_handler!(CHANNEL7_SUBSCRIPTION, 8);
generate_subscription_handler!(CHANNEL8_SUBSCRIPTION, 8);

const DECODER_ID_SIZE: usize = size_of::<u32>();

#[link_section = ".decoder_id"]
#[no_mangle]
static DECODER_ID_FLASH_DATA: &[u8; DECODER_ID_SIZE] = b"D_ID";
