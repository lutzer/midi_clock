use crate::peripherals::{Led1Gpio};

pub use embedded_hal::digital::v2::{OutputPin, InputPin};
use cortex_m::interrupt::{CriticalSection};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::{CONTEXT};

// stop pulse after 100ms
const TRIGGER_OVERFLOW_COUNT: u16 = 20;

static TRIGGER_STARTED: AtomicBool = AtomicBool::new(false);

pub struct Triggers {
  led1: Led1Gpio
}

impl Triggers {
  pub fn new(led1: Led1Gpio) -> Triggers {
    return Triggers { led1: led1 }
  }

  pub fn fire(&mut self) {
    self.led1.set_low().ok();
    TRIGGER_STARTED.store(true, Ordering::Relaxed);
  }

  fn stop_pulse(&mut self) {
    self.led1.set_high().ok();
  }

  pub fn on_timer_tick(cs: &CriticalSection) {
    static mut OVERFLOWS: u16 = 0;

    if !TRIGGER_STARTED.load(Ordering:: Relaxed) {
      return;
    }

    unsafe {
      if OVERFLOWS > TRIGGER_OVERFLOW_COUNT {
        // stop trigger pulse
        let mut context = CONTEXT.borrow(cs).borrow_mut();
        context.as_mut().map(|ctx| ctx.triggers.stop_pulse() );
        OVERFLOWS = 0;
        TRIGGER_STARTED.store(false, Ordering::Relaxed);
      } else {
        OVERFLOWS += 1;
      }
    }
  }
}