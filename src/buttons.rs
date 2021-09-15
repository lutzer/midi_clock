use crate::peripherals::*;

use core::sync::atomic::{AtomicU16, Ordering};

// 2 bits per button. can count to 4, can handle 8 buttons
static BUTTON_DEBOUNCE_COUNTERS: AtomicU16 = AtomicU16::new(0);

type ButtonHandler = fn(u8, u8);

pub struct Buttons {
  button1: Button1Gpio,
  button2: Button2Gpio,
  button3: Button3Gpio,
  button_handler: Option<ButtonHandler>
}

impl Buttons {
  pub fn new(
    button1: Button1Gpio, 
    button2: Button2Gpio, 
    button3: Button3Gpio, 
    handler: ButtonHandler
  ) -> Buttons {
    return Buttons {
      button1: button1,
      button2: button2,
      button3: button3,
      button_handler: Some(handler)
    }
  }

  pub fn update(&self) {
    static mut BUTTON_STATES: u8 = 0;

    // read input pins
    let button_readings : u8 = 
      self.button1.is_low().unwrap() as u8
      | (self.button2.is_low().unwrap() as u8) << 1
      | (self.button3.is_low().unwrap() as u8) << 2;

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
        self.button_handler.map(|f| f(changes as u8, button_readings as u8));
      }
    }

  }

  pub fn on_tick() {
    BUTTON_DEBOUNCE_COUNTERS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
      let low = x & 0xFF;
      let high = x >> 8;

      // bitwise vertical increment until count to 4 = 0b11 in each column
      let increment : u16 = (x | high) | ((!(low | high) | high) << 8);
      return Some(increment)
    }).ok();
  }
}