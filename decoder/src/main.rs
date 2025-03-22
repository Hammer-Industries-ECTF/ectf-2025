#![no_std]
#![no_main]

mod commands;
mod message;
mod sys;

pub extern crate max7800x_hal as hal;
use hal::pac;
use hal::entry;

use message::receive::receive_message;
use message::transmit::{transmit_err, transmit_message};
use commands::execute_command;

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
// use cortex_m_semihosting::heprintln; // uncomment to use this for printing through semihosting

use sys::allocator::init_heap;

#[entry]
fn main() -> ! {
    // Initialize embedded allocator
    init_heap();

    // Get HAL and PAC references to on-board devices
    let p = pac::Peripherals::take().unwrap();
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

    let aes = hal::aes::Aes::new(
        p.aes,
        &mut gcr.reg
    );

    let flc = hal::flc::Flc::new(p.flc, clks.sys_clk);

    // Main loop
    // On TXError, no recourse possible, so start main loop over
    'message_loop: loop {
        // Receive command from host device
        let host_message = receive_message(&flc, &uart, &aes);
        if host_message.is_err() {
            let _ = transmit_err(&uart, host_message.unwrap_err());
            continue 'message_loop;
        }
        let host_message = host_message.unwrap();

        // Execute instructions
        let response_message = execute_command(&flc, &aes, host_message);

        // Respond to host device
        match response_message {
            Ok(response) => {
                let _ = transmit_message(&uart, response);
            },
            Err(error) => {
                let _ = transmit_err(&uart, error);
            }
        }
    }
}
