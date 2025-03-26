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
use sys::rng::{new_rng, delay_rand};

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

    // Initialize a delay timer using the ARM SYST (SysTick) peripheral
    let rate = clks.sys_clk.frequency;
    let mut delay = cortex_m::delay::Delay::new(core.SYST, rate);

    let aes = hal::aes::Aes::new(
        p.aes,
        &mut gcr.reg
    );

    let flc = hal::flc::Flc::new(p.flc, clks.sys_clk);
    let _simo = hal::simo::Simo::new(p.simo, &mut gcr.reg);
    let trng = hal::trng::Trng::new(p.trng, &mut gcr.reg);
    let mut rng = new_rng(trng);

    // Main loop
    // On TXError, no recourse possible, so start main loop over
    'message_loop: loop {
        // Receive command from host device
        let host_message = receive_message(&flc, &uart, &aes);
        delay_rand(&mut rng, &mut delay);

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
