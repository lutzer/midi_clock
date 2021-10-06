#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use cortex_m_rt::entry;
use cortex_m::interrupt;
use cortex_m::interrupt::{CriticalSection};

mod peripherals;
use peripherals::*;

mod serial;
use serial::*;

mod buttons;
use buttons::*;

mod timers;
use timers::*;

mod debug;

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

mod display;
use display::*;

mod midi;
use midi::*;

// When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use panic_halt;

// holds vars that need to be globally available
pub static CONTEXT: CSContext = CS_CONTEXT_INIT;

fn on_button_press(statemachine: &mut Statemachine, changes: u8, state: u8) {
  debug!("button changed");
  if (changes & BUTTON1_MASK) > 0 {
    statemachine.button1_pressed(changes & BUTTON1_MASK & state > 0);
  }
  if (changes & BUTTON2_MASK) > 0 {
    statemachine.button2_pressed(changes & BUTTON2_MASK & state > 0);
  }
  if (changes & BUTTON3_MASK) > 0 {
    statemachine.button3_pressed(changes & BUTTON3_MASK & state > 0);
  }
  if (changes & BUTTON4_MASK) > 0 {
    statemachine.encoder_pressed(changes & BUTTON4_MASK & state > 0);
  }
}

fn on_encoder_change(statemachine: &mut Statemachine, rotation: i16) {
  debug!("encoder turn");
  statemachine.encoder_turn(rotation);
}

fn on_state_change(state: &State, clock: &mut Clock, display: &mut Display) {
  static mut PREV_STATE : Option<State> = None;

  debug!("state change (run/bpm)");
  debug!(state.running as u16);
  debug!(state.bpm);

  if let Some(prev_state) = unsafe { PREV_STATE } {
    // check for state changes
    if prev_state.running != state.running {
      clock.set_running(state.running == RunState::RUNNING || state.running == RunState::SYNC);
      send_midi_ctrl_msg(state.running, prev_state.running);
    }
    if prev_state.bpm != state.bpm {
      clock.set_bpm(state.bpm);
      display.update(state);
    }
  } else {
    // set initial state
    clock.set_running(state.running == RunState::RUNNING);
    clock.set_bpm(state.bpm);
    display.update(state);
  }

  unsafe { PREV_STATE = Some(*state) }
}

fn send_midi_ctrl_msg(current: RunState, _: RunState) {
  interrupt::free(|cs| {
    let mut context = CONTEXT.borrow(cs).borrow_mut();
    context.as_mut().map(|ctx| {
      match current {
        RunState::RUNNING => ctx.serial.write(2, MidiMessage::Continue as u8).ok(),
        RunState::PAUSED => ctx.serial.write(2, MidiMessage::Stop as u8).ok(),
        RunState::STOPPED => ctx.serial.write(2, MidiMessage::Stop as u8).ok(),
        RunState::RESTART => ctx.serial.write(2, MidiMessage::Start as u8).ok(),
        _ => None
      }
    });
  });
}

fn on_clock_tick(_clock: u8, big_tick: bool, cs: &CriticalSection) {
  let mut context = CONTEXT.borrow(cs).borrow_mut();
  context.as_mut().map(|ctx| {
    ctx.serial.write(2, MidiMessage::TimingClock as u8).ok();
    if big_tick {
      ctx.triggers.fire()
    }
  });
}

#[entry]
fn main() -> ! {
  // initialize statemachine
  let mut statemachine = Statemachine::new();
  let initial_state = statemachine.get_state();

  // initialize peripherals
  let peripherals = Peripherals::init();

  let buttons = Buttons::new(peripherals.button1.unwrap(), peripherals.button2.unwrap(), 
    peripherals.button3.unwrap(), peripherals.button4.unwrap());
  Timer3::add_handler(0, Buttons::on_timer_tick);

  let mut clock = Clock::new(initial_state.bpm, initial_state.running == RunState::RUNNING);
  clock.on_tick(on_clock_tick);
  Timer2::set_handler(Clock::on_timer_tick);
  
  let encoder = Encoder::new();

  let mut display = Display::new(peripherals.displayi2c.unwrap());
  Timer3::add_handler(1, Display::on_timer_tick);
  display.init();

  // create global context to share peripherals among interrupts
  {
    let triggers = Triggers::new(peripherals.led.unwrap());
    Timer3::add_handler(2, Triggers::on_timer_tick);
    let serial = SerialWriter::new(peripherals.usart1.unwrap(), peripherals.usart2.unwrap());
    
    interrupt::free(|cs| {
      let context = Context { triggers: triggers, serial: serial };
      CONTEXT.borrow(cs).replace(Some(context));
    });
  }

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
      on_state_change(&state, &mut clock, &mut display);
    });
    display.on_update().map(|_| {
      // interrupt::free(|cs| {
      //   display.flush(cs);
      // });
    });
    
  }
}