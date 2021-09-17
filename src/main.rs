#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use cortex_m_rt::entry;

use cortex_m::asm;
use core::alloc::Layout;

use core::mem::MaybeUninit;

mod peripherals;
use peripherals::*;

mod serial;
use serial::{SerialWriter};

mod buttons;
use buttons::*;

mod timers;
use timers::{Timer2, Timer3};

mod debug;
use debug::*;

mod utils;

mod encoder;
use encoder::*;

mod clock;
use clock::*;

mod statemachine;
use statemachine::*;

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt;

static mut STATEMACHINE: MaybeUninit<Statemachine> = MaybeUninit::uninit();

fn on_button_press(changes: u8, state: u8) {
  debug!("button changed");
  debug!(changes as u16);
  debug!(state as u16);
}

fn on_encoder_change(rotation: i16) {
  debug!("encoder turn");
  let statemachine = unsafe { &mut *STATEMACHINE.as_mut_ptr() };
  statemachine.encoder_turn(rotation);
}

#[entry]
fn main() -> ! {
  // initialize statemachine
  let statemachine = unsafe { &mut *STATEMACHINE.as_mut_ptr() };
  *statemachine = Statemachine::new();

  // initialize peripherals
  let peripherals = Peripherals::init();
  
  let serial = SerialWriter::new(peripherals.usart1.unwrap());
  debug_init!(serial);
  
  let buttons = Buttons::new(peripherals.button1.unwrap(), peripherals.button2.unwrap(), 
    peripherals.button3.unwrap(), peripherals.button4.unwrap(), on_button_press);
  Timer2::add_handler(0, Buttons::on_tick);

  let mut clock = Clock::new(statemachine.get_state().bpm);
  Timer3::add_handler(0, Clock::on_timer_tick);
  
  let encoder = Encoder::new(on_encoder_change);
  
  // main loop
  loop {
    buttons.update();
    encoder.update();
    
    let state = statemachine.update();
    
    // statechange handler
    state.map(|s| {
      debug!("state changed");
      clock.set_bpm(s.bpm)
    });

  }
}

#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
  asm::bkpt();
  loop {}
}