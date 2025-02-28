extern crate alloc;
use alloc::vec::Vec;

use hal::aes::{AesKey, AesBlock};

#[derive(Debug, Clone)]
pub enum SecureMemoryError {
    InvalidSubscriptionChannel(u32),
    SubscriptionNotValid(u32),
    SubscriptionMemoryFull
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub channel_id: u32,
    pub valid: bool,
    pub end: u64,
    pub start: u64
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecretType {
    Channel(u32),
    Master
}

#[derive(Debug, Clone)]
pub struct Secret {
    pub secret_type: SecretType,
    pub aes_key: AesKey,
    pub aes_iv: AesBlock
}

static mut SUBSCRIPTIONS: [Subscription; 4] = [
    Subscription{
        channel_id: 123123,
        valid: false,
        end: 64353244,
        start: 14345
    },
    Subscription{
        channel_id: 65432,
        valid: false,
        end: 1435243,
        start: 64532
    },
    Subscription{
        channel_id: 566543,
        valid: false,
        end: 2365443,
        start: 135246764
    },
    Subscription{
        channel_id: 1536,
        valid: false,
        end: 645312,
        start: 5465642
    }
];

static SECRETS: [Secret; 6] = [
    Secret{
        secret_type: SecretType::Master,
        aes_key: [
            3478232595,
            2941868455,
            3929294904,
            1708710345,
            2748321695,
            928514543,
            3990285456,
            2512103925
        ],
        aes_iv: [
            1480735525,
            1959622316,
            1802974681,
            3940972782
        ]
    },
    Secret{
        secret_type: SecretType::Channel(0),
        aes_key: [
            2273376030,
            1953860943,
            4004508207,
            3872792155,
            3271825017,
            2612002843,
            3286900081,
            491734632
        ],
        aes_iv: [
            462299052,
            2333640759,
            668295373,
            1048872516
        ]
    },
    Secret{
        secret_type: SecretType::Channel(1),
        aes_key: [
            3145218359,
            305800554,
            1166012645,
            83913713,
            4229096117,
            2589109261,
            2079184311,
            3805355829
        ],
        aes_iv: [
            59562964,
            4159113488,
            1001649435,
            1482442691
        ]
    },
    Secret{
        secret_type: SecretType::Channel(2),
        aes_key: [
            2197328252,
            3267128575,
            414311197,
            1009154054,
            1570135109,
            3297591940,
            3894486351,
            2425096737
        ],
        aes_iv: [
            772280606,
            3904485816,
            759211601,
            1820836589
        ]
    },
    Secret{
        secret_type: SecretType::Channel(3),
        aes_key: [
            3647853426,
            3493334782,
            3270760113,
            2554201485,
            2008673157,
            3038985576,
            1321461936,
            1035882248
        ],
        aes_iv: [
            1423678370,
            1813749163,
            3597523908,
            801371321
        ]
    },
    Secret{
        secret_type: SecretType::Channel(4),
        aes_key: [
            1608308680,
            2865776529,
            3278781565,
            3903078428,
            1639522939,
            3735439251,
            3385339406,
            3798504259
        ],
        aes_iv: [
            1040422077,
            1444003685,
            1275726570,
            613036963
        ]
    }
];

pub fn retrieve_subscription(channel_id: u32) -> Option<&'static Subscription> {
    unsafe { SUBSCRIPTIONS.iter().find(|x| x.channel_id == channel_id) }
}

pub fn retrieve_subscriptions() -> Vec<Subscription> {
    unsafe { SUBSCRIPTIONS.to_vec() }
}

pub fn retrieve_channel_secret(channel_id: u32) -> Option<&'static Secret> {
    SECRETS.iter().find(|x| x.secret_type == SecretType::Channel(channel_id))
}

pub fn retrieve_master_secret() -> &'static Secret {
    SECRETS.iter().find(|x| x.secret_type == SecretType::Master).unwrap()
}

pub fn overwrite_subscription(subscription: Subscription) -> Result<(), SecureMemoryError> {
    if subscription.channel_id == 0 { return Err(SecureMemoryError::InvalidSubscriptionChannel(subscription.channel_id)); }
    if !subscription.valid { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    if subscription.end < subscription.start { return Err(SecureMemoryError::SubscriptionNotValid(subscription.channel_id)); }
    unsafe {
        let open_slot = SUBSCRIPTIONS.iter().position(|x| !x.valid);
        if open_slot.is_none() { return Err(SecureMemoryError::SubscriptionMemoryFull); }
        let open_slot = open_slot.unwrap();
        SUBSCRIPTIONS[open_slot] = subscription;
        Ok(())
    }
}

pub fn verify_decoder_id(decoder_id: u32) -> bool {
    decoder_id == 0xDEADBEEF
}