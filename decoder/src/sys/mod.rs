//! Functions pertaining to operating the broad microcontroller system
//! NOT AN ACTUAL RTOS
//! Contains:
//! - Initialization of Flash memory
//! - Interupts / Handlers
//! - System Watchdog (his name is Cupcake)

pub mod allocator;

#[derive(Debug, Clone)]
pub struct Subscription {
    channel_id: u32,
    valid: bool,
    end: u64,
    start: u64
}