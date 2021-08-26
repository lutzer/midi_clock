use crate::peripherals::*;

use core::mem::MaybeUninit;

use stm32f1xx_hal::{
  gpio::{ExtiPin, Edge},
  afio,
  pac
};

use pac::interrupt;

pub struct Buttons {}

static mut BUTTON1_GPIO: MaybeUninit<Button1Gpio> = MaybeUninit::uninit();
static mut BUTTON1_STATE: bool = false;

#[interrupt]
fn EXTI9_5() {
    let button1_gpio = unsafe { &mut *BUTTON1_GPIO.as_mut_ptr() };

    if button1_gpio.check_interrupt() {
        let button1_state = unsafe { &mut *BUTTON1_STATE.as_mut_ptr() };
        button1_gpio.clear_interrupt_pending_bit();
    }
}

impl Buttons {
  pub fn new(mut button1: Button1Gpio,  afio: &mut afio::Parts, exti: &stm32f1xx_hal::pac::EXTI) -> Buttons {

    let button1_gpio = unsafe { &mut *BUTTON1_GPIO.as_mut_ptr() };
    *button1_gpio = button1;

    button1_gpio.make_interrupt_source(afio);
    button1_gpio.trigger_on_edge(&exti, Edge::FALLING);
    button1_gpio.enable_interrupt(&exti);

    unsafe {
      pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }

    return Buttons{}
  }

  pub fn read() {

  }
}