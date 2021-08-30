/*
 * Wrapper for Serial Interfaces
 */

use core::convert::Infallible;
use stm32f1xx_hal::{
  prelude::*, 
};

use crate::peripherals::{Usart1Serial};

pub struct SerialWriter {
  serial1: Usart1Serial
}

impl SerialWriter {
  pub fn new(serial1: Usart1Serial) -> SerialWriter {
    return SerialWriter {
      serial1: serial1
    }
  }

  pub fn write(&mut self, byte: u8) -> nb::Result<(), Infallible> {
    return self.serial1.write(byte);
  }

  pub fn write_str(&mut self, str: &str) -> nb::Result<(), Infallible> {
    let _ = str.bytes().map(|c| nb::block!(self.write(c))).last();
    Ok(())
  }
}