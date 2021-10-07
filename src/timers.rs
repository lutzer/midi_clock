
use stm32f1xx_hal::{
  pac::{interrupt, Interrupt, TIM3, TIM2},
  prelude::*,
  timer::{Event, Timer, CountDownTimer},
};

use cortex_m::interrupt::{CriticalSection, Mutex};
use core::cell::{RefCell};

use crate::utils::{CSCell};

type CSTimerHandler = fn(&CriticalSection);
type TimerHandler = fn();

const MAX_TIM2_HANDLERS: usize = 3;


static TIMER_2_HANDLER: CSCell<Option<CSTimerHandler>> = CSCell::new(None);
static G_TIM2: Mutex<RefCell<Option<CountDownTimer<TIM2>>>> = Mutex::new(RefCell::new(None));

pub struct Timer2;
impl Timer2  {
  pub fn init(tim2: TIM2, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {

    let timer = Timer::tim2(tim2, &clocks, apb1).start_count_down(1.hz());

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
    cortex_m::interrupt::free(|cs| {
      let mut tim2 = G_TIM2.borrow(cs).borrow_mut();
      tim2.as_mut().unwrap().start((us).us())
    });
  }

  pub fn set_handler(cb: CSTimerHandler) {
    cortex_m::interrupt::free(|cs| {
      TIMER_2_HANDLER.set(Some(cb), cs);
    })
  }
}

#[interrupt]
fn TIM2() {
  cortex_m::interrupt::free(|cs| {
    TIMER_2_HANDLER.get(cs).map(|f| f(cs) );
    let mut tim2 = G_TIM2.borrow(cs).borrow_mut();
    tim2.as_mut().map(|t| {
      t.clear_update_interrupt_flag();
      t.reset();
    });
  });
}

static TIMER_3_HANDLERS: CSCell<[Option<TimerHandler>; MAX_TIM2_HANDLERS]> = CSCell::new([None; MAX_TIM2_HANDLERS]);
static G_TIM3: Mutex<RefCell<Option<CountDownTimer<TIM3>>>> = Mutex::new(RefCell::new(None));

pub struct Timer3;
impl Timer3 {
  pub fn init(tim3: TIM3, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {
    
    // set timer2 to 100khz = 10us
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
fn TIM3() {
  let handlers = unsafe { TIMER_3_HANDLERS.get_unsafe() };
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