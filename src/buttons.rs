use crate::peripherals::*;

use core::sync::atomic::{AtomicU8, Ordering};

const BUTTON_OVERFLOWS: u8 = 5;

static BUTTON_DEBOUNCE_COUNTER: AtomicU8 = AtomicU8::new(0);

type ButtonHandler = fn(u8, bool);


pub struct Buttons {
  button1: Button1Gpio,
  button_handler: Option<ButtonHandler>
}

impl Buttons {
  pub fn new(button1: Button1Gpio, handler: ButtonHandler) -> Buttons {
    return Buttons {
      button1: button1,
      button_handler: Some(handler)
    }
  }

  pub fn update(&self) {
    static mut BUTTON1_STATE: bool = false;

    let button1_reading = self.button1.is_low().unwrap();

    unsafe {
      // button state was changed
      if BUTTON1_STATE != button1_reading {
        // read value from debounce counter
        let val = BUTTON_DEBOUNCE_COUNTER.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| {
          return if x > BUTTON_OVERFLOWS { Some(0) } else { Some(x) }
        });
        if val.unwrap() <= BUTTON_OVERFLOWS {
          // break function
          return
        }

        BUTTON1_STATE = button1_reading;
        if BUTTON1_STATE {
          self.button_handler.map(|f| f(0,true));
        } else {
          self.button_handler.map(|f| f(0,false));
        }
      }
    }

  }

  pub fn on_tick() {
    BUTTON_DEBOUNCE_COUNTER.fetch_add(1, Ordering::Relaxed);
  }
}