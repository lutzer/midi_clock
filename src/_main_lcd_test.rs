#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
  pac, 
  prelude::*,
  gpio,
  afio,
  serial::{Serial, Config},
  delay::{Delay}
};
use hd44780_driver::{Cursor, CursorBlink, Display, DisplayMode, HD44780};

use embedded_hal::digital::v2::{OutputPin, InputPin};

pub struct DisplayPins {
  pub rs: gpio::gpioa::PA8<gpio::Output<gpio::PushPull>>,
  pub en: gpio::gpiob::PB15<gpio::Output<gpio::PushPull>>,
  pub d4: gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>,
  pub d5: gpio::gpiob::PB10<gpio::Output<gpio::PushPull>>,
  pub d6: gpio::gpioa::PA4<gpio::Output<gpio::PushPull>>,
  pub d7: gpio::gpioa::PA5<gpio::Output<gpio::PushPull>>,
  pub delay: Delay
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let mut flash = dp.FLASH.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let delay = Delay::new(cp.SYST, clocks);

    let mut apb2 = rcc.apb2;
    let mut gpioa = dp.GPIOA.split(&mut apb2);
    let mut gpiob = dp.GPIOB.split(&mut apb2);
    let mut gpioc = dp.GPIOC.split(&mut apb2);

    let mut rs = gpioa.pa8.into_push_pull_output(&mut gpioa.crh);
    let mut en = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);
    let b4 = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);
    let b5 = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
    let b6 = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let b7 = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    rs.set_low().ok();
    en.set_low().ok();

    let mut display = DisplayPins {
      rs: rs,
      en: en,
      d4: b4,
      d5: b5,
      d6: b6,
      d7: b7,
      delay: delay
    };


    /*
      This is the procedure how to use 4-bit mode to display character ‘A’:

      Send command: 0b0010 xxxx (Set 4-bit mode).
      Send command: 0b0010 xxxx (high nibble of function set).
      Send command: 0b1000 xxxx (low nibble of function set).
      Send command: 0b0000 xxxx (high nibble of display on, underline on, and blinking on).
      Send command: 0b1111 xxxx (low nibble of display on, underline on, and blinking on).
      Send data: 0b0100 xxxx (high nibble of  ‘A’).
      Send data: 0b0001 xxxx (low nibble of ‘A’).
    */


    display.delay.delay_ms(200u16);

    // init display
    write_4bits(&mut display, 0x3);
    display.delay.delay_ms(5u8);
    write_4bits(&mut display, 0x3);
    display.delay.delay_ms(1u8);
    write_4bits(&mut display, 0x3);
    write_4bits(&mut display, 0x2);
  
    //function set, and font size DL = 0, N = 1, F = 0
    write_command(&mut display, 0x28, false);
    
    // display clear
    write_command(&mut display, 0x01, false);
    //entry mode id=1, S = 0
    write_command(&mut display, 0x0F, false);

    //display on, D=0, C=0, B=0
    write_command(&mut display, 0x06, false);

    //write data
    write_command(&mut display, 0x51, true);
    write_command(&mut display, 0x52, true);
    write_command(&mut display, 0x53, true);
    write_command(&mut display, 0x54, true);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
      led.set_high().ok();
      display.delay.delay_ms(1_000_u16);
      led.set_low().ok();
      display.delay.delay_ms(1_000_u16);
    }
}

fn pull_enable_high(disp: &mut DisplayPins) {
  // pulse en
  disp.en.set_low().ok();
  disp.delay.delay_us(40u8);
  disp.en.set_high().ok();
  disp.delay.delay_us(40u8);
  disp.en.set_low().ok();
}

fn write_command(disp: &mut DisplayPins, cmd: u8, is_data: bool) {

  // rs high on data send
  if is_data {
    disp.rs.set_high().ok();
  } else {
    disp.rs.set_low().ok();
  }

  disp.delay.delay_ms(1u8);

  // send higher nibble
  write_4bits(disp, (cmd & 0xF0) >> 4);

  //send lower nibble
  write_4bits(disp, cmd & 0x0F);

}

fn write_4bits(disp: &mut DisplayPins, cmd: u8) {
  if (cmd & 0b0001) > 0 {
    disp.d4.set_high().ok();
  } else {
    disp.d4.set_low().ok();
  }

  if (cmd & 0b0010) > 0 {
    disp.d5.set_high().ok();
  } else {
    disp.d5.set_low().ok();
  }

  if (cmd & 0b0100) > 0 {
    disp.d6.set_high().ok();
  } else {
    disp.d6.set_low().ok();
  }

  if (cmd & 0b1000) > 0 {
    disp.d7.set_high().ok();
  } else {
    disp.d7.set_low().ok();
  } 

  // pulse en
  pull_enable_high(disp);
}