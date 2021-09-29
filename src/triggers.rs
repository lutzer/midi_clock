use crate::peripherals::{Led1Gpio};

pub use embedded_hal::digital::v2::{OutputPin, InputPin};
use cortex_m::interrupt::{CriticalSection};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::{CONTEXT};

// stop pulse after 50ms
const TIMER_OVERFLOW_COUNT: u8 = 50;

static TRIGGER_STARTED: AtomicBool = AtomicBool::new(false);

pub struct Triggers {
  led1: Led1Gpio
}

impl Triggers {
  pub fn new(led1: Led1Gpio) -> Triggers {
    return Triggers { led1: led1 }
  }

  pub fn fire(&mut self) {
    self.start_pulse();
  }

  fn start_pulse(&mut self) {
    self.led1.set_low().ok();
    TRIGGER_STARTED.store(true, Ordering::Relaxed);
  }

  fn stop_pulse(&mut self) {
    self.led1.set_high().ok();
    TRIGGER_STARTED.store(false, Ordering::Relaxed);
  }

  pub fn on_timer_tick(cs: &CriticalSection) {
    static mut OVERFLOWS: u8 = 0;

    if !TRIGGER_STARTED.load(Ordering:: Relaxed) {
      unsafe { OVERFLOWS = 0 } // reset overflows
      return;
    }
  
    if unsafe { OVERFLOWS > TIMER_OVERFLOW_COUNT } {
      // stop trigger pulse
      let mut context = CONTEXT.borrow(cs).borrow_mut();
      context.as_mut().map(|ctx| ctx.triggers.stop_pulse() );
    } else {
      unsafe { OVERFLOWS += 1; }
    }
  }
}