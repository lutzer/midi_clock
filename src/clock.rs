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

struct ClockSettings {
  divisions: [u8;2],
  triggers_ppq: u8,
  bar_length: u8,
  reset: bool,
  sync: bool
}
impl ClockSettings {
  // pub fn store(divisions: [u8;2], triggers_ppq: u8, bar_length: u8, reset: bool, sync: bool) {
  pub fn store(s: ClockSettings) {
    let settings_u32 : u32 =  
      (s.divisions[0] as u32) | (s.divisions[1] as u32) << 8 |
      (s.triggers_ppq as u32) << 16 |
      (s.bar_length as u32) << 24 |
      (s.reset as u32) << 28 |
      (s.sync as u32) << 29;
    CLOCK_TICK_SETTINGS.store(settings_u32, Ordering::Relaxed)
  }

  pub fn store_reset(reset: bool) {
    CLOCK_TICK_SETTINGS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      return Some((x & !(1 << 28)) | (reset as u32) << 28);
    }).ok();
  }

  pub fn read(reset: bool) -> ClockSettings {
    let settings_u32 = CLOCK_TICK_SETTINGS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |s| {
      // set reset bit to zero
      return if reset { Some(s & !(1 << 28)) } else { Some(s) };
    }).unwrap();
    return ClockSettings {
      divisions: [(settings_u32) as u8, (settings_u32 >> 8) as u8],
      triggers_ppq: (settings_u32 >> 16) as u8,
      bar_length: (settings_u32 >> 24 & 0xF) as u8,
      reset: (settings_u32 >> 28 & 0b1) == 1,
      sync: (settings_u32 >> 29 & 0b1) == 1,
    };
  }
}

impl Clock {
  pub fn new(state: &State) -> Clock {
    let mut clock = Clock{ bpm: 1 };
    clock.set_bpm(state.bpm);
    clock.set_runstate(state.running);

    ClockSettings::store( ClockSettings {
      divisions: state.clock_divisions, 
      triggers_ppq: state.clock_trigger_multiplier, 
      bar_length: state.clock_bar_length, 
      reset: false, 
      sync: state.clock_sync
      }
    );

    Timer2::set_handler(Clock::on_timer_tick);
    return clock;
  }

  pub fn set_divisions(&self, divisions: [u8;2]) {
    let mut settings = ClockSettings::read(false);
    settings.divisions = [divisions[0], divisions[1]];
    ClockSettings::store(settings)
  }

  pub fn set_trigger_multiplier(&self, multiplier: u8) {
    let mut settings = ClockSettings::read(false);
    settings.triggers_ppq = multiplier;
    ClockSettings::store(settings);
  }

  pub fn set_bar_length(&self, bar_length: u8) {
    let mut settings = ClockSettings::read(false);
    settings.bar_length = bar_length;
    ClockSettings::store(settings);
  }

  pub fn sync(&self, sync: bool) {
    let mut settings = ClockSettings::read(false);
    settings.sync = sync;
    ClockSettings::store(settings);
  }

  pub fn set_bpm(&mut self, bpm: u16) {
    self.bpm = bpm;

    // sends 24 triggers for every quarternote
    let intervall_in_us : u32 = 60 * 1000 * 1000 / ((self.bpm as u32) * CLOCK_TICKS_PER_QUARTER_NOTE);
    Timer2::set_interval(intervall_in_us);
  }

  pub fn set_runstate(&mut self, running: RunState) {
    match running {
      RunState::RUNNING => {
        Timer2::set_running(true);
      },
      RunState::STOPPED => {
        ClockSettings::store_reset(true);
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
    static mut SYNC : bool = false;

    let csettings = ClockSettings::read(true);

    SYNC = SYNC || csettings.sync;

    // reset Clock
    OVERFLOWS = if csettings.reset { 0 } else { OVERFLOWS };

    let mut triggers: u8 = 0;
    let mut midi_outs = [ false, false ];
    
    // handle the two midi channels
    for i in 0..2 {
      if OVERFLOWS % (csettings.divisions[i] as u32 * CLOCK_TICKS_PER_QUARTER_NOTE) == 0 {
        triggers |= 1 << i;
      }
      if OVERFLOWS % csettings.divisions[i] as u32 == 0 {
        midi_outs[i] = true
      }
    }

    // handle the trigger out
    if OVERFLOWS % (CLOCK_TICKS_PER_QUARTER_NOTE/csettings.triggers_ppq as u32) == 0 {
      triggers |= TRIGGER3_MASK;
    }

    // handle reset out after 4 quarter notes
    if SYNC && OVERFLOWS % (CLOCK_TICKS_PER_QUARTER_NOTE * csettings.bar_length as u32) == 0 {
      SYNC = false;
      triggers |= TRIGGER4_MASK;
    }

    CLOCK_TICK_HANDLER.get(cs).map(|f| f(triggers, midi_outs, cs) ); 

    // reset overflows when reached largest common multiple of all possible divisors and 24
    OVERFLOWS = (OVERFLOWS + 1) % 806400; 
  }
}