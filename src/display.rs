use core::sync::atomic::{AtomicBool, Ordering};
use stm32f1xx_hal::{
  delay::{Delay},
  {gpio}
};

use crate::peripherals::{DisplayPins};
use crate::statemachine::{State, RunState};
use crate::utils::{u16_to_string};

use crate::debug;

use crate::st7066::ST7066;

static UPDATE_TIME_ARRIVED: AtomicBool = AtomicBool::new(false);

const DISPLAY_UPDATE_OVERFLOWS: u8 = 50;

type ST7066Display = ST7066<
  gpio::gpioa::PA8<gpio::Output<gpio::PushPull>>, 
  gpio::gpiob::PB15<gpio::Output<gpio::PushPull>>, 
  gpio::gpiob::PB11<gpio::Output<gpio::PushPull>>, 
  gpio::gpiob::PB10<gpio::Output<gpio::PushPull>>, 
  gpio::gpioa::PA4<gpio::Output<gpio::PushPull>>, 
  gpio::gpioa::PA5<gpio::Output<gpio::PushPull>>
>;

pub struct Display {
  lcd: ST7066Display,
  updated: bool,
  state: Option<State>
}

impl Display {

  pub fn new(pins: DisplayPins, delay: Delay) -> Display {

    let lcd = ST7066::new(pins.rs, pins.en, pins.d4, pins.d5, pins.d6, pins.d7, delay);

    return Display {
      lcd: lcd,
      updated: true,
      state: None
    };
  }

  pub fn init(&mut self) {
    self.lcd.init();
  }

  pub fn update(&mut self, state: &State) {
    self.state = Some(*state);
    self.updated = true;
  }

  pub fn render(&mut self) {
    let update_time_arrived = UPDATE_TIME_ARRIVED.fetch_and(false, Ordering::Relaxed);
    if self.updated && update_time_arrived {
      debug!("display render");
      let state = self.state.unwrap();
      self.lcd.clear();
      
      // write bpm
      let bpm = u16_to_string(state.bpm as u16);
      self.lcd.write_str("Bpm ");
      self.lcd.write_str(bpm);

      //write run state
      self.lcd.set_cursor((0,1));
      match state.running {
        RunState::RUNNING => self.lcd.write_str("running"),
        RunState::PAUSED => self.lcd.write_str("paused"),
        _ => self.lcd.write_str("stopped")
      }

      self.updated = false; 
    } 
  }

  pub fn print(&mut self, text: &str) {
    self.lcd.clear();
    self.lcd.write_str(text);
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

