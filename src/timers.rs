
use stm32f1xx_hal::{
  pac::{interrupt, Interrupt, TIM3, TIM2},
  prelude::*,
  timer::{Event, Timer, CountDownTimer},
};
use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m::interrupt::{CriticalSection, Mutex};
use core::cell::{RefCell};

use crate::utils::{CSCell};

type CSTimerHandler = unsafe fn(&CriticalSection);
type TimerHandler = fn();

const MAX_TIM2_HANDLERS: usize = 3;



static TIMER_2_HANDLER: CSCell<Option<CSTimerHandler>> = CSCell::new(None);
static G_TIM2: Mutex<RefCell<Option<CountDownTimer<TIM2>>>> = Mutex::new(RefCell::new(None));
static TIMER2_OVERFLOWS: AtomicU32 = AtomicU32::new(1);

/* Timer2 is used only to send trigger and midi tick messages to the clock */
pub struct Timer2;
impl Timer2  {
  pub fn init(tim2: TIM2, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {

    // set timer 2 to 50us
    let timer = Timer::tim2(tim2, &clocks, apb1).start_count_down(50.us());

    cortex_m::interrupt::free(|cs| G_TIM2.borrow(cs).replace(Some(timer)));

    cortex_m::peripheral::NVIC::unpend(Interrupt::TIM2);
    unsafe {
      cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }
  }

  pub fn set_running(running: bool) {
    cortex_m::interrupt::free(|cs| {
      let mut tim2 = G_TIM2.borrow(cs).borrow_mut();
      tim2.as_mut().map(|t| {
        if running {
          t.reset();
          t.listen(Event::Update);
        } else {
          t.unlisten(Event::Update);
        }
      });
      
    });
  }

  pub fn set_interval(us: u32) {
    /* mapping us to overflows for 50.us() timer
     1000000 us = 20000 - 1310 = factor 0.00131
      200000 us = 4000 - 264 = factor 0.00132
      100000 us = 2000 - 132 = factor 0.00132
       50000 us = 1000 - 67 =  factor 0.00134
       10000 us = 200 - 14 =   factor 0,00140
        5000 us = 100 - 8 =    factor 0.00160
    */
    let overflows = us/50 - (us as f64 * 0.00136) as u32;
    TIMER2_OVERFLOWS.store(overflows, Ordering::Relaxed);
  }

  // pub fn set_frequency(hz: u32) {
  //   TIMER2_OVERFLOWS.store(us/10, Ordering::Relaxed);
  // }

  pub fn set_handler(cb: CSTimerHandler) {
    cortex_m::interrupt::free(|cs| {
      TIMER_2_HANDLER.set(Some(cb), cs);
    })
  }
}

#[interrupt]
unsafe fn TIM2() {
  static mut OVERFLOWS: u32 = 0;

  cortex_m::interrupt::free(|cs| {
    // run handler and reset overflows
    if *OVERFLOWS >= TIMER2_OVERFLOWS.load(Ordering::Relaxed) {
      TIMER_2_HANDLER.get(cs).map(|f| f(cs) );
      *OVERFLOWS = 0;
    } else {
      *OVERFLOWS += 1;
    }
    // reset interrupt
    let mut tim2 = G_TIM2.borrow(cs).borrow_mut();
    tim2.as_mut().map(|t| {
      t.clear_update_interrupt_flag();
      t.reset();
    });
  });
}

static TIMER_3_HANDLERS: CSCell<[Option<TimerHandler>; MAX_TIM2_HANDLERS]> = CSCell::new([None; MAX_TIM2_HANDLERS]);
static G_TIM3: Mutex<RefCell<Option<CountDownTimer<TIM3>>>> = Mutex::new(RefCell::new(None));

/* Timer3 is used as a general purpose trigger for debouncing, pulse generation, etc */
pub struct Timer3;
impl Timer3 {
  pub fn init(tim3: TIM3, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {
    
    // set timer3 to 1khz = 1ms
    let mut timer = Timer::tim3(tim3, &clocks, apb1).start_count_down(1000.hz());
    
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| G_TIM3.borrow(cs).replace(Some(timer)));

    cortex_m::peripheral::NVIC::unpend(Interrupt::TIM3);
    unsafe {
      cortex_m::peripheral::NVIC::unmask(Interrupt::TIM3);
    }
  }

  pub fn add_handler(index: usize, cb: TimerHandler) {
    unsafe {
      let handlers = TIMER_3_HANDLERS.get_unsafe();
      handlers[index] = Some(cb);
      TIMER_3_HANDLERS.set_unsafe(*handlers);
    }
  }
}

#[interrupt]
unsafe fn TIM3() {
  let handlers = TIMER_3_HANDLERS.get_unsafe();
  for handler in handlers {
    handler.map(|f| f());
  }

  cortex_m::interrupt::free(|cs| {
    let mut tim3 = G_TIM3.borrow(cs).borrow_mut();
    tim3.as_mut().map(|t| {
      t.reset();
      t.clear_update_interrupt_flag();
    })
  });
}