use core::sync::atomic::{AtomicBool, Ordering};
use hd44780_driver::{Cursor, CursorBlink, DisplayMode, HD44780};
use stm32f1xx_hal::{delay::Delay, gpio};
use crate::peripherals::{DisplayPins};
use crate::statemachine::{State};

use crate::debug;

static UPDATE_TIME_ARRIVED: AtomicBool = AtomicBool::new(false);

const DISPLAY_UPDATE_OVERFLOWS: u8 = 25;

type Hd44780Display = HD44780<hd44780_driver::bus::FourBitBus<
  gpio::gpioa::PA8<gpio::Output<gpio::PushPull>>, 
  gpio::gpiob::PB15<gpio::Output<gpio::PushPull>>, 
  gpio::gpiob::PB14<gpio::Output<gpio::PushPull>>, 
  gpio::gpiob::PB10<gpio::Output<gpio::PushPull>>, 
  gpio::gpioa::PA4<gpio::Output<gpio::PushPull>>, 
  gpio::gpioa::PA5<gpio::Output<gpio::PushPull>>
>>;

pub struct Display {
  lcd: Hd44780Display,
  updated: bool,
  bpm: u16,
  delay: Delay
}

impl Display {

  pub fn new(mut pins: DisplayPins) -> Display {

    let lcd = HD44780::new_4bit(pins.rs, pins.enable, pins.d4, pins.d5, pins.d6, pins.d7, &mut pins.delay).unwrap();

    return Display {
      lcd: lcd,
      updated: true,
      bpm: 0,
      delay: pins.delay
    };
  }

  pub fn init(&mut self) {
    debug!("init display");
    self.lcd.reset(&mut self.delay).ok();
    self.lcd.clear(&mut self.delay).ok();
    self.lcd.set_display_mode(
      DisplayMode {
          display: hd44780_driver::Display::On,
          cursor_visibility: Cursor::Visible,
          cursor_blink: CursorBlink::On,
      }, &mut self.delay).ok();
    self.lcd.write_str("test",&mut  self.delay).ok();
  }

  pub fn update(&mut self, state: &State) {
    self.bpm = state.bpm;
    self.updated = true;
  }

  pub fn render(&mut self) {
    let update_time_arrived = UPDATE_TIME_ARRIVED.fetch_and(false, Ordering::Relaxed);
    if self.updated && update_time_arrived {
      debug!("render display");
      self.lcd.reset(&mut self.delay).ok();
      self.lcd.write_str("120 BPM",&mut  self.delay).ok();
      self.updated = false; 
    } 
  }

  pub unsafe fn on_timer_tick() {
    static mut OVERFLOWS : u8 = 0;

    if OVERFLOWS > DISPLAY_UPDATE_OVERFLOWS {
      UPDATE_TIME_ARRIVED.store(true, Ordering::Relaxed);
      OVERFLOWS = 0;
    } else {
      OVERFLOWS += 1;
    }
  }
}

