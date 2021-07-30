#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use cortex_m_rt::entry;

use stm32f1xx_hal::{
    prelude::*, 
};

use alloc_cortex_m::CortexMHeap;

use cortex_m::asm;
use core::alloc::Layout;

mod peripherals;
use peripherals::{Peripherals};

mod serial;
use serial::{SerialWriter};

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt; 

// initialize the heap allocator to be able to use dynamic sized types
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty(); 
const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {

    // Initialize heap allocator
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // initialize peripherals
    let peripherals = Peripherals::init();

    let mut led = peripherals.led.unwrap();
    let mut delay = peripherals.delay.unwrap();
    let mut serial = SerialWriter{ serial: peripherals.usart1.unwrap() };

    // main loop
    loop {
        led.set_high().ok();
        delay.delay_ms(1000 as u32);
        led.set_low().ok();
        delay.delay_ms(1000 as u32);
        serial.write_str("Hello there?\n").ok();
    }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}