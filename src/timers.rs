
use stm32f1xx_hal::{
  pac::{interrupt, Interrupt, TIM2},
  prelude::*,
  timer::{Event, Timer, CountDownTimer},
};

use cortex_m::interrupt::{CriticalSection, Mutex};
use core::cell::{ UnsafeCell, RefCell};

use crate::{COUNTER};
use crate::*;

use core::sync::atomic::{AtomicU16, Ordering};

type TimerHandler = fn();

const MAX_HANDLERS: usize = 5;

struct CSTimerHandler( UnsafeCell<[Option<TimerHandler>; MAX_HANDLERS]> );

impl CSTimerHandler {
  fn set(&self, index: usize, handler: TimerHandler, _cs: &CriticalSection ) {
    unsafe { 
      let mut handlers = *self.0.get();
      handlers[index] = Some(handler);
      debug!("add handler");
     }
  }
  pub fn execute(&self, _cs: &CriticalSection) {
    unsafe {
      let handlers = *self.0.get();
      for i in 0..MAX_HANDLERS {
        debug!("what?");
        if handlers[i].is_some() {
          debug!("is some");
        }
        handlers[i].map(|f| { f(); debug!("handlers"); } );
      }
    }
  }
}

unsafe impl Sync for CSTimerHandler {}

static TIMER_HANDLERS: CSTimerHandler = CSTimerHandler(UnsafeCell::new([None; MAX_HANDLERS]));

static G_TIM2: Mutex<RefCell<Option<CountDownTimer<TIM2>>>> = Mutex::new(RefCell::new(None));

pub struct Timer2;

impl Timer2  {
  pub fn init(tim2: stm32f1xx_hal::stm32::TIM2, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {
    // Set up a timer expiring after 1s
    let mut timer = Timer::tim2(tim2, &clocks, apb1).start_count_down(1.hz());

    // Generate an interrupt when the timer expires
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| G_TIM2.borrow(cs).replace(Some(timer)));

    cortex_m::peripheral::NVIC::unpend(Interrupt::TIM2);
    unsafe {
      cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }
  }

  pub fn add_handler(index: usize, cb: TimerHandler) {
    cortex_m::interrupt::free(|cs| {
      TIMER_HANDLERS.set(index, cb, cs);
    })
  }
}

#[interrupt]
fn TIM2() {
  // COUNTER.fetch_add(1, Ordering::Relaxed);
  //   let tim = G_TIM2.get_or_insert_with(|| {
  //     cortex_m::interrupt::free(|cs| {
  //         // Move LED pin here, leaving a None in its place
  //         G_TIM2.borrow(cs).replace(None).unwrap()
  //     })
  // });
  cortex_m::interrupt::free(|cs| {
    TIMER_HANDLERS.execute(cs);
    let mut tim2 = G_TIM2.borrow(cs).borrow_mut();
    tim2.as_mut().unwrap().clear_update_interrupt_flag();
  });
}