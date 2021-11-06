use crate::peripherals::{I2c1Port};

use crate::debug;

use cortex_m::prelude::{
  _embedded_hal_blocking_i2c_Write, 
  _embedded_hal_blocking_i2c_Read, 
  _embedded_hal_blocking_i2c_WriteRead
};

pub struct Eeprom {
  i2c: I2c1Port
}

const EEPROM_ADDRESS : u8 = 0b1010_0000 >> 1;

impl Eeprom {
  pub fn new(i2c: I2c1Port) -> Eeprom {
    return Eeprom {
      i2c: i2c
    }
  }

  pub fn write_byte(&mut self, mem_addr: u16, data: u8) -> Result<(), nb::Error<stm32f1xx_hal::i2c::Error>> {
    let address_msb = (mem_addr >> 8) as u8;
    let address_lsb = (mem_addr & 0xFF) as u8;
    return self.i2c.write(EEPROM_ADDRESS, &[address_msb, address_lsb, data]);
  }

  pub fn read_byte(&mut self, mem_addr: u16) -> Result<u8, nb::Error<stm32f1xx_hal::i2c::Error>> {
    let mut buffer = [0];
    let address_msb = (mem_addr >> 8) as u8;
    let address_lsb = (mem_addr & 0xFF) as u8;
    self.i2c.write_read(EEPROM_ADDRESS, &[address_msb, address_lsb], &mut buffer)?;
    return Ok(buffer[0]);
  }

  pub fn write_u16(&mut self, mem_addr: u16, data: u16) -> Result<(), nb::Error<stm32f1xx_hal::i2c::Error>> {
    let address_msb = (mem_addr >> 8) as u8;
    let address_lsb = (mem_addr & 0xFF) as u8;
    return self.i2c.write(EEPROM_ADDRESS, &[address_msb, address_lsb, (data >> 8) as u8, (data & 0xFF) as u8]);
  }

  pub fn read_u16(&mut self, mem_addr: u16) -> Result<u16, nb::Error<stm32f1xx_hal::i2c::Error>> {
    let mut buffer = [0,0];
    let address_msb = (mem_addr >> 8) as u8;
    let address_lsb = (mem_addr & 0xFF) as u8;
    self.i2c.write_read(EEPROM_ADDRESS, &[address_msb, address_lsb], &mut buffer)?;
    return Ok((buffer[0] as u16) << 8 | (buffer[1] as u16));
  }
}