//! How to use the heap and a dynamic memory allocator
//!
//! This example depends on the embedded-alloc crate so you'll have to add it to your Cargo.toml:
//!
//! ``` text
//! # or edit the Cargo.toml file manually
//! $ cargo add embedded-alloc
//! ```
//!
//! ---

extern crate alloc;

use embedded_alloc::LlffHeap as Heap;

// this is the allocator the application will use
#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init_heap() {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }
}
