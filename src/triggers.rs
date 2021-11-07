use crate::peripherals::{Trigger1Gpio, Trigger2Gpio, Trigger3Gpio, Trigger4Gpio};

use embedded_hal::digital::v2::{OutputPin};
use cortex_m::interrupt;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::{CONTEXT};

// bitmasks for triggers
pub const TRIGGER1_MASK : u8 = 0b00000001;
pub const TRIGGER2_MASK : u8 = 0b00000010;
pub const TRIGGER3_MASK : u8 = 0b00000100;
pub const TRIGGER4_MASK : u8 = 0b00001000;

// stop pulse after 5ms
const TIMER_OVERFLOW_COUNT: u8 = 5;

static TRIGGERS_STARTED: AtomicU8 = AtomicU8::new(0);

pub struct Triggers {
  trigger1: Trigger1Gpio, // midi out1+2
  trigger2: Trigger2Gpio, // midi out3+4
  trigger3: Trigger3Gpio, // trigger
  trigger4: Trigger4Gpio  // reset trigger
}

impl Triggers  {
  pub fn new(
    trigger1: Trigger1Gpio, 
    trigger2: Trigger2Gpio, 
    trigger3: Trigger3Gpio, 
    trigger4: Trigger4Gpio
  ) -> Triggers  {
    return Triggers { 
      trigger1: trigger1, 
      trigger2: trigger2,
      trigger3: trigger3,
      trigger4: trigger4,
    }
  }

  pub fn fire(&mut self, triggers: u8) {
    self.start_pulse(triggers);
  }

  fn start_pulse(&mut self, triggers: u8) {
    TRIGGERS_STARTED.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      if (triggers & TRIGGER1_MASK) > 0 {
        self.trigger1.set_high().ok();
      }
      if (triggers & TRIGGER2_MASK) > 0 {
        self.trigger2.set_high().ok();
      }
      if (triggers & TRIGGER3_MASK) > 0 {
        self.trigger3.set_high().ok();
      }
      if (triggers & TRIGGER4_MASK) > 0 {
        self.trigger4.set_high().ok();
      }
      return Some(x | triggers);
    }).ok();
  }

  fn stop_pulse(&mut self, triggers: u8) {
    TRIGGERS_STARTED.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      if (triggers & TRIGGER1_MASK) > 0 {
        self.trigger1.set_low().ok();
      }
      if (triggers & TRIGGER2_MASK) > 0 {
        self.trigger2.set_low().ok();
      }
      if (triggers & TRIGGER3_MASK) > 0 {
        self.trigger3.set_low().ok();
      }
      if (triggers & TRIGGER4_MASK) > 0 {
        self.trigger4.set_low().ok();
      }
      return Some(x & !(triggers));
    }).ok();
  }

  pub fn on_timer_tick() {
    static mut OVERFLOWS: [u8;4] = [0,0,0,0];

    let triggers_started = TRIGGERS_STARTED.load(Ordering:: Relaxed);
    let mut triggers_ended: u8 = 0;

    // resets overflow is not started, else counts until TIMER_OVERFLOW_COUNT
    pub fn check_trigger(trigger_started: u8, overflows: &mut u8) -> u8  {
      if trigger_started > 0 {
        if *overflows > TIMER_OVERFLOW_COUNT {
          return trigger_started;
        }
        *overflows += 1;
      } else {
        *overflows = 0;
      }
      return 0;
    }

    unsafe {
      triggers_ended |= check_trigger(triggers_started & TRIGGER1_MASK, &mut OVERFLOWS[0]);
      triggers_ended |= check_trigger(triggers_started & TRIGGER2_MASK, &mut OVERFLOWS[1]);
      triggers_ended |= check_trigger(triggers_started & TRIGGER3_MASK, &mut OVERFLOWS[2]);
      triggers_ended |= check_trigger(triggers_started & TRIGGER4_MASK, &mut OVERFLOWS[3]);
    }

    if triggers_ended > 0 {
      interrupt::free(|cs| {
        let mut context = CONTEXT.borrow(cs).borrow_mut();
        context.as_mut().map(|ctx| ctx.triggers.stop_pulse(triggers_ended) );
      })
    }
  }
}