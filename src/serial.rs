/*
 * Wrapper for Serial Interfaces
 */

use core::convert::Infallible;
use stm32f1xx_hal::{
  prelude::*, 
};

use crate::peripherals::{Usart1Serial, Usart2Serial};

pub struct SerialWriter {
  serial1: Usart1Serial,
  serial2: Usart2Serial
}

impl SerialWriter {
  pub fn new(serial1: Usart1Serial, serial2: Usart2Serial) -> SerialWriter {
    return SerialWriter {
      serial1: serial1,
      serial2: serial2
    }
  }

  pub fn write(&mut self, uart: u8, byte: u8) -> nb::Result<(), Infallible> {
    match uart {
      1 => return self.serial1.write(byte),
      2 => return self.serial2.write(byte),
      _ => return Ok(())
    }
  }

  pub fn write_str(&mut self, uart: u8, str: &str) -> nb::Result<(), Infallible> {
    let _ = str.bytes().map(|c| nb::block!(self.write(uart, c))).last();
    Ok(())
  }
}