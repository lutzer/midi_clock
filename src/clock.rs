use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::interrupt::{ CriticalSection };
use cortex_m::interrupt;

use crate::utils::{CSCell};

pub struct Clock {
  bpm: u16
}

use crate::timers::{Timer2};

type ClockTickHandler = fn(u8, bool, &CriticalSection);

const BIG_STEP_NUMBER: u16 = 24;

static CLOCK_TICK_HANDLER: CSCell<Option<ClockTickHandler>> = CSCell::new(None);
static CLOCK_DIVISIONS: AtomicU32 = AtomicU32::new(0);

impl Clock {
  pub fn new(bpm: u16, running: bool) -> Clock {
    let mut clock = Clock{ bpm: 1 };
    clock.set_bpm(bpm);
    clock.set_running(false);
    clock.set_running(running);
    clock.set_divisions([1,1,1]);

    Timer2::set_handler(Clock::on_timer_tick);
    return clock;
  }

  pub fn set_divisions(&mut self, divisions: [u8;3]) {
    CLOCK_DIVISIONS.store( 
      divisions[0] as u32 & 
      (divisions[1] as u32) << 8 &
      (divisions[2] as u32) << 16
      ,Ordering::Relaxed)
  }

  pub fn set_bpm(&mut self, bpm: u16) {
    self.bpm = bpm;
    // sends 24 ticks for every quarternote
    let intervall_in_us : u32 = 60 * 1000 * 1000 / ((self.bpm as u32) * 24);
    Timer2::set_interval(intervall_in_us);
  }

  pub fn set_running(&mut self, running: bool) {
    static mut PREV: bool = true;
    if unsafe { PREV != running } {
      Timer2::set_running(running);
    }
    unsafe { PREV = running }
  }


  pub fn on_tick<'a>(&self, cb: ClockTickHandler) {
    interrupt::free( |cs| CLOCK_TICK_HANDLER.set(Some(cb), cs) );
  }

  pub fn on_timer_tick(cs : &CriticalSection) {
    static mut OVERFLOWS : u16 = 0;

    let divisions = CLOCK_DIVISIONS.load(Ordering::Relaxed);

    // for i in [0..3] {

    // }

    unsafe {
      CLOCK_TICK_HANDLER.get(cs).map(|f| f(0, OVERFLOWS % 24 == 0, cs) ); 
      OVERFLOWS += 1; // send a big step every 24 steps
    }
  }
}