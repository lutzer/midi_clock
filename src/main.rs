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

mod encoder;
use encoder::*;

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt;


fn on_button_press(changes: u8, state: u8) {
  debug!("button changed");
  debug!(changes as u16);
  debug!(state as u16);
}

fn on_encoder_change(rotation: i32) {
  debug!("encoder changed");
  debug!(rotation as i16);
}

#[entry]
fn main() -> ! {
  // initialize peripherals
  let peripherals = Peripherals::init();
  
  let serial = SerialWriter::new(peripherals.usart1.unwrap());
  debug_init!(serial);
  
  let buttons = Buttons::new(peripherals.button1.unwrap(), peripherals.button2.unwrap(), 
    peripherals.button3.unwrap(), peripherals.button4.unwrap(), on_button_press);
  Timer2::add_handler(0, Buttons::on_tick);
  
  let encoder = Encoder::new(on_encoder_change);
  
  debug!("start");
  
  // main loop
  loop {
    buttons.update();
    encoder.update();
  }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
  asm::bkpt();
  loop {}
}