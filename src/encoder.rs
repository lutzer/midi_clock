use stm32f1xx_hal::{
  pac::{interrupt},
  gpio,
  gpio::ExtiPin,
  gpio::Edge,
  afio,
  pac
};

use embedded_hal::digital::v2::InputPin;

use core::sync::atomic::{AtomicI32, Ordering};

use cortex_m::interrupt::{Mutex};
use core::cell::{RefCell};

#[derive(Copy,Clone)]
enum EncoderState {
  CwStart = 0x01,
  CwFinal = 0x10,

  CcwStart = 0x02,
  CcwFinal = 0x20,

  Undefined = 0x40,
}

// lookup table for transitioning the encoder state from current state and current reading
const TRANSITION_LOOKUPTABLE: [[EncoderState; 4]; 3] = [
  [ EncoderState::Undefined, EncoderState::Undefined, EncoderState::CwStart, EncoderState::CcwStart ],  // init state -> 2,3
  [ EncoderState::Undefined, EncoderState::CwFinal, EncoderState::Undefined, EncoderState::Undefined ], // cw start -> 1
  [ EncoderState::CcwFinal, EncoderState::Undefined, EncoderState::Undefined, EncoderState::Undefined ], // cw start -> 0
];

fn get_transition(state: u8, transition: u8) -> EncoderState {
  return TRANSITION_LOOKUPTABLE[state as usize][transition as usize];
}

const MAX_I32_HALF: i32 = i32::MAX / 2;

static ENCODER_POSITION: AtomicI32 = AtomicI32::new(0);

type EncoderChangeHandler = fn(i16);

pub struct Encoder {
  change_handler: EncoderChangeHandler
}

type EncoderPin1Type = stm32f1xx_hal::gpio::gpioa::PA0<gpio::Input<gpio::PullUp>>;
type EncoderPin2Type = stm32f1xx_hal::gpio::gpiob::PB0<gpio::Input<gpio::PullUp>>;

static ENCODER_PIN1: Mutex<RefCell<Option<EncoderPin1Type>>> =
  Mutex::new(RefCell::new(None));

static ENCODER_PIN2: Mutex<RefCell<Option<EncoderPin2Type>>> =
  Mutex::new(RefCell::new(None));

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
    let mut enc_pin1 = pa0.into_pull_up_input(crla);
    enc_pin1.make_interrupt_source(afio);
    enc_pin1.trigger_on_edge(exti, Edge::RISING_FALLING);
    enc_pin1.enable_interrupt(exti);
    cortex_m::interrupt::free(|cs| ENCODER_PIN1.borrow(cs).replace(Some(enc_pin1)));

    // enable interrupt for pin pb0
    let mut enc_pin2 = pb0.into_pull_up_input(crlb);
    enc_pin2.make_interrupt_source(afio);
    enc_pin2.trigger_on_edge(exti, Edge::RISING_FALLING);
    enc_pin2.enable_interrupt(exti);
    cortex_m::interrupt::free(|cs| ENCODER_PIN2.borrow(cs).replace(Some(enc_pin2)));

    unsafe {
      pac::NVIC::unmask(pac::Interrupt::EXTI0);
    }
  }

  pub fn new(handler: EncoderChangeHandler) -> Encoder {
    return Encoder { change_handler: handler };
  }

  pub fn update(&self) {
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
        (self.change_handler)(delta as i16);
        LAST_POSITION = position;
      }
    }
  }
}

#[interrupt]
fn EXTI0() {
  static mut STATE: EncoderState = EncoderState::Undefined;

  cortex_m::interrupt::free(|cs|  {
    let mut enc_pin1 = ENCODER_PIN1.borrow(cs).borrow_mut();
    let mut enc_pin2 = ENCODER_PIN2.borrow(cs).borrow_mut();

    let reading = enc_pin1.as_ref().unwrap().is_low().unwrap() as u8 |
      (enc_pin2.as_ref().unwrap().is_low().unwrap() as u8) << 1;

    // set new state depending on reading and old state
    *STATE = get_transition((*STATE as u8) & 0x0F, reading);

    // adjust position when reached final turn state
    if *STATE as u8 == EncoderState::CwFinal as u8 {
      ENCODER_POSITION.fetch_add(1, Ordering::Relaxed);
    } else if *STATE as u8 == EncoderState::CcwFinal as u8 {
      ENCODER_POSITION.fetch_add(-1, Ordering::Relaxed);
    }

    enc_pin1.as_mut().unwrap().clear_interrupt_pending_bit();
    enc_pin2.as_mut().unwrap().clear_interrupt_pending_bit();
  });
}