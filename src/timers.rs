
use stm32f1xx_hal::{
  pac::{interrupt, Interrupt, TIM3, TIM2},
  prelude::*,
  timer::{Event, Timer, CountDownTimer},
};

use cortex_m::interrupt::{CriticalSection, Mutex};
use core::cell::{RefCell};

use crate::utils::{CSCell};

type TimerHandler = fn(&CriticalSection);

const MAX_TIM2_HANDLERS: usize = 3;


static TIMER_2_HANDLERS: CSCell<[Option<TimerHandler>; MAX_TIM2_HANDLERS]> = CSCell::new([None; MAX_TIM2_HANDLERS]);
static G_TIM2: Mutex<RefCell<Option<CountDownTimer<TIM2>>>> = Mutex::new(RefCell::new(None));

pub struct Timer2;
impl Timer2  {
  pub fn init(tim2: TIM2, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {

    // set timer2 to 1000hz = 1ms
    let mut timer = Timer::tim2(tim2, &clocks, apb1).start_count_down(1000.hz());

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
      let handlers = TIMER_2_HANDLERS.get(cs);
      handlers[index] = Some(cb);
      TIMER_2_HANDLERS.set(*handlers, cs);
    })
  }
}

#[interrupt]
fn TIM2() {
  cortex_m::interrupt::free(|cs| {
    let handlers = TIMER_2_HANDLERS.get(cs);
    for handler in handlers {
      handler.map(|f| f(cs));
    }
    let mut tim2 = G_TIM2.borrow(cs).borrow_mut();
    tim2.as_mut().unwrap().clear_update_interrupt_flag();
  });
}

static TIMER_3_HANDLER: CSCell<Option<TimerHandler>> = CSCell::new(None);
static G_TIM3: Mutex<RefCell<Option<CountDownTimer<TIM3>>>> = Mutex::new(RefCell::new(None));

pub struct Timer3;
impl Timer3 {
  pub fn init(tim3: TIM3, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {
    
    // set timer2 to 100khz = 10us
    let timer = Timer::tim3(tim3, &clocks, apb1).start_count_down(1.hz());

    cortex_m::interrupt::free(|cs| G_TIM3.borrow(cs).replace(Some(timer)));

    cortex_m::peripheral::NVIC::unpend(Interrupt::TIM3);
    unsafe {
      cortex_m::peripheral::NVIC::unmask(Interrupt::TIM3);
    }
  }

  pub fn set_running(running: bool) {
    cortex_m::interrupt::free(|cs| {
      let mut tim3 = G_TIM3.borrow(cs).borrow_mut();
      if running {
        tim3.as_mut().unwrap().listen(Event::Update);
      } else {
        tim3.as_mut().unwrap().unlisten(Event::Update);
      }
    });
  }

  pub fn set_frequency(hertz: u32) {
    cortex_m::interrupt::free(|cs| {
      let mut tim3 = G_TIM3.borrow(cs).borrow_mut();
      tim3.as_mut().unwrap().start((hertz).hz())
    });
  }

  pub fn add_handler(cb: TimerHandler) {
    cortex_m::interrupt::free(|cs| {
      TIMER_3_HANDLER.set(Some(cb), cs);
    })
  }
}

#[interrupt]
fn TIM3() {
  cortex_m::interrupt::free(|cs| {
    TIMER_3_HANDLER.get(cs).map(|f| f(cs) );
    let mut tim3 = G_TIM3.borrow(cs).borrow_mut();
    tim3.as_mut().unwrap().clear_update_interrupt_flag();
  });
}