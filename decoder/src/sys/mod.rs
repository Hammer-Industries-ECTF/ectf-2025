//! Functions pertaining to operating the broad microcontroller system
//! NOT AN ACTUAL RTOS
//! Contains:
//! - Initialization of Flash memory
//! - Interupts / Handlers
//! - System Watchdog (his name is Cupcake) (Still awaiting shipment)

pub mod allocator;
pub mod secure_memory;
pub mod decrypt;
pub mod generated_flash;
pub mod rng;
