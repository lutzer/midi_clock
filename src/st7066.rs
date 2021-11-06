use embedded_hal::digital::v2::{OutputPin};
use stm32f1xx_hal::{
  delay::{Delay},
  prelude::*
};

struct ST7066Bus<
  RS: OutputPin,
  EN: OutputPin,
  D4: OutputPin,
  D5: OutputPin,
  D6: OutputPin,
  D7: OutputPin
> {
  rs: RS,
  en: EN,
  d4: D4,
  d5: D5,
  d6: D6,
  d7: D7,
}

pub struct ST7066<
  RS: OutputPin,
  EN: OutputPin,
  D4: OutputPin,
  D5: OutputPin,
  D6: OutputPin,
  D7: OutputPin
> {
  bus: ST7066Bus<RS,EN,D4,D5,D6,D7>,
  delay: Delay,
  data_mode: bool
}

impl<
  RS: OutputPin,
  EN: OutputPin,
  D4: OutputPin,
  D5: OutputPin,
  D6: OutputPin,
  D7: OutputPin
> ST7066<RS,EN,D4,D5,D6,D7> {


  pub fn new(rs: RS, en: EN, d4: D4, d5: D5, d6: D6, d7: D7, delay: Delay) -> ST7066<RS,EN,D4,D5,D6,D7> {

    let mut bus = ST7066Bus {
      rs: rs,
      en: en,
      d4: d4,
      d5: d5,
      d6: d6,
      d7: d7
    };

    // initialize control pins
    bus.rs.set_low().ok();
    bus.en.set_low().ok();

    return ST7066 {
      bus: bus,
      delay: delay,
      data_mode: false,
    }
  }

  // sends initializing commands for 4bit operation and 2 row display
  pub fn init(&mut self) {
    self.delay.delay_ms(1000u16);

    // init display
    self.write_4bits(0x3);
    self.delay.delay_ms(5u8);

    self.write_4bits(0x3);
    self.delay.delay_ms(5u8);

    self.write_4bits(0x3);
    self.write_4bits(0x2);

    self.delay.delay_us(100u8);
  
    //function set 0b001D_NFxx and font size D = 0 (4bit), N = 1 (2rows), F = 0 (font1)
    self.write_command(0b0010_1000, false);

    // display on, 0b0000_1DCB D=1 (on), C=1 (cursor), B=1 (blink)
    self.write_command(0b0000_1100, false);

    //entry mode, 0b0000_01IS, I=1 (increment), S = 1 (shift)
    self.write_command(0b0000_0110, false);

    self.clear();


    self.write_char('i' as u8);
    self.write_char('n' as u8);
    self.write_char('i' as u8);
    self.write_char('t' as u8);
  }

  pub fn clear(&mut self) {
    self.write_command(0x01, false);
    self.delay.delay_ms(2u8);
  }

  pub fn return_home(&mut self) {
    self.write_command(0x02, false);
    self.delay.delay_ms(2u8);
  }

  pub fn write_char(&mut self, c: u8) {
    self.write_command(c, true);
  }

  pub fn write_str(&mut self, text: &str) {
    text.bytes().map(|c| self.write_char(c as u8)).last();
  }

  fn set_data_write_mode(&mut self, enable: bool) {
    if enable == self.data_mode { return }

    if enable {
      self.bus.rs.set_high().ok();
    } else {
      self.bus.rs.set_low().ok();
    }
    self.data_mode = enable;
  }
  
  fn write_command(&mut self, cmd: u8, is_data: bool) {
  
    self.set_data_write_mode(is_data);
  
    // send higher nibble
    self.write_4bits((cmd & 0xF0) >> 4);
  
    //send lower nibble
    self.write_4bits(cmd & 0x0F);
  
  }
  
  fn write_4bits(&mut self, cmd: u8) {
    if (cmd & 0b0001) > 0 {
      self.bus.d4.set_high().ok();
    } else {
      self.bus.d4.set_low().ok();
    }
  
    if (cmd & 0b0010) > 0 {
      self.bus.d5.set_high().ok();
    } else {
      self.bus.d5.set_low().ok();
    }
  
    if (cmd & 0b0100) > 0 {
      self.bus.d6.set_high().ok();
    } else {
      self.bus.d6.set_low().ok();
    }
  
    if (cmd & 0b1000) > 0 {
      self.bus.d7.set_high().ok();
    } else {
      self.bus.d7.set_low().ok();
    } 
  
    // pulse en
    self.bus.en.set_low().ok();
    self.delay.delay_us(1u8);
    self.bus.en.set_high().ok();
    self.delay.delay_us(1u8);
    self.bus.en.set_low().ok();

    self.delay.delay_us(40u8);
  }

}