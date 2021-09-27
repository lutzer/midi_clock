use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use cortex_m::interrupt::{ CriticalSection };
use core::mem::MaybeUninit;

pub struct Clock {
  bpm: u16
}

use crate::timers::{Timer3};

type ClockTickHandler = fn(u8, bool, &CriticalSection);

static mut CLOCK_TICK_HANDLER: MaybeUninit<ClockTickHandler> = MaybeUninit::uninit();

static CLOCK_DIVISIONS: AtomicU32 = AtomicU32::new(0);

const MAX_DIVISION: u16 = 24;

impl Clock {
  pub fn new(bpm: u16, running: bool) -> Clock {
    // let mut clock = unsafe { &mut *CLOCK.as_mut_ptr() };
    let mut clock = Clock{ bpm: 1 };
    clock.set_bpm(bpm);
    clock.set_running(running);
    clock.set_divisions([1,1,1]);
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
    let frequency_in_hertz : u32 = (self.bpm as u32) * (MAX_DIVISION as u32) / 60;
    Timer3::set_frequency(frequency_in_hertz);
  }

  pub fn set_running(&mut self, running: bool) {
    Timer3::set_running(running);
  }


  pub fn on_tick<'a>(&self, cb: ClockTickHandler) {
    unsafe { CLOCK_TICK_HANDLER.write(cb); }
  }

  pub fn on_timer_tick(cs : &CriticalSection) {
    static mut OVERFLOWS : u16 = 0;

    unsafe {
      
      (*CLOCK_TICK_HANDLER.as_mut_ptr())(0, OVERFLOWS == 0, cs); 
      OVERFLOWS = OVERFLOWS % (MAX_DIVISION + 1);
    }
  }
}