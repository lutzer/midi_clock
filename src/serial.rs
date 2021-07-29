use core::convert::Infallible;
use stm32f1xx_hal::{
  prelude::*, 
};

use crate::peripherals::{Usart1Serial};

pub struct SerialWriter {
  pub serial: Usart1Serial
}

impl SerialWriter {
  pub fn write(&mut self, byte: u8) -> nb::Result<(), Infallible> {
    return self.serial.write(byte);
  }

  pub fn write_str(&mut self, str: &str) -> nb::Result<(), Infallible> {
    let _ = str.bytes().map(|c| nb::block!(self.write(c))).last();
    Ok(())
  }
}