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

mod utils;

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt;


fn on_button_press(changes: u8, state: u8) {
    debug!("button changed");
    debug!(num_to_string(changes as u16));
    debug!(num_to_string(state as u16));
}

#[entry]
fn main() -> ! {
    // initialize peripherals
    let peripherals = Peripherals::init();

    // let mut led = peripherals.led.unwrap();

    let serial = SerialWriter::new(peripherals.usart1.unwrap());
    debug_init!(serial);

    let buttons = Buttons::new(peripherals.button1.unwrap(), peripherals.button2.unwrap(), 
        peripherals.button3.unwrap(), on_button_press);
    Timer2::add_handler(0, Buttons::on_tick);

    debug!("start");

    // main loop
    loop {
        buttons.update();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}