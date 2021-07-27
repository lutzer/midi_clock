#![no_std]
#![no_main]

use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;

use stm32f1xx_hal::{
    pac, 
    prelude::*, 
};

mod peripherals;
use peripherals::{Peripherals};

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {

    let peripherals = Peripherals::init();

    let mut led = peripherals.led.unwrap();
    let mut delay = peripherals.delay.unwrap();
    let mut serial = peripherals.usart1.unwrap();

    loop {
        led.set_high().ok();
        delay.delay_ms(100 as u32);
        led.set_low().ok();
        delay.delay_ms(100 as u32);
        let sent = b'T';
        serial.write(sent).ok();
    }
}