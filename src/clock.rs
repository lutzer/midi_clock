use crate::debug;
use crate::debug::*;

use core::sync::atomic::{AtomicU16, AtomicBool, Ordering};
use cortex_m::interrupt::{ Mutex, CriticalSection };
use core::cell::{ RefCell };
use core::mem::MaybeUninit;

pub struct Clock {
  bpm: u16,
}

type ClockTickHandler = fn(&CriticalSection);

static FREQ_OVERFLOWS: AtomicU16 = AtomicU16::new(0);
static RUNNING: AtomicBool = AtomicBool::new(true);
static mut CLOCK_TICK_HANDLER: MaybeUninit<ClockTickHandler> = MaybeUninit::uninit();

impl Clock {
  pub fn new(bpm: u16, running: bool) -> Clock {
    // let mut clock = unsafe { &mut *CLOCK.as_mut_ptr() };
    let mut clock = Clock{ bpm: 1 };
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
    debug!("set running");
    debug!(running);
    RUNNING.store(running, Ordering::Relaxed);
  }


  pub fn on_tick<'a>(&self, cb: ClockTickHandler) {
    unsafe { CLOCK_TICK_HANDLER.write(cb); }
  }

  pub fn on_timer_tick(cs : &CriticalSection) {
    static mut OVERFLOWS: u16 = 0;

    // clock is not running
    if !RUNNING.load(Ordering::Relaxed) {
      return;
    }

    let freq_overflows = FREQ_OVERFLOWS.load(Ordering::Relaxed);

    // clock is running
    unsafe { 
      if OVERFLOWS % freq_overflows == 0 {
        (*CLOCK_TICK_HANDLER.as_mut_ptr())(cs);
        OVERFLOWS = 1;
      } else {
        OVERFLOWS += 1;
      }
    }
  }
}