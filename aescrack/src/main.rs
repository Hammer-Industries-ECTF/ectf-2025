#![no_std]
#![no_main]

pub extern crate max7800x_hal as hal;
use hal::pac;
use hal::entry;

// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_semihosting::heprintln; // uncomment to use this for printing through semihosting

#[entry]
fn main() -> ! {
    // heprintln!("Hello from semihosting!");
    let p = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut gcr = hal::gcr::Gcr::new(p.gcr, p.lpgcr);
    let ipo = hal::gcr::clocks::Ipo::new(gcr.osc_guards.ipo).enable(&mut gcr.reg);
    let clks = gcr.sys_clk
        .set_source(&mut gcr.reg, &ipo)
        .set_divider::<hal::gcr::clocks::Div1>(&mut gcr.reg)
        .freeze();

    // Initialize a delay timer using the ARM SYST (SysTick) peripheral
    let rate = clks.sys_clk.frequency;
    let mut delay = cortex_m::delay::Delay::new(core.SYST, rate);

    // Initialize the GPIO2 peripheral
    let pins = hal::gpio::Gpio2::new(p.gpio2, &mut gcr.reg).split();
    // Enable output mode for the RGB LED pins
    let mut led_r = pins.p2_0.into_input_output();
    let mut led_g = pins.p2_1.into_input_output();
    let mut led_b = pins.p2_2.into_input_output();
    // Use VDDIOH as the power source for the RGB LED pins (3.0V)
    // Note: This HAL API may change in the future
    led_r.set_power_vddioh();
    led_g.set_power_vddioh();
    led_b.set_power_vddioh();

    // Initialize and split the GPIO0 peripheral into pins
    let gpio0_pins = hal::gpio::Gpio0::new(p.gpio0, &mut gcr.reg).split();
    // Configure UART to host computer with 115200 8N1 settings
    let rx_pin = gpio0_pins.p0_0.into_af1();
    let tx_pin = gpio0_pins.p0_1.into_af1();
    let console = hal::uart::UartPeripheral::uart0(
        p.uart0,
        &mut gcr.reg,
        rx_pin,
        tx_pin
    )
        .baud(115200)
        .clock_pclk(&clks.pclk)
        .parity(hal::uart::ParityBit::None)
        .build();

    console.write_bytes(b"Hello, world!\r\n");

    let aes = hal::aes::Aes::new(
        p.aes,
        &mut gcr.reg
    );
    aes.set_key(&[0x0; 32]);

    delay.delay_ms(10);
    led_b.set_low();
    delay.delay_ms(500);
    led_b.set_high();
    loop {
        if led_g.is_high() {
            led_g.set_low();
        } else {
            led_g.set_high();
        }

        let enc_buf: &mut [u8; 16] = &mut [0; 16];
        let ctrl: u8 = console.read_byte();
        match ctrl {
            0 => {
                led_r.set_low();
                delay.delay_ms(1000);
                console.write_byte(0);
                led_r.set_high();
            }
            1 => {
                console.read_bytes(enc_buf);
                enc_buf.reverse();
                let dec_buf: &mut [u8; 16] = &mut aes.decrypt_block(*enc_buf).unwrap();
                dec_buf.reverse();
                console.write_bytes(dec_buf); // TODO not echo
            }
            2 => {
                let ctrl_w: [u8; 4] = unsafe { *(0x40007400 as *mut [u8; 4]) };
                console.write_bytes(&ctrl_w);
            }
            3..=u8::MAX => {}
        }
    }
}
