//! Keys
use super::secure_memory::{Secret, SecretType, Subscription};

#[link_section = ".subscriptions"]
#[no_mangle]
static mut SUBSCRIPTIONS: [Subscription; 8] = [
    Subscription{
        channel_id: 0,
        valid: false,
        end: 0,
        start: 0
    }; 8
];

#[link_section = ".channel_secrets"]
#[no_mangle]
static SECRETS: [Secret; 128] = [
    Secret{
        secret_type: SecretType::Master,
        valid: true,
        aes_key: [
            0x13, 0x9e, 0x51, 0xcf, 0xa7, 0x59, 0x59, 0xaf, 0x38, 0x48, 0x34, 0xea, 0xc9, 0xd9, 0xd8, 0x65, 0x9f, 0x0f, 0xd0, 0xa3, 0xef, 0x01, 0x58, 0x37, 0x90, 0xec, 0xd6, 0xed, 0xf5, 0xa9, 0xbb, 0x95
        ],
        aes_iv: [
            0x25, 0x3b, 0x42, 0x58, 0xac, 0x76, 0xcd, 0x74, 0xd9, 0x35, 0x77, 0x6b, 0xee, 0x78, 0xe6, 0xea
        ]
    },
    Secret{
        secret_type: SecretType::Channel(0),
        valid: true,
        aes_key: [
            0x1e, 0xf7, 0x80, 0x87, 0x4f, 0x8d, 0x75, 0x74, 0x2f, 0xf2, 0xaf, 0xee, 0x5b, 0x1e, 0xd6, 0xe6, 0x79, 0x16, 0x04, 0xc3, 0x1b, 0x00, 0xb0, 0x9b, 0x71, 0x1d, 0xea, 0xc3, 0x68, 0x46, 0x4f, 0x1d
        ],
        aes_iv: [
            0xac, 0x1f, 0x8e, 0x1b, 0x37, 0x88, 0x18, 0x8b, 0xcd, 0x60, 0xd5, 0x27, 0x44, 0x86, 0x84, 0x3e
        ]
    },
    Secret{
        secret_type: SecretType::Channel(1),
        valid: true,
        aes_key: [
            0x37, 0x39, 0x78, 0xbb, 0x6a, 0x25, 0x3a, 0x12, 0xe5, 0xf0, 0x7f, 0x45, 0xf1, 0x6b, 0x00, 0x05, 0xb5, 0xe2, 0x12, 0xfc, 0x0d, 0xac, 0x52, 0x9a, 0xb7, 0xd5, 0xed, 0x7b, 0x35, 0x1f, 0xd1, 0xe2
        ],
        aes_iv: [
            0xd4, 0xdb, 0x8c, 0x03, 0x10, 0x09, 0xe7, 0xf7, 0x1b, 0xf5, 0xb3, 0x3b, 0xc3, 0x47, 0x5c, 0x58
        ]
    },
    Secret{
        secret_type: SecretType::Channel(2),
        valid: true,
        aes_key: [
            0x7c, 0x91, 0xf8, 0x82, 0xff, 0x6c, 0xbc, 0xc2, 0x1d, 0xe3, 0xb1, 0x18, 0x06, 0x78, 0x26, 0x3c, 0x45, 0x5c, 0x96, 0x5d, 0x84, 0x42, 0x8d, 0xc4, 0x4f, 0x25, 0x21, 0xe8, 0x21, 0x0a, 0x8c, 0x90
        ],
        aes_iv: [
            0x1e, 0x11, 0x08, 0x2e, 0xb8, 0xb9, 0xb9, 0xe8, 0x51, 0xa6, 0x40, 0x2d, 0xed, 0xc2, 0x87, 0x6c
        ]
    },
    Secret{
        secret_type: SecretType::Channel(3),
        valid: true,
        aes_key: [
            0x72, 0xd3, 0x6d, 0xd9, 0xfe, 0x0e, 0x38, 0xd0, 0xb1, 0xd6, 0xf3, 0xc2, 0x8d, 0x05, 0x3e, 0x98, 0x85, 0xeb, 0xb9, 0x77, 0x68, 0x3d, 0x23, 0xb5, 0xb0, 0xe8, 0xc3, 0x4e, 0x08, 0x4f, 0xbe, 0x3d
        ],
        aes_iv: [
            0xa2, 0x9b, 0xdb, 0x54, 0xab, 0x9d, 0x1b, 0x6c, 0xc4, 0xdb, 0x6d, 0xd6, 0xb9, 0xf4, 0xc3, 0x2f
        ]
    },
    Secret{
        secret_type: SecretType::Channel(4),
        valid: true,
        aes_key: [
            0xc8, 0xd7, 0xdc, 0x5f, 0x91, 0x47, 0xd0, 0xaa, 0x7d, 0x3c, 0x6e, 0xc3, 0x1c, 0x40, 0xa4, 0xe8, 0x7b, 0x22, 0xb9, 0x61, 0x93, 0x47, 0xa6, 0xde, 0x0e, 0x2e, 0xc8, 0xc9, 0x43, 0x93, 0x68, 0xe2
        ],
        aes_iv: [
            0xbd, 0x94, 0x03, 0x3e, 0x65, 0xbf, 0x11, 0x56, 0xea, 0x0a, 0x0a, 0x4c, 0xa3, 0x33, 0x8a, 0x24
        ]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    },
    Secret{
        secret_type: SecretType::Master,
        valid: false,
        aes_key: [0; 32],
        aes_iv: [0; 16]
    }
];

#[link_section = ".decoder_id"]
#[no_mangle]
static DECODER_ID: u32 = 0xDEADBEEF;
