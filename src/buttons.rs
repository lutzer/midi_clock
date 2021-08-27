use crate::peripherals::*;

pub struct Buttons {
  pub button1: Button1Gpio
}

impl Buttons {
  pub fn read(&self) -> bool {
    let pressed = self.button1.is_low().unwrap();
    return pressed;
  }
}