use embedded_graphics::{
  mono_font::{ascii::FONT_10X20, MonoTextStyle},
  pixelcolor::{BinaryColor,Rgb565},
  prelude::*,
  primitives::{PrimitiveStyleBuilder, Rectangle},
  text::{Baseline, Text}
};

use stm32f1xx_hal::delay::{Delay};

use cortex_m::interrupt::{CriticalSection};


use crate::peripherals::{DisplaySpi1};
use crate::statemachine::{State};
use crate::utils::{u16_to_string};
use core::sync::atomic::{AtomicBool, Ordering};

use st7789::{Orientation, ST7789};
use display_interface_spi::SPIInterfaceNoCS;

const DISPLAY_UPDATE_OVERFLOWS: u8 = 50;

pub type St7789Display = st7789::ST7789<display_interface_spi::SPIInterfaceNoCS<stm32f1xx_hal::spi::Spi<stm32f1xx_hal::pac::SPI1, stm32f1xx_hal::spi::Spi1NoRemap, (stm32f1xx_hal::gpio::gpioa::PA5<stm32f1xx_hal::gpio::Alternate<stm32f1xx_hal::gpio::PushPull>>, stm32f1xx_hal::spi::NoMiso, stm32f1xx_hal::gpio::gpioa::PA7<stm32f1xx_hal::gpio::Alternate<stm32f1xx_hal::gpio::PushPull>>)>, stm32f1xx_hal::gpio::gpiob::PB15<stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>>>, stm32f1xx_hal::gpio::gpiob::PB14<stm32f1xx_hal::gpio::Output<stm32f1xx_hal::gpio::PushPull>>>;


static UPDATE_TIME_ARRIVED: AtomicBool = AtomicBool::new(false);

struct DisplayStyles<'a> {
  text: MonoTextStyle<'a, Rgb565>
}

pub struct Display<'a> {
  display: St7789Display,
  updated: bool,
  styles: DisplayStyles<'a>,
  delay: Delay,
  bpm: u16
}

impl<'a> Display<'a> {


  pub fn new(display_peripherals: DisplaySpi1) -> Display<'a> {
    let interface = SPIInterfaceNoCS::new(display_peripherals.spi, display_peripherals.dc);
    let display = ST7789::new(interface, display_peripherals.rst, 240, 240);

    let styles = DisplayStyles {
      text: MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE)
    };

    return Display {
      display: display,
      updated: true,
      styles: styles,
      delay: display_peripherals.delay,
      bpm: 0
    }
  }

  pub fn init(&mut self) {
    self.display.init(&mut self.delay).ok();
    self.display.set_orientation(Orientation::Landscape).unwrap();
    self.display.clear(Rgb565::BLACK).ok();
  }

  pub fn update(&mut self, state: &State) {
    self.bpm = state.bpm;
    self.updated = true;
  }

  pub fn draw(&mut self) {
    let update_time_arrived = UPDATE_TIME_ARRIVED.fetch_and(false, Ordering::Relaxed);
    if self.updated && update_time_arrived {
      let style = PrimitiveStyleBuilder::new()
      .fill_color(Rgb565::BLACK)
      .build();

      let rect = Rectangle::new(Point::new(50,22), Size::new(40,20)).into_styled(style);
      rect.draw(&mut self.display).ok();

      let bpm = u16_to_string(self.bpm);
      Text::with_baseline(bpm, Point::new(50, 32), self.styles.text, Baseline::Middle)
        .draw(&mut self.display)
        .ok();
      
        self.updated = false; 
    }
  }

  pub fn on_timer_tick() {
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

