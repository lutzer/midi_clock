use crate::peripherals::{Led1Gpio};

pub use embedded_hal::digital::v2::{OutputPin, InputPin};

pub struct Triggers {
  led1: Led1Gpio,
  led1_on : bool
}

impl Triggers {
  pub fn new(led1: Led1Gpio) -> Triggers {
    return Triggers { led1: led1, led1_on: false}
  }

  pub fn fire(&mut self) {
    self.led1_on = !self.led1_on;
    if self.led1_on {
      self.led1.set_low().ok();
    } else {
      self.led1.set_high().ok();
    }
  }
}