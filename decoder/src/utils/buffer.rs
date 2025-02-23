//! Buffers
//! Statically defined
//! Ideally memory safe even with concurrent code
#![allow(dead_code)]

/// Fixed-size ring buffer
pub struct RingBuffer<const N: usize> {
    buffer: [u8; N],  // The underlying storage
    head: usize,       // Write position
    tail: usize,       // Read position
    count: usize,      // Number of elements
}

impl<const N: usize> RingBuffer<N> {
    /// Creates a new empty ring buffer
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    /// Adds a byte to the buffer, returns Err if full
    pub fn push(&mut self, byte: u8) -> Result<(), ()> {
        if self.count >= N {
            return Err(()); // Buffer is full
        }

        self.buffer[self.head] = byte;
        self.head = (self.head + 1) % N;
        self.count += 1;
        Ok(())
    }

    /// Removes and returns a byte from the buffer, returns None if empty
    pub fn pop(&mut self) -> Option<u8> {
        if self.count == 0 {
            return None; // Buffer is empty
        }

        let byte = self.buffer[self.tail];
        self.tail = (self.tail + 1) % N;
        self.count -= 1;
        Some(byte)
    }

    /// Returns true if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns true if the buffer is full
    pub fn is_full(&self) -> bool {
        self.count >= N
    }

    /// Check for overflow
    pub fn is_overflow(&self) -> bool {
        self.count > N || self.head > N
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
        self.count = 0;
    }
}

#[macro_export]
macro_rules! generate_buffers {
    ($($name:ident),+; $size:expr) => {
        use core::cell::RefCell;
        use cortex_m::interrupt::Mutex;
        use paste::paste;
        use utils::buffer::RingBuffer;

        $(
            static $name: Mutex<RefCell<RingBuffer<$size>>> = Mutex::new(RefCell::new(RingBuffer::new()));

            // Push a byte to this buffer
            paste! {
                #[allow(dead_code)]
                pub fn [<push_ $name:lower>](byte: u8) -> Result<(), ()> {
                    cortex_m::interrupt::free(|cs| {
                        $name.borrow(cs).borrow_mut().push(byte)
                    })
                }
            }

            // Pop a byte from this buffer
            paste! {
                #[allow(dead_code)]
                pub fn [<pop_ $name:lower>]() -> Option<u8> {
                    cortex_m::interrupt::free(|cs| {
                        $name.borrow(cs).borrow_mut().pop()
                    })
                }
            }


            // Check if this buffer is empty
            paste! {
                #[allow(dead_code)]
                pub fn [<is_empty_ $name:lower>]() -> bool {
                    cortex_m::interrupt::free(|cs| {
                        $name.borrow(cs).borrow().is_empty()
                    })
                }
            }

            // Clear this buffer
            paste! {
                #[allow(dead_code)]
                pub fn [<clear_ $name:lower>]() {
                    cortex_m::interrupt::free(|cs| {
                        $name.borrow(cs).borrow_mut().clear();
                    })
                }
            }
        )*
    };
}
