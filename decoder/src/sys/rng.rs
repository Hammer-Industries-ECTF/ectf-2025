//! Random number generator
//! For use by entire system
//! Uses the ChaCha20 RNG
//! Seeded by trng

use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};

pub fn new_rng(trng: hal::trng::Trng) -> ChaCha20Rng {
    let high = trng.gen_u32();
    let low = trng.gen_u32();
    let seed: u64 = ((high as u64) << 32) | (low as u64);
    ChaCha20Rng::seed_from_u64(seed)
}

pub fn delay_rand(rng: &mut ChaCha20Rng, delay: &mut cortex_m::delay::Delay) {
    let rand = rng.next_u32();
    let time_us = rand % 400 + 100;
    delay.delay_us(time_us);
}
