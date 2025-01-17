#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    //MAX78000 User guid, pg 111
    // Perform the following steps to configure a pin for output mode:
    // 1. Set the GPIO Configuration Enable bits shown in Table 6-2 to any one of the I/O mode settings.
        // Do Nothing
    // 2. Configure the electrical characteristics of the pin as desired, as shown in Table 6-4.
    // 3. Set the output logic high or logic low using the GPIOn_OUT.level[pin] bit.
    // 4. Enable the output buffer for the pin by setting GPIOn_OUTEN.en[pin] to 1
    // GPIO2 at 0x4008 0400
    // (GPIO2_OUTEN.en[0] = 1). 0x000C offset
    // GPIO2_OUT.level[0] = 1 0x001C offset

    unsafe {
        let gpio2_outen = 0x4008040C as *mut u32;
        let gpio2_out = 0x40080420 as *mut u32;
        
        *gpio2_outen |= 1; //enable output to LED
        *gpio2_out |= 1; //Turn on LED, active low


        loop {
            // *gpio2_out |= 1; //Turn on LED, active low
        }
    }

}
