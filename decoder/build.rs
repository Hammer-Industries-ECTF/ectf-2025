//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
//!
//! The build script also sets the linker flags to tell it which link script to use.
//!
//! It also creates the flash memory data for subscriptions, secrets, and decoder ID.

use base64::prelude::*;
use std::env;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::path::Path;
use serde_json;
use serde_json::Value;

struct RawSecret {
    id: String,
    aes_key: Vec<u8>,
    iv: Vec<u8>,
}

// Must be copied from src
// vvv
type AesSubBlock = u8;
type AesBlock = [AesSubBlock; 16];
type AesKey = [u8; 32];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecretType {
    Channel(u32),
    Master
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(4))]
pub struct Secret {
    pub secret_type: SecretType,
    pub valid: bool,
    pub aes_key: AesKey,
    pub aes_iv: AesBlock
}
// ^^^

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    // Specify linker arguments.

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg=--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    println!("cargo:rustc-link-arg=-Tlink.x");

    //==================================//
    // Add secrets during build process //
    //==================================//
    // Read the secrets JSON file
    let decoder_id: u32 = u32::from_str_radix(&env::var("DECODER_ID").unwrap()[2..], 16).unwrap();

    let secrets_path: &Path = Path::new("/global.secrets");
    let secrets_str = fs::read_to_string(secrets_path).expect("Failed to read secrets.json");

    // Parse secrets JSON
    let secrets: Value = serde_json::from_str(&secrets_str).expect("Invalid JSON format");
    let secrets_map = match secrets {
        Value::Object(map) => { map },
        _ => { panic!("Invalid JSON format") },
    };

    let mut raw_secrets_vec = Vec::<RawSecret>::new();
    for (id, value) in secrets_map.into_iter() {
        let value_vec = match value {
            Value::Array(vec) => { vec },
            _ => { panic!("Invalid JSON format") },
        };

        let aes_key = match value_vec.get(0).unwrap() {
            Value::String(s) => { s.to_string() },
            _ => { panic!("Invalid JSON format") },
        };

        let iv = match value_vec.get(1).unwrap() {
            Value::String(s) => { s.to_string() },
            _ => { panic!("Invalid JSON format") },
        };

        let secret_arr = RawSecret {
            id: id, 
            aes_key: BASE64_STANDARD.decode(&aes_key).unwrap(),
            iv: BASE64_STANDARD.decode(&iv).unwrap(),
        };

        assert_eq!(secret_arr.aes_key.len(), 32);
        assert_eq!(secret_arr.iv.len(), 16);

        raw_secrets_vec.push(secret_arr);
    }

    // Prevent overfilling flash on build
    // Technically we should support an indeterminate number of valid channels,
    // but we saw the rule update about this too late in the process to change
    // to a system of dynamically communicating channel secrets.
    // We figured if you are going to try more than 128 channels for this competition,
    // oh well.
    assert!(raw_secrets_vec.len() <= 128);

    // Transform RawSecrets from JSON into Secrets for flash
    let mut secrets_vec = Vec::<Secret>::new();
    for raw_secret in raw_secrets_vec {
        let secret_type = match raw_secret.id.as_str() {
            "master" => { SecretType::Master }
            _ => { SecretType::Channel(raw_secret.id.parse::<u32>().unwrap()) }
        };

        let secret = Secret {
            secret_type: secret_type,
            valid: true,
            aes_key: raw_secret.aes_key.try_into().unwrap(),
            aes_iv: raw_secret.iv.try_into().unwrap(),
        };

        secrets_vec.push(secret);
    }

    let mut secrets_arr: [Secret; 128] = [
        Secret {
            secret_type: SecretType::Channel(u32::MAX),
            valid: false,
            aes_key: [0xFFu8; 32],
            aes_iv: [0xFFu8; 16],
        }; 128];

    let mut code_secrets_vec = Vec::<String>::new();
    for (sidx, secret) in secrets_vec.into_iter().enumerate() {
        secrets_arr[sidx] = secret;
    }

    for  secret in secrets_arr {
        let code_secret_type = match secret.secret_type {
            SecretType::Master          => { format!("SecretType::Master") },
            SecretType::Channel(c) => { format!("SecretType::Channel(0x{:08x}u32)", c) }, 
        };

        let code_valid = match secret.valid {
            true  => { format!("true") },
            false => { format!("false") },
        };

        let mut code_aes_key: String = String::from("["); 
        for aes_key_byte in secret.aes_key {
            let code_aes_key_byte = format!("0x{:02x}u8, ", aes_key_byte);
            code_aes_key.push_str(&code_aes_key_byte);
        }
        code_aes_key.push(']');

        let mut code_aes_iv: String = String::from("["); 
        for aes_iv_byte in secret.aes_iv {
            let code_aes_iv_byte = format!("0x{:02x}u8, ", aes_iv_byte);
            code_aes_iv.push_str(&code_aes_iv_byte);
        }
        code_aes_iv.push(']');

        let code_secret = format!(
            "Secret {{ secret_type: {}, valid: {}, aes_key: {}, aes_iv: {}, }},",
            code_secret_type, code_valid, code_aes_key, code_aes_iv,
        );

        code_secrets_vec.push(code_secret);
    }

    let code_secrets_arr = code_secrets_vec.join("\n");

    let code_header = r#"//! Flash Secrets
//! Generated by build script

use super::secure_memory::{Secret, SecretType, Subscription};"#;

    let code_subscriptions = r#"#[link_section = ".subscriptions"]
pub static SUBSCRIPTIONS: [Subscription; 8] = [
    Subscription{
        channel_id: 0,
        valid: false,
        end: 0,
        start: 0
    }; 8
];"#;

    let code_decoder_id: String = format!(r#"pub static DECODER_ID: u32 = 0x{:08x}u32;"#, decoder_id);

    let final_code = format!(
r#"{}

{}

{}

pub static SECRETS: [Secret; 128] = [
{}
];
"#,
        code_header,
        code_subscriptions,
        code_decoder_id,
        code_secrets_arr,
    );

    let flash_code_path = Path::new("src/sys/generated_flash.rs");
    fs::write(&flash_code_path, final_code).unwrap();
    println!("cargo:rerun-if-changed=/global.secrets");
    println!("cargo:rerun-if-changed=build.rs");    
}
