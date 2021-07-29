#![no_std]
#![no_main]

use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;

use stm32f1xx_hal::{
    prelude::*, 
};

mod peripherals;
use peripherals::{Peripherals};

mod serial;
use serial::{SerialWriter};

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {

    let peripherals = Peripherals::init();

    let mut led = peripherals.led.unwrap();
    let mut delay = peripherals.delay.unwrap();
    let mut serial = SerialWriter{ serial: peripherals.usart1.unwrap() };

    loop {
        led.set_high().ok();
        delay.delay_ms(100 as u32);
        led.set_low().ok();
        delay.delay_ms(100 as u32);
        serial.write_str("Hello there?\n").ok();
    }
}