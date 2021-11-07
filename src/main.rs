#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::interrupt;
use cortex_m::interrupt::{CriticalSection};

use core::panic::PanicInfo;

mod peripherals;
use peripherals::{Peripherals};

mod serial;
use serial::{SerialWriter};

mod buttons;
use buttons::{Buttons, BUTTON1_MASK, BUTTON2_MASK, BUTTON3_MASK, BUTTON4_MASK, buttons_on_timer_tick};

mod timers;
use timers::{Timer3};

mod debug;

mod utils;

mod encoder;
use encoder::{Encoder};

mod clock;
use clock::{Clock};

mod triggers;
use triggers::{Triggers, TRIGGER4_MASK};

mod statemachine;
use statemachine::{Statemachine, State, RunState};

mod context;
use context::{Context, CONTEXT};

mod display;
use display::{Display};

mod midi;
use midi::{MidiMessage};

mod st7066;

mod eeprom;
use eeprom::{Eeprom};

mod memory;
use memory::{Memory};

fn on_button_press(statemachine: &mut Statemachine, changes: u8, state: u8) {
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
  statemachine.encoder_turn(rotation);
}

fn on_state_change(state: &State, clock: &mut Clock, display: &mut Display) {
  static mut PREV_STATE : Option<State> = None;

  debug!("state change");
  debug!(*state);

  if let Some(prev_state) = unsafe { PREV_STATE } {
    // check for state changes
    if prev_state.running != state.running {
      clock.set_runstate(state.running);
      send_midi_ctrl_msg(state.running);
    }
    if prev_state.bpm != state.bpm {
      clock.set_bpm(state.bpm);
    }
    if prev_state.clock_trigger_multiplier != state.clock_trigger_multiplier {
      clock.set_trigger_multiplier(state.clock_trigger_multiplier);
    }
    if prev_state.clock_divisions[0] != state.clock_divisions[0] || prev_state.clock_divisions[1] != state.clock_divisions[1] {
      clock.set_divisions(state.clock_divisions);
    }
    if prev_state.clock_bar_length != state.clock_bar_length {
      clock.set_bar_length(state.clock_bar_length);
    } 
    if prev_state.clock_sync != state.clock_sync {
      clock.sync(state.clock_sync);
    }
    display.update(state);
  }
  unsafe { PREV_STATE = Some(*state) }
}

fn send_midi_ctrl_msg(current: RunState) {
  interrupt::free(|cs| {
    Context::get_instance(cs, &|ctx| {
      match current {
        RunState::RUNNING => { 
          ctx.serial.write(2, MidiMessage::Continue as u8).ok(); 
        },
        RunState::PAUSED => { 
          ctx.serial.write(2, MidiMessage::Stop as u8).ok(); 
        },
        RunState::STOPPING => { 
          ctx.serial.write(2, MidiMessage::Stop as u8).ok(); 
        },
        RunState::STOPPED => { 
          ctx.serial.write(2, MidiMessage::Start as u8).ok();
          ctx.triggers.fire(TRIGGER4_MASK); // send sync reset trigger
        }
      }
    });
  });
}

#[entry]
fn main() -> ! {

  // initialize peripherals
  let peripherals = Peripherals::init();

  // init eeprom memory chip
  let mut memory = Memory::new(Eeprom::new(peripherals.i2c1.unwrap()));

  // initialize statemachine and read state from memory
  let mut statemachine = Statemachine::new(memory.load_state());
  let initial_state = statemachine.get_state();

  // initializes all buttons and sets debounce timer
  let buttons = Buttons::new(peripherals.button1.unwrap(), peripherals.button2.unwrap(), 
    peripherals.button3.unwrap(), peripherals.button4.unwrap());
  Timer3::add_handler(0, buttons_on_timer_tick);

  // initialize clock, sends triggers and MIDI CLOCK msgs in regular intervals
  let mut clock = Clock::new(&initial_state);
  
  // initialize rotary encoder
  let encoder = Encoder::new();

  // create global context to share peripherals among interrupts
  {
    let triggers = Triggers::new(
      peripherals.trigger1.unwrap(), 
      peripherals.trigger2.unwrap(),
      peripherals.trigger3.unwrap(),
      peripherals.trigger4.unwrap()
    );
    Timer3::add_handler(2, Triggers::on_timer_tick);
    let serial = SerialWriter::new(peripherals.usart1.unwrap(), peripherals.usart2.unwrap());
    
    interrupt::free(|cs| {
      let context = Context { triggers: triggers, serial: serial };
      CONTEXT.borrow(cs).replace(Some(context));
    });
  }

  // setup display
  let mut display = Display::new(peripherals.display.unwrap(), peripherals.delay.unwrap());
  Timer3::add_handler(1, Display::on_timer_tick);
  display.init();
  display.update(&initial_state);

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
      // memory.write_state(&state).ok();
    });
    display.render();
  }
}

// Call this function when panic occurs
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  debug!("PANIC");
  if let Some(s) = info.payload().downcast_ref::<&str>() {
    debug!(*s);
  }
  
  loop {}
}