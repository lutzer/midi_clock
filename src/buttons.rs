use crate::peripherals::*;

pub struct Buttons {
  button1: Button1Gpio
}

impl Buttons {
  pub fn new(button1: Button1Gpio) -> Buttons {
    return Buttons {
      button1: button1
    }
  }

  pub fn read(&self) -> bool {
    let pressed = self.button1.is_low().unwrap();
    return pressed;
  }
}