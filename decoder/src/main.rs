#![no_std]
#![no_main]

mod commands;
mod message;
mod sys;
mod utils;

pub extern crate max7800x_hal as hal;
use hal::pac;
use hal::entry;

// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_semihosting::heprintln; // uncomment to use this for printing through semihosting

use sys::allocator::init_heap;

#[entry]
fn main() -> ! {
    heprintln!("Hello, World! You're semihosting!");
    init_heap();
    let p = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut gcr = hal::gcr::Gcr::new(p.gcr, p.lpgcr);
    let ipo = hal::gcr::clocks::Ipo::new(gcr.osc_guards.ipo).enable(&mut gcr.reg);
    let clks = gcr.sys_clk
        .set_source(&mut gcr.reg, &ipo)
        .set_divider::<hal::gcr::clocks::Div1>(&mut gcr.reg)
        .freeze();

    // Initialize and split the GPIO0 peripheral into pins
    let gpio0_pins = hal::gpio::Gpio0::new(p.gpio0, &mut gcr.reg).split();
    // Configure UART to host computer with 115200 8N1 settings
    let rx_pin = gpio0_pins.p0_0.into_af1();
    let tx_pin = gpio0_pins.p0_1.into_af1();
    let uart = hal::uart::UartPeripheral::uart0(
        p.uart0,
        &mut gcr.reg,
        rx_pin,
        tx_pin
    )
        .baud(115200)
        .clock_pclk(&clks.pclk)
        .parity(hal::uart::ParityBit::None)
        .build();

    // TODO INIT FLASH
    // TODO INIT SECRETS
    // TODO INIT SUBSCRIPTION MEMORY

    // TODO INIT AES
    
    // INIT TEMP BUFFERS

    loop {
        // RX

        // *DECRYPT

        // CALL FUNCTION
        let host_message = message::HostMessage::Debug;
        let response_message = commands::message_respond(host_message);

        // TX
    }

    todo!();

}
