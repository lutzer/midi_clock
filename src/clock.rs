use crate::debug;
use crate::debug::*;

use core::sync::atomic::{AtomicU16, Ordering};

pub struct Clock {
  bpm: u16,
  running: bool
}

static FREQ_OVERFLOWS: AtomicU16 = AtomicU16::new(0);

impl Clock {
  pub fn new(bpm: u16) -> Clock {
    let mut clock = Clock{ bpm: 0, running: false };
    clock.set_bpm(bpm);
    return clock;
  }

  pub fn set_bpm(&mut self, bpm: u16) {
    self.bpm = bpm;
    debug!("set bpm");
    debug!(bpm);
    FREQ_OVERFLOWS.store(60000/bpm, Ordering::Relaxed);
  }

  pub fn set_running(&mut self, running: bool) {
    self.running = running;
    if !running {
      FREQ_OVERFLOWS.store(0, Ordering::Relaxed);
    } else {
      FREQ_OVERFLOWS.store(60000/self.bpm, Ordering::Relaxed);
    }
  }

  pub fn is_running(&self) -> bool {
    return self.running;
  }

  pub fn on_timer_tick() {
    static mut OVERFLOWS: u16 = 0;
    
    let freq_overflows = FREQ_OVERFLOWS.load(Ordering::Relaxed);

    // clock is paused
    if freq_overflows == 0 {
      return;
    }

    // clock is running
    unsafe { 
      if OVERFLOWS % freq_overflows == 0 {
        debug!("clock");
        OVERFLOWS = 1;
      } else {
        OVERFLOWS += 1;
      }
    }
  }
}