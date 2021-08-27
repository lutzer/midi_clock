#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use cortex_m_rt::entry;

// use stm32f1xx_hal::{
//     prelude::*
// };

use cortex_m::asm;
use core::alloc::Layout;

mod peripherals;
use peripherals::*;

mod serial;
use serial::{SerialWriter};

mod buttons;
use buttons::*;

mod timers;
use timers::*;

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt;

#[entry]
fn main() -> ! {

    // initialize peripherals
    let peripherals = Peripherals::init();

    let mut led = peripherals.led.unwrap();
    let mut serial = SerialWriter{ serial: peripherals.usart1.unwrap() };
    let buttons = Buttons{ button1: peripherals.button1.unwrap() };

    let mut on_timer_listener = || {
        serial.write_str("tick\n").ok();
    };

    let mut timers = Timers::new();
    timers.add_handler(&mut on_timer_listener);

    // main loop
    loop {
        let pressed = buttons.read();
        if pressed {
            led.set_low().ok();
            timers.emit();
        } else {
            led.set_high().ok();
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