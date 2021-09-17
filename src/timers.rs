
use stm32f1xx_hal::{
  pac::{interrupt, Interrupt, TIM3, TIM2},
  prelude::*,
  timer::{Event, Timer, CountDownTimer},
};

use cortex_m::interrupt::{CriticalSection, Mutex};
use core::cell::{ UnsafeCell, RefCell};

type TimerHandler = fn();

const MAX_HANDLERS: usize = 5;

struct CSTimerHandler( UnsafeCell<[Option<TimerHandler>; MAX_HANDLERS]> );

impl CSTimerHandler {
  fn set(&self, index: usize, handler: TimerHandler, _cs: &CriticalSection ) {
    unsafe {
      (*self.0.get())[index] = Some(handler);
    }
  }
  pub fn execute(&self, _cs: &CriticalSection) {
    unsafe {
      for i in 0..MAX_HANDLERS {
        (*self.0.get())[i].map(|f| { f(); } );
      }
    }
  }
}

unsafe impl Sync for CSTimerHandler {}

static TIMER_2_HANDLERS: CSTimerHandler = CSTimerHandler(UnsafeCell::new([None; MAX_HANDLERS]));
static G_TIM2: Mutex<RefCell<Option<CountDownTimer<TIM2>>>> = Mutex::new(RefCell::new(None));

pub struct Timer2;
impl Timer2  {
  pub fn init(tim2: TIM2, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {

    // set timer2 to 40hz = 25ms
    let mut timer = Timer::tim2(tim2, &clocks, apb1).start_count_down(40.hz());

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
      TIMER_2_HANDLERS.set(index, cb, cs);
    })
  }
}

#[interrupt]
fn TIM2() {
  cortex_m::interrupt::free(|cs| {
    TIMER_2_HANDLERS.execute(cs);
    let mut tim2 = G_TIM2.borrow(cs).borrow_mut();
    tim2.as_mut().unwrap().clear_update_interrupt_flag();
  });
}

static TIMER_3_HANDLERS: CSTimerHandler = CSTimerHandler(UnsafeCell::new([None; MAX_HANDLERS]));
static G_TIM3: Mutex<RefCell<Option<CountDownTimer<TIM3>>>> = Mutex::new(RefCell::new(None));

pub struct Timer3;
impl Timer3 {
  pub fn init(tim3: TIM3, clocks: &stm32f1xx_hal::rcc::Clocks, apb1: &mut stm32f1xx_hal::rcc::APB1) {
    
    // set timer2 to 1khz = 1ms
    let mut timer = Timer::tim3(tim3, &clocks, apb1).start_count_down(1.khz());

    // Generate an interrupt when the timer expires
    timer.listen(Event::Update);

    cortex_m::interrupt::free(|cs| G_TIM3.borrow(cs).replace(Some(timer)));

    cortex_m::peripheral::NVIC::unpend(Interrupt::TIM3);
    unsafe {
      cortex_m::peripheral::NVIC::unmask(Interrupt::TIM3);
    }
  }

  pub fn add_handler(index: usize, cb: TimerHandler) {
    cortex_m::interrupt::free(|cs| {
      TIMER_3_HANDLERS.set(index, cb, cs);
    })
  }
}

#[interrupt]
fn TIM3() {
  cortex_m::interrupt::free(|cs| {
    TIMER_3_HANDLERS.execute(cs);
    let mut tim3 = G_TIM3.borrow(cs).borrow_mut();
    tim3.as_mut().unwrap().clear_update_interrupt_flag();
  });
}