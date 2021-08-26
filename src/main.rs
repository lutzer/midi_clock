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

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt; 

#[entry]
fn main() -> ! {

    // initialize peripherals
    let peripherals = Peripherals::init();

    let mut led = peripherals.led.unwrap();
    let serial = SerialWriter{ serial: peripherals.usart1.unwrap() };
    let buttons = Buttons::new(peripherals.button1.unwrap());

    // main loop
    loop {
        let pressed = buttons.read();
        if pressed {
            led.set_high().ok();
        } else {
            led.set_low().ok();
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