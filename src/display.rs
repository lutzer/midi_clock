use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::BinaryColor,
  prelude::*,
  text::{Baseline, Text}
};

use cortex_m::interrupt::{CriticalSection};

use ssd1306::{
  prelude::*, 
  I2CDisplayInterface,
  Ssd1306,
  mode::{BufferedGraphicsMode}
};

use crate::peripherals::{DisplayI2C};
use crate::statemachine::{State};
use crate::utils::*;
use core::sync::atomic::{AtomicBool, Ordering};

const DISPLAY_UPDATE_OVERFLOWS: u8 = 2;

pub type Ssd1306Display = ssd1306::Ssd1306<I2CInterface<DisplayI2C>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>;

static UPDATE_TIME_ARRIVED: AtomicBool = AtomicBool::new(false);

struct DisplayStyles<'a> {
  text: MonoTextStyle<'a, BinaryColor>
}

pub struct Display<'a> {
  display: Ssd1306Display,
  updated: bool,
  styles: DisplayStyles<'a>
}

impl<'a> Display<'a> {


  pub fn new(i2c: DisplayI2C) -> Display<'a> {
    let interface = I2CDisplayInterface::new(i2c);
    let display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_buffered_graphics_mode();

    let styles = DisplayStyles {
      text: MonoTextStyle::new(&FONT_10X20, BinaryColor::On)
    };

    return Display {
      display: display,
      updated: true,
      styles: styles
    }
  }

  pub fn init(&mut self) {
    self.display.init().unwrap();
    self.display.clear();
    self.display.flush().unwrap();
  }

  pub fn update(&mut self, state: &State) {
    self.display.clear();

    let bpm = u16_to_string(state.bpm);
    Text::with_baseline(bpm, Point::new(50, 32), self.styles.text, Baseline::Middle)
      .draw(&mut self.display)
      .unwrap();
    
      self.updated = true;
  }

  pub fn on_update(&mut self) -> Option<()> {
    let update_time_arrived = UPDATE_TIME_ARRIVED.fetch_and(false, Ordering::Relaxed);
    return if self.updated && update_time_arrived {
      self.updated = false; 
      Some(()) 
    } else { None }
  }

  pub fn flush(&mut self, _: &CriticalSection) {
    self.display.flush().unwrap();
  }

  pub fn on_timer_tick(_: &CriticalSection) {
    static mut OVERFLOWS : u8 = 0;

    unsafe {
      if OVERFLOWS > DISPLAY_UPDATE_OVERFLOWS {
        UPDATE_TIME_ARRIVED.store(true, Ordering::Relaxed);
        OVERFLOWS = 0;
      } else {
        OVERFLOWS += 1;
      }
    }
  }
}

