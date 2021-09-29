use crate::peripherals::*;

use core::sync::atomic::{AtomicU16, Ordering};
use cortex_m::interrupt::{CriticalSection};

// 2 bits per button. can count to 4, can handle 8 buttons
static BUTTON_DEBOUNCE_COUNTERS: AtomicU16 = AtomicU16::new(0);

pub const BUTTON1_MASK : u8 = 0b00000001;
pub const BUTTON2_MASK : u8 = 0b00000010;
// pub const BUTTON3_MASK : u8 = 0b00000100;
pub const BUTTON4_MASK : u8 = 0b00001000;

const TIMER_OVERFLOW_COUNT: u8 = 20;

pub struct Buttons {
  button1: Button1Gpio,
  button2: Button2Gpio,
  button3: Button3Gpio,
  button4: Button4Gpio
}

// reads and debounces buttons.
impl Buttons {
  pub fn new(
    button1: Button1Gpio, 
    button2: Button2Gpio, 
    button3: Button3Gpio, 
    button4: Button4Gpio
  ) -> Buttons {
    return Buttons {
      button1: button1,
      button2: button2,
      button3: button3,
      button4: button4
    }
  }

  pub fn on_change(&self) -> Option<(u8,u8)>  {
    static mut BUTTON_STATES: u8 = 0;

    // read input pins
    let button_readings : u8 = 
      self.button1.is_low().unwrap() as u8
      | (self.button2.is_low().unwrap() as u8) << 1
      | (self.button3.is_low().unwrap() as u8) << 2
      | (self.button4.is_low().unwrap() as u8) << 3;

    // button state was changed
    let changes = unsafe { (BUTTON_STATES ^ button_readings) as u16 };
    if changes > 0  {
      // read value from debounce counter
      let counts = BUTTON_DEBOUNCE_COUNTERS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
        let low = x & 0xFF;
        let high = x >> 8;

        // set debounce counter to zero on the changed bits
        return Some((low & !changes) | ((high & !changes) << 8));
      }).unwrap();

      let low = counts & 0xFF;
      let high = counts >> 8;
      
      unsafe { BUTTON_STATES = button_readings; }
        // check if both bits are 1
      if (changes & low & high) > 0 {
        return Some((changes as u8, button_readings as u8));
      }
    }
    return None;
  }

  pub fn on_timer_tick(_: &CriticalSection) {
    static mut OVERFLOWS : u8 = 0;

    if unsafe { OVERFLOWS > TIMER_OVERFLOW_COUNT } {
      BUTTON_DEBOUNCE_COUNTERS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
        let low = x & 0xFF;
        let high = x >> 8;

        // bitwise vertical increment until count to 4 = 0b11 in each column
        let increment : u16 = (x | high) | ((!(low | high) | high) << 8);
        return Some(increment)
      }).ok();
      unsafe { OVERFLOWS = 0; }
    } else {
      unsafe { OVERFLOWS += 1; }
    }
  }
}