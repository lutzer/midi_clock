use crate::debug;
use crate::debug::*;

use core::sync::atomic::{AtomicU16, AtomicBool, Ordering};

pub struct Clock {
  bpm: u16
}

static FREQ_OVERFLOWS: AtomicU16 = AtomicU16::new(0);
static RUNNING: AtomicBool = AtomicBool::new(true);

impl Clock {
  pub fn new(bpm: u16, running: bool) -> Clock {
    let mut clock = Clock{ bpm: 0 };
    clock.set_bpm(bpm);
    clock.set_running(running);
    return clock;
  }

  pub fn set_bpm(&mut self, bpm: u16) {
    self.bpm = bpm;
    debug!("set bpm");
    debug!(bpm);
    FREQ_OVERFLOWS.store(60000/bpm, Ordering::Relaxed);
  }

  pub fn set_running(&mut self, running: bool) {
    RUNNING.store(running, Ordering::Relaxed);
  }

  pub fn is_running(&self) -> bool {
    return RUNNING.load(Ordering::Relaxed)
  }

  pub fn on_timer_tick() {
    static mut OVERFLOWS: u16 = 0;

    // clock is not running
    if !RUNNING.load(Ordering::Relaxed) {
      return;
    }

    let freq_overflows = FREQ_OVERFLOWS.load(Ordering::Relaxed);

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