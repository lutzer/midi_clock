use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use cortex_m::interrupt::{ CriticalSection };
use cortex_m::interrupt;

use crate::utils::{CSCell};
use crate::statemachine::{State, RunState};

pub struct Clock {
  bpm: u16
}

use crate::timers::{Timer2};

type ClockTickHandler = fn(u8, [bool;2], &CriticalSection);

const CLOCK_TICKS_PER_QUARTER_NOTE: u32 = 24;

static CLOCK_TICK_HANDLER: CSCell<Option<ClockTickHandler>> = CSCell::new(None);
static CLOCK_DIVISIONS: AtomicU32 = AtomicU32::new(0);
static TRIGGER_TICKS_PER_QUARTERNOTE: AtomicU8 = AtomicU8::new(0);

impl Clock {
  pub fn new(state: &State) -> Clock {
    let mut clock = Clock{ bpm: 1 };
    clock.set_bpm(state.bpm);
    clock.set_running(false);
    clock.set_running(state.running == RunState::RUNNING );
    clock.set_divisions(state.clock_divisions);
    clock.set_trigger_multiplier(state.trigger_clock_multiplier);

    Timer2::set_handler(Clock::on_timer_tick);
    return clock;
  }

  pub fn set_divisions(&mut self, divisions: [u8;4]) {
    CLOCK_DIVISIONS.store( 
      divisions[0] as u32 |
      (divisions[1] as u32) << 8 |
      (divisions[2] as u32) << 16 |
      (divisions[3] as u32) << 24
      ,Ordering::Relaxed)
  }

  pub fn set_trigger_multiplier(&mut self, multiplier: u8) {
    let ticks_per_quarternote: u8 = CLOCK_TICKS_PER_QUARTER_NOTE as u8 / multiplier;
    TRIGGER_TICKS_PER_QUARTERNOTE.store(ticks_per_quarternote, Ordering::Relaxed);
  }

  pub fn set_bpm(&mut self, bpm: u16) {
    self.bpm = bpm;
    // sends CLOCK_TICKS_PER_QUARTER_NOTE triggers for every quarternote
    let intervall_in_us : u32 = 60 * 1000 * 1000 / ((self.bpm as u32) * CLOCK_TICKS_PER_QUARTER_NOTE);
    Timer2::set_interval(intervall_in_us);
  }

  pub fn set_running(&mut self, running: bool) {
    static mut PREV: bool = true;
    if unsafe { PREV != running } {
      Timer2::set_running(running);
    }
    unsafe { PREV = running }
  }


  pub fn on_tick<'a>(&self, cb: ClockTickHandler) {
    interrupt::free( |cs| CLOCK_TICK_HANDLER.set(Some(cb), cs) );
  }

  pub fn on_timer_tick(cs : &CriticalSection) {
    static mut OVERFLOWS : u32 = 0;

    let mut triggers: u8 = 0;
    let mut midi_outs = [ false, false ];

    let divisions_u32 = CLOCK_DIVISIONS.load(Ordering::Relaxed);
    
    // handle the two midi channels
    for i in 0..2 {
      let division = (divisions_u32 >> (i*8) & 0xFF) as u32;
      if unsafe { OVERFLOWS % (division * CLOCK_TICKS_PER_QUARTER_NOTE) == 0 } {
        triggers |= 1 << i;
      }
      if unsafe { OVERFLOWS % division == 0 } {
        midi_outs[i] = true
      }
    }

    let trigger_ticks_per_quarternote = TRIGGER_TICKS_PER_QUARTERNOTE.load(Ordering::Relaxed) as u32;

    // handle the two trigger outs
    for i in 2..4 {
      let division = (divisions_u32 >> (i*8) & 0xFF) as u32;
      if unsafe { OVERFLOWS % (division * trigger_ticks_per_quarternote) == 0 } {
        triggers |= 1 << i;
      }
    }

    unsafe {
      CLOCK_TICK_HANDLER.get(cs).map(|f| f(triggers, midi_outs, cs) ); 
      OVERFLOWS = (OVERFLOWS + 1) % 806400; // send a big step every 24 steps
    }
  }
}