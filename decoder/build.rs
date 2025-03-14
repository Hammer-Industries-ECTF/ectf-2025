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

#[path = "src/sys/mod.rs"]
mod secure_memory;

use sys::secure_memory::{Secret, SecretType};
use std::default;
use std::env;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::raw;
use std::path::PathBuf;
use std::path::Path;
use serde::Deserialize;
use serde_json;
use serde_json::Map;
use serde_json::Value;

// include!("src/sys/secure_memory.rs");

struct RawSecret {
    id: String,
    aes_key: Vec<u8>,
    iv: Vec<u8>,
}

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
    let secrets_path = Path::new("../secrets.json");
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
            aes_key: base64_url::decode(&aes_key).unwrap(),
            iv: base64_url::decode(&iv).unwrap(),
        };
        raw_secrets_vec.push(secret_arr);
    }

    // Prevent overfilling flash on build
    assert!(raw_secrets_vec.len() < 128);

    let mut secrets_vec = Vec::<Secret>::new();
    for raw_secret in raw_secrets_vec {
        let secret_type = match raw_secret.id.as_str() {
            "master" => { SecretType::Master }
            _ => { SecretType::Channel(raw_secret.id.parse::<u32>().unwrap()) }
        };

        let secret = Secret {
            secret_type: secret_type,
            valid: true,
            aes_key: raw_secret.aes_key,
            aes_iv: raw_secret.iv,
        };

        secrets_vec.push(secret);
    }

    let mut secrets_arr: [Secret; 128] = [
        Secret {
            secret_type: SecretType::Channel(u32::MAX),
            valid: false,
            aes_key: [0xFFu8; 32],
            aes_iv: [0xFFu8; 8],
        }; 128];

    for (sidx, secret) in secrets_vec.into_iter().enumerate() {
        secrets_arr[sidx] = secret;
    }

    todo!("Decoder ID load");
    
}
