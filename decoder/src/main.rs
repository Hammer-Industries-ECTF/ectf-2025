#![no_std]
#![no_main]

mod commands;
mod message;
mod sys;
mod utils;

pub extern crate max7800x_hal as hal;
use hal::pac;
use hal::entry;

use message::receive::receive_message;
use message::transmit::{transmit_err, transmit_message};
use commands::execute_command;
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
// use cortex_m_semihosting::heprintln; // uncomment to use this for printing through semihosting

use sys::allocator::init_heap;

#[entry]
fn main() -> ! {
    // heprintln!("Hello, World! You're semihosting!");
    init_heap();
    let p = pac::Peripherals::take().unwrap();
    // let core = pac::CorePeripherals::take().unwrap();

    let mut gcr = hal::gcr::Gcr::new(p.gcr, p.lpgcr);
    let ipo = hal::gcr::clocks::Ipo::new(gcr.osc_guards.ipo).enable(&mut gcr.reg);
    let clks = gcr.sys_clk
        .set_source(&mut gcr.reg, &ipo)
        .set_divider::<hal::gcr::clocks::Div1>(&mut gcr.reg)
        .freeze();

    // Initialize and split the GPIO2 peripheral into pins
    let gpio2_pins = hal::gpio::Gpio2::new(p.gpio2, &mut gcr.reg).split();
    // Enable output mode for the RGB LED pins
    let mut led_r = gpio2_pins.p2_0.into_input_output();
    let mut led_g = gpio2_pins.p2_1.into_input_output();
    let mut led_b = gpio2_pins.p2_2.into_input_output();
    led_r.set_power_vddioh();
    led_g.set_power_vddioh();
    led_b.set_power_vddioh();

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

    // TODO INIT FLASH
    // TODO INIT SECRETS
    // TODO INIT SUBSCRIPTION MEMORY
    
    // INIT TESTING BUFFERS

    led_r.set_low();
    led_g.set_high();
    led_b.set_low();

    'message_loop: loop {
        let host_message = receive_message(&uart, &aes);
        if host_message.is_err() {
            let transmit = transmit_err(&uart, host_message.unwrap_err());
            if transmit.is_err() { todo!(); } // panic? reset?
            continue 'message_loop;
        }
        let host_message = host_message.unwrap();

        let response_message = execute_command(&aes, host_message);

        match response_message {
            Ok(response) => {
                let transmit = transmit_message(&uart, response);
                if transmit.is_err() { todo!(); } // panic? reset?
            },
            Err(error) => {
                let transmit = transmit_err(&uart, error);
                if transmit.is_err() { todo!(); } // panic? reset?
            }
        }
    }
}
