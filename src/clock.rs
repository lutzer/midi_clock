use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::interrupt::{ CriticalSection };
use cortex_m::interrupt;

use crate::utils::{CSCell};
use crate::statemachine::{State, RunState};
use crate::triggers::{TRIGGER3_MASK, TRIGGER4_MASK};

pub struct Clock {
  bpm: u16
}

use crate::timers::{Timer2};

type ClockTickHandler = fn(u8, [bool;2], &CriticalSection);

const CLOCK_TICKS_PER_QUARTER_NOTE: u32 = 24;

static CLOCK_TICK_HANDLER: CSCell<Option<ClockTickHandler>> = CSCell::new(None);
static CLOCK_TICK_SETTINGS: AtomicU32 = AtomicU32::new(0);

fn clock_settings_store(divisions: [u8;2], triggers_ppq: u8, bar_length: u8, reset: bool, sync: bool) {
  let settings : u32 =  
    (divisions[0] as u32) | (divisions[1] as u32) << 8 |
    (triggers_ppq as u32) << 16 |
    (bar_length as u32) << 24 |
    (reset as u32) << 28 |
    (sync as u32) << 29;
  CLOCK_TICK_SETTINGS.store(settings, Ordering::Relaxed)
}

fn clock_settings_load() -> ([u8;2], u8, u8, bool, bool) {
  let settings_u32 = CLOCK_TICK_SETTINGS.load(Ordering::Relaxed);
  return(
    [(settings_u32) as u8, (settings_u32 >> 8) as u8],
    (settings_u32 >> 16) as u8,
    (settings_u32 >> 24 & 0xF) as u8,
    (settings_u32 >> 28 & 0b1) == 1,
    (settings_u32 >> 29 & 0b1) == 1,
  )
}

fn clock_settings_store_reset(reset: bool) {
  CLOCK_TICK_SETTINGS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
    return Some((x & !(1 << 28)) | (reset as u32) << 28);
  }).ok();
}

fn clock_settings_store_sync(sync: bool) {
  CLOCK_TICK_SETTINGS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
    return Some((x & !(1 << 29)) | (sync as u32) << 29);
  }).ok();
}

impl Clock {
  pub fn new(state: &State) -> Clock {
    let mut clock = Clock{ bpm: 1 };
    clock.set_bpm(state.bpm);
    clock.set_runstate(state.running);

    clock_settings_store(
      state.clock_divisions, 
      state.clock_trigger_multiplier, 
      state.clock_bar_length, 
      false, 
      state.clock_sync
    );

    Timer2::set_handler(Clock::on_timer_tick);
    return clock;
  }

  pub fn set_divisions(&self, divisions: [u8;2]) {
    CLOCK_TICK_SETTINGS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      return Some((x & !0xFFFF) | (divisions[0] as u32) | (divisions[1] as u32) << 8);
    }).ok();
  }

  pub fn set_trigger_multiplier(&self, multiplier: u8) {
    CLOCK_TICK_SETTINGS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      return Some((x & !0xFF << 16) | (multiplier as u32) << 16);
    }).ok();
  }

  pub fn set_bar_length(&self, bar_length: u8) {
    CLOCK_TICK_SETTINGS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      return Some((x & !0xF << 24) | (bar_length as u32) << 24);
    }).ok();
  }

  pub fn sync(&self) {
    clock_settings_store_sync(true);
  }

  pub fn set_bpm(&mut self, bpm: u16) {
    self.bpm = bpm;
    // sends CLOCK_TICKS_PER_QUARTER_NOTE triggers for every quarternote
    let intervall_in_us : u32 = 60 * 1000 * 1000 / ((self.bpm as u32) * CLOCK_TICKS_PER_QUARTER_NOTE);
    Timer2::set_interval(intervall_in_us);
  }

  pub fn set_runstate(&mut self, running: RunState) {
    match running {
      RunState::RUNNING => {
        Timer2::set_running(true);
      },
      RunState::STOPPED => {
        clock_settings_store_reset(true);
        Timer2::set_running(false);
      },
      _ => {
        Timer2::set_running(false);
      }
    }
  }

  pub fn on_tick<'a>(&self, cb: ClockTickHandler) {
    interrupt::free( |cs| CLOCK_TICK_HANDLER.set(Some(cb), cs) );
  }

  pub unsafe fn on_timer_tick(cs : &CriticalSection) {
    static mut OVERFLOWS : u32 = 0;

    let (divisions, triggers_ppq, bar_length, reset, sync) = clock_settings_load();

    // reset Clock
    OVERFLOWS = if reset { 0 } else { OVERFLOWS };
    clock_settings_store_reset(false);

    let mut triggers: u8 = 0;
    let mut midi_outs = [ false, false ];
    
    // handle the two midi channels
    for i in 0..2 {
      if OVERFLOWS % (divisions[i] as u32 * CLOCK_TICKS_PER_QUARTER_NOTE) == 0 {
        triggers |= 1 << i;
      }
      if OVERFLOWS % divisions[i] as u32 == 0 {
        midi_outs[i] = true
      }
    }

    // handle the trigger out
    if OVERFLOWS % (CLOCK_TICKS_PER_QUARTER_NOTE/triggers_ppq as u32) == 0 {
      triggers |= TRIGGER3_MASK;
    }

    // handle reset out after 4 quarter notes
    if sync && OVERFLOWS % (CLOCK_TICKS_PER_QUARTER_NOTE * bar_length as u32) == 0 {
      clock_settings_store_sync(false);
      triggers |= TRIGGER4_MASK;
    }

    CLOCK_TICK_HANDLER.get(cs).map(|f| f(triggers, midi_outs, cs) ); 
    // reset overflows when reached largest common multiple of divisors and 24
    OVERFLOWS = (OVERFLOWS + 1) % 806400; 
  }
}