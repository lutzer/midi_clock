#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use cortex_m_rt::entry;

use cortex_m::asm;
use core::alloc::Layout;

mod peripherals;
use peripherals::*;

mod serial;
use serial::{SerialWriter};

mod buttons;
use buttons::*;

mod timers;
use timers::{Timer2};

mod debug;
use debug::*;

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt;

fn on_tick() {
    debug!("on_tick");
}

#[entry]
fn main() -> ! {


    // initialize peripherals
    let peripherals = Peripherals::init();

    let mut led = peripherals.led.unwrap();

    let serial = SerialWriter::new(peripherals.usart1.unwrap());

    // only use this in debug mode
    #[cfg(feature = "debug")]
    debug_init(serial);

    let buttons = Buttons::new(peripherals.button1.unwrap());

    Timer2::add_handler(0, on_tick);

    debug!("start");

    let mut pressed_before = false;

    // main loop
    loop {
        let pressed = buttons.read();
        if pressed && !pressed_before {
            led.set_low().ok();
            pressed_before = true;
        } else if !pressed {
            led.set_high().ok();
            pressed_before = false;
        }
        // led.set_high().ok();
        // delay.delay_ms(1000 as u32);
        // led.set_low().ok();
        // delay.delay_ms(100 as u32);
        // serial.write_str("Hello there?\n").ok();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}