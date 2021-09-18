#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use cortex_m_rt::entry;
use cortex_m::interrupt;
use cortex_m::interrupt::{CriticalSection};

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

mod triggers;
use triggers::*;

mod statemachine;
use statemachine::*;

mod context;
use context::*;

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt;

static mut CONTEXT: CSContext = CS_CONTEXT_INIT;

fn on_button_press(statemachine: &mut Statemachine, changes: u8, state: u8) {
  debug!("button changed");
  if (changes & BUTTON1_MASK & state ) == 1 {
    statemachine.button1_pressed();
  }
}

fn on_encoder_change(statemachine: &mut Statemachine, rotation: i16) {
  debug!("encoder turn");
  statemachine.encoder_turn(rotation);
}

fn on_state_change(state: &State, clock: &mut Clock) {
  clock.set_running(state.running == RunState::RUNNING);
  clock.set_bpm(state.bpm);
}

fn on_clock_tick(_cs: &CriticalSection) {
  debug!("clock");
  let context = unsafe {&mut *CONTEXT.as_mut_ptr() };
  context.triggers.fire();
}

#[entry]
fn main() -> ! {
  // initialize statemachine
  let mut statemachine = Statemachine::new();
  let initial_state = statemachine.get_state();

  // initialize peripherals
  let peripherals = Peripherals::init();

  // create global context
  unsafe {
    CONTEXT.write(Context{
      triggers: Triggers::new( peripherals.led.unwrap() )
    });
  }
  
  let serial = SerialWriter::new(peripherals.usart1.unwrap());
  debug_init!(serial);

  
  let buttons = Buttons::new(peripherals.button1.unwrap(), peripherals.button2.unwrap(), 
    peripherals.button3.unwrap(), peripherals.button4.unwrap());
  Timer2::add_handler(0, Buttons::on_timer_tick);

  let mut clock = Clock::new(initial_state.bpm, initial_state.running == RunState::RUNNING);
  clock.on_tick(on_clock_tick);
  Timer3::add_handler(0, Clock::on_timer_tick);
  
  let encoder = Encoder::new();

  debug!("start");
  
  // main loop
  loop {
    buttons.on_change().map(|(changes, reading)| {
      on_button_press(&mut statemachine, changes, reading);
    });
    encoder.on_change().map(|rotation| {
      on_encoder_change(&mut statemachine, rotation);
    });
    statemachine.on_change().map(|state| {
      on_state_change(&state, &mut clock);
    });
  }
}