use crate::peripherals::{Trigger1Gpio, Trigger2Gpio, Trigger3Gpio, Trigger4Gpio};

pub use embedded_hal::digital::v2::{OutputPin, InputPin};
use cortex_m::interrupt;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::{CONTEXT};

#[derive(Copy, Clone)]
pub enum TriggerMask {
  Trigger1 = 0b0001,
  Trigger2 = 0b0010,
  Trigger3 = 0b0100,
  Trigger4 = 0b1000
}

// stop pulse after 50ms
const TIMER_OVERFLOW_COUNT: u8 = 20;

static TRIGGERS_STARTED: AtomicU8 = AtomicU8::new(0);

pub struct Triggers {
  trigger1: Trigger1Gpio, 
  trigger2: Trigger2Gpio, 
  trigger3: Trigger3Gpio, 
  trigger4: Trigger4Gpio
}

impl Triggers {
  pub fn new(
    trigger1: Trigger1Gpio, 
    trigger2: Trigger2Gpio, 
    trigger3: Trigger3Gpio, 
    trigger4: Trigger4Gpio
  ) -> Triggers {
    return Triggers { 
      trigger1: trigger1, 
      trigger2: trigger2,
      trigger3: trigger3,
      trigger4: trigger4 
    }
  }

  pub fn fire(&mut self, triggers: u8) {
    self.start_pulse(triggers);
  }

  fn start_pulse(&mut self, triggers: u8) {
    TRIGGERS_STARTED.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      if (triggers & TriggerMask::Trigger1 as u8) > 0 {
        self.trigger1.set_high().ok();
      }
      if (triggers & TriggerMask::Trigger2 as u8) > 0 {
        self.trigger2.set_high().ok();
      }
      if (triggers & TriggerMask::Trigger3 as u8) > 0 {
        self.trigger3.set_high().ok();
      }
      if (triggers & TriggerMask::Trigger4 as u8) > 0 {
        self.trigger4.set_high().ok();
      }
      return Some(x | triggers);
    }).ok();
  }

  fn stop_pulse(&mut self, triggers: u8) {
    TRIGGERS_STARTED.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      if (triggers & TriggerMask::Trigger1 as u8) > 0 {
        self.trigger1.set_low().ok();
      }
      if (triggers & TriggerMask::Trigger2 as u8) > 0 {
        self.trigger2.set_low().ok();
      }
      if (triggers & TriggerMask::Trigger3 as u8) > 0 {
        self.trigger3.set_low().ok();
      }
      if (triggers & TriggerMask::Trigger4 as u8) > 0 {
        self.trigger4.set_low().ok();
      }
      return Some(x & !(triggers));
    }).ok();
  }

  pub fn on_timer_tick() {
    static mut OVERFLOWS1: u8 = 0;
    static mut OVERFLOWS2: u8 = 0;
    static mut OVERFLOWS3: u8 = 0;
    static mut OVERFLOWS4: u8 = 0;

    let triggers_started = TRIGGERS_STARTED.load(Ordering:: Relaxed);
    let mut triggers_ended: u8 = 0;

    // resets overflow is not started, else counts until TIMER_OVERFLOW_COUNT
    pub fn check_trigger(trigger_started: u8, overflows: &mut u8) -> u8  {
      if (trigger_started) > 0 {
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
      triggers_ended |= check_trigger(triggers_started & TriggerMask::Trigger1 as u8, &mut OVERFLOWS1);
      triggers_ended |= check_trigger(triggers_started & TriggerMask::Trigger2 as u8, &mut OVERFLOWS2);
      triggers_ended |= check_trigger(triggers_started & TriggerMask::Trigger3 as u8, &mut OVERFLOWS3);
      triggers_ended |= check_trigger(triggers_started & TriggerMask::Trigger4 as u8, &mut OVERFLOWS4);
    }

    if triggers_ended > 0 {
      interrupt::free(|cs| {
        let mut context = CONTEXT.borrow(cs).borrow_mut();
        context.as_mut().map(|ctx| ctx.triggers.stop_pulse(triggers_ended) );
      })
    }
  }
}