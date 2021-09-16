use stm32f1xx_hal::{
  pac::{interrupt},
  gpio,
  gpio::ExtiPin,
  gpio::Edge,
  afio,
  pac
};

use embedded_hal::digital::v2::InputPin;

use crate::debug::*;
use crate::debug;

use crate::utils::*;

use core::mem::MaybeUninit;

type EncoderChangeHandler = fn(u8);

#[derive(Copy,Clone)]
enum EncoderState {
  CW_START = 0x01,
  CW_STEP1 = 0x02,
  CW_STEP2 = 0x03,
  CW_FINAL = 0x10,

  CCW_START = 0x04,
  CCW_STEP1 = 0x05,
  CCW_STEP3 = 0x06,
  CCW_FINAL = 0x20,

  UNDEFINED = 0x40,
}

const TRANSITION_TABLE: [[EncoderState; 4]; 7] = [
  [ EncoderState::UNDEFINED, EncoderState::CCW_START, EncoderState::CW_START, EncoderState::UNDEFINED ],  // init state -> 1,2

  [ EncoderState::CW_STEP1, EncoderState::UNDEFINED, EncoderState::UNDEFINED, EncoderState::UNDEFINED ], // cw start -> 0
  [ EncoderState::UNDEFINED, EncoderState::CW_STEP2, EncoderState::CW_START, EncoderState::UNDEFINED ], // cw step1 -> 1, <- 2
  [ EncoderState::CW_STEP1, EncoderState::UNDEFINED, EncoderState::UNDEFINED, EncoderState::CW_FINAL ], // cw step2 -> 3, <- 0

  [ EncoderState::CCW_STEP1, EncoderState::UNDEFINED, EncoderState::UNDEFINED, EncoderState::UNDEFINED ], // ccw start -> 0
  [ EncoderState::UNDEFINED, EncoderState::CCW_START, EncoderState::CCW_STEP3, EncoderState::UNDEFINED ], // ccw step1 -> 2, <- 0
  [ EncoderState::UNDEFINED, EncoderState::CCW_STEP1, EncoderState::UNDEFINED , EncoderState::CCW_FINAL ] // ccw step2 -> 3, <- 2
];

pub struct Encoder {
  position: i32,
  change_handler: EncoderChangeHandler
}

static mut ENCODER_PIN1: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA0<gpio::Input<gpio::PullUp>>> =
  MaybeUninit::uninit();

static mut ENCODER_PIN2: MaybeUninit<stm32f1xx_hal::gpio::gpiob::PB0<gpio::Input<gpio::PullUp>>> =
  MaybeUninit::uninit();

impl Encoder {
  pub fn init(
    exti: &stm32f1xx_hal::pac::EXTI, 
    pa0: gpio::gpioa::PA0<gpio::Input<gpio::Floating>>,
    crla: &mut gpio::gpioa::CRL,
    pb0: gpio::gpiob::PB0<gpio::Input<gpio::Floating>>,
    crlb: &mut gpio::gpiob::CRL,
    afio: &mut afio::Parts,
  ) {
    // enable interrupt for pin pa0
    let enc_pin1 = unsafe { &mut *ENCODER_PIN1.as_mut_ptr() };
    *enc_pin1 = pa0.into_pull_up_input(crla);
    enc_pin1.make_interrupt_source(afio);
    enc_pin1.trigger_on_edge(exti, Edge::RISING_FALLING);
    enc_pin1.enable_interrupt(exti);

    // enable interrupt for pin pb0
    let enc_pin2 = unsafe { &mut *ENCODER_PIN2.as_mut_ptr() };
    *enc_pin2 = pb0.into_pull_up_input(crlb);
    enc_pin2.make_interrupt_source(afio);
    enc_pin2.trigger_on_edge(exti, Edge::RISING_FALLING);
    enc_pin2.enable_interrupt(exti);

    unsafe {
      pac::NVIC::unmask(pac::Interrupt::EXTI0);
    }
   
  }

  pub fn new(handler: EncoderChangeHandler) -> Encoder {
    return Encoder { position: 0 , change_handler: handler };
  }

  pub fn update(&self) {
    // static mut LAST_POSITION : u32 = 0;
  }
}

#[interrupt]
fn EXTI0() {
  static mut STATE: EncoderState = EncoderState::UNDEFINED;

  let enc_pin2 = unsafe { &mut *ENCODER_PIN2.as_mut_ptr() };
  let enc_pin1 = unsafe { &mut *ENCODER_PIN1.as_mut_ptr() };

  let reading = enc_pin1.is_low().unwrap() as u8 | (enc_pin2.is_low().unwrap() as u8) << 1;
  debug!("interrupt");
  debug!(num_to_string(reading as u16));

  // unsafe { STATE = TRANSITION_TABLE[0][0]; }

  if enc_pin1.check_interrupt() {
    enc_pin1.clear_interrupt_pending_bit();
  }
  if enc_pin2.check_interrupt() {
    enc_pin2.clear_interrupt_pending_bit();
  }
}