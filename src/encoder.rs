use stm32f1xx_hal::{
  pac::{interrupt},
  gpio,
  gpio::ExtiPin,
  gpio::Edge,
  afio,
  pac
};

use core::sync::atomic::{AtomicI32, Ordering};

use cortex_m::interrupt::{Mutex};
use core::cell::{RefCell};

use embedded_hal::digital::v2::InputPin;

#[derive(Copy,Clone)]
enum EncoderState {
  CwStart = 0x01,
  CwStep1 = 0x02,
  CwStep2 = 0x03,
  CwFinal = 0x10,

  CcwStart = 0x04,
  CcwStep1 = 0x05,
  CcwStep2 = 0x06,
  CcwFinal = 0x20,

  Undefined = 0x40,
}

const TRANSITION_LOOKUPTABLE: [[EncoderState; 4]; 7] = [
  [ EncoderState::Undefined, EncoderState::CcwStart, EncoderState::CwStart, EncoderState::Undefined ],  // init state -> 1,2

  [ EncoderState::Undefined, EncoderState::Undefined, EncoderState::Undefined, EncoderState::CwStep1 ], // cw start -> 3
  [ EncoderState::Undefined, EncoderState::CwStep2, EncoderState::CwStart, EncoderState::Undefined ], // cw step1 -> 1, <- 2
  [ EncoderState::CwFinal, EncoderState::CwStep2, EncoderState::Undefined, EncoderState::CwStep1 ], // cw step2 -> 0, <- 2

  [ EncoderState::Undefined, EncoderState::Undefined, EncoderState::Undefined, EncoderState::CcwStep1 ], // ccw start -> 3
  [ EncoderState::Undefined, EncoderState::Undefined, EncoderState::CcwStep2, EncoderState::Undefined ], // ccw step1 -> 2, <- 0
  [ EncoderState::CcwFinal, EncoderState::Undefined, EncoderState::Undefined , EncoderState::Undefined ] // ccw step2 -> 0, <- 2
];

fn get_transition(state: u8, transition: u8) -> EncoderState {
  return TRANSITION_LOOKUPTABLE[state as usize][transition as usize];
}

const MAX_I32_HALF: i32 = i32::MAX / 2;

static ENCODER_POSITION: AtomicI32 = AtomicI32::new(0);

pub struct Encoder {}

type EncoderPin1Type = stm32f1xx_hal::gpio::gpioa::PA0<gpio::Input<gpio::PullUp>>;
type EncoderPin2Type = stm32f1xx_hal::gpio::gpioa::PA1<gpio::Input<gpio::PullUp>>;

static ENCODER_PIN1: Mutex<RefCell<Option<EncoderPin1Type>>> =
  Mutex::new(RefCell::new(None));

static ENCODER_PIN2: Mutex<RefCell<Option<EncoderPin2Type>>> =
  Mutex::new(RefCell::new(None));

impl Encoder {

  pub fn init(
    exti: &stm32f1xx_hal::pac::EXTI, 
    pa0: gpio::gpioa::PA0<gpio::Input<gpio::Floating>>,
    pa1: gpio::gpioa::PA1<gpio::Input<gpio::Floating>>,
    crl: &mut gpio::gpioa::CRL,
    afio: &mut afio::Parts,
  ) {
    // enable interrupt for pin pa0
    let mut enc_pin1 = pa0.into_pull_up_input(crl);
    enc_pin1.make_interrupt_source(afio);
    enc_pin1.trigger_on_edge(exti, Edge::RISING_FALLING);
    enc_pin1.enable_interrupt(exti);
    cortex_m::interrupt::free(|cs| ENCODER_PIN1.borrow(cs).replace(Some(enc_pin1)));

    // enable interrupt for pin pa1
    let mut enc_pin2 = pa1.into_pull_up_input(crl);
    enc_pin2.make_interrupt_source(afio);
    enc_pin2.trigger_on_edge(exti, Edge::RISING_FALLING);
    enc_pin2.enable_interrupt(exti);
    cortex_m::interrupt::free(|cs| ENCODER_PIN2.borrow(cs).replace(Some(enc_pin2)));

    unsafe {
      pac::NVIC::unmask(pac::Interrupt::EXTI0);
      pac::NVIC::unmask(pac::Interrupt::EXTI1);
    }
  }


  pub fn new() -> Encoder {
    return Encoder {};
  }

  pub fn on_change(&self) -> Option<i16> {
    static mut LAST_POSITION : i32 = 0;

    let position = ENCODER_POSITION.load(Ordering::Relaxed);
    
    // calculate the difference between last read position and current position
    unsafe {
      let delta = position - LAST_POSITION;

      // reset position when position is out of bounds
      if i32::abs(delta) > MAX_I32_HALF {
        LAST_POSITION = 0;
        ENCODER_POSITION.store(0, Ordering::Relaxed);
      // else call handler function
      } else if delta != 0 {
        LAST_POSITION = position;
        return Some(delta as i16);
      }
    }
    return None;
  }
}

unsafe fn on_interrupt(pin1: Option<bool>, pin2: Option<bool>) {
  static mut STATE: EncoderState = EncoderState::Undefined;
  static mut READING: u8 = 0;

  pin1.map(|r| {
    READING &= !0b01;
    READING |= r as u8;
  });

  pin2.map(|r| {
    READING &= !0b10;
    READING |= (r as u8) << 1;
  });

  STATE = get_transition((STATE as u8) & 0x0F, READING);
  
  if STATE as u8 == EncoderState::CwFinal as u8 {
    ENCODER_POSITION.fetch_add(1, Ordering::Relaxed);
  } else if STATE as u8 == EncoderState::CcwFinal as u8 {
    ENCODER_POSITION.fetch_add(-1, Ordering::Relaxed);
  }
}

// interrupt for pin1
#[interrupt]
unsafe fn EXTI0() {
  cortex_m::interrupt::free(|cs|  {
    let mut enc_pin1 = ENCODER_PIN1.borrow(cs).borrow_mut();
    on_interrupt(Some(enc_pin1.as_ref().unwrap().is_low().unwrap()), None);
    enc_pin1.as_mut().unwrap().clear_interrupt_pending_bit();
  });
}

// interrupt for pin2
#[interrupt]
unsafe fn EXTI1() {
  cortex_m::interrupt::free(|cs|  {
    let mut enc_pin2 = ENCODER_PIN2.borrow(cs).borrow_mut();
    on_interrupt(None, Some(enc_pin2.as_ref().unwrap().is_low().unwrap()));
    enc_pin2.as_mut().unwrap().clear_interrupt_pending_bit();
  });
}