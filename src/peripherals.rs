/*
 * Initializes all used peripherals on the stm32
 */

use stm32f1xx_hal::{
  pac, 
  prelude::*,
  gpio,
  afio,
  serial::{Serial, Config},
};


use crate::timers::*;

use stm32f1xx_hal::pac::{USART1};

pub use embedded_hal::digital::v2::{OutputPin, InputPin};

// types for Initialized peripherals
pub type Led1Gpio = gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>;

pub type Button1Gpio = gpio::gpioa::PA0<gpio::Input<gpio::PullUp>>;
pub type Button2Gpio = gpio::gpioa::PA1<gpio::Input<gpio::PullUp>>;
pub type Button3Gpio = gpio::gpioa::PA4<gpio::Input<gpio::PullUp>>;

pub type Usart1Serial = Serial<
  USART1, (gpio::gpioa::PA9<gpio::Alternate<gpio::PushPull>>, 
  gpio::gpioa::PA10<gpio::Input<gpio::Floating>>)>;

// holds all peripherals
pub struct Peripherals {
  pub led: Option<Led1Gpio>,
  pub button1: Option<Button1Gpio>,
  pub button2: Option<Button2Gpio>,
  pub button3: Option<Button3Gpio>,
  pub usart1: Option<Usart1Serial>
}

impl<'a> Peripherals {

  pub fn init() -> Peripherals {

    let dp = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();

    // set clock frequency to internal 8mhz oscillator
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    // access PGIOC and PGIOB registers and prepare the alternate function I/O registers
    let mut apb1 = rcc.apb1;
    let mut apb2 = rcc.apb2;
    let mut gpioc = dp.GPIOC.split(&mut apb2);
    let mut gpioa = dp.GPIOA.split(&mut apb2);
    let mut afio = dp.AFIO.constrain(&mut apb2);

    // init timers
    Timer2::init(dp.TIM2, &clocks, &mut apb1);

    return Peripherals {
      led: Peripherals::init_led(gpioc.pc13, &mut gpioc.crh),
      button1: Peripherals::init_button1(gpioa.pa0, &mut gpioa.crl),
      button2: Peripherals::init_button2(gpioa.pa1, &mut gpioa.crl),
      button3: Peripherals::init_button3(gpioa.pa4, &mut gpioa.crl),
      usart1: Peripherals::init_usart1(dp.USART1, gpioa.pa9, gpioa.pa10, &mut gpioa.crh, &mut afio, &clocks, apb2)
    };
  }

  fn init_led(
    pc13: gpio::gpioc::PC13<gpio::Input<gpio::Floating>>, 
    crh: &mut gpio::gpioc::CRH
  ) -> Option<Led1Gpio> {
    let led = pc13.into_push_pull_output(crh);
    return Some(led);
  }

  fn init_button1(
    pa0: gpio::gpioa::PA0<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpioa::CRL
  ) -> Option<Button1Gpio> {
    let button = pa0.into_pull_up_input(crl);
    return Some(button);
  }

  fn init_button2(
    pa1: gpio::gpioa::PA1<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpioa::CRL
  ) -> Option<Button2Gpio> {
    let button = pa1.into_pull_up_input(crl);
    return Some(button);
  }

  fn init_button3(
    pa4: gpio::gpioa::PA4<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpioa::CRL
  ) -> Option<Button3Gpio> {
    let button = pa4.into_pull_up_input(crl);
    return Some(button);
  }

  fn init_usart1(
    usart1: USART1, 
    pa9: gpio::gpioa::PA9<gpio::Input<gpio::Floating>>,
    pa10: gpio::gpioa::PA10<gpio::Input<gpio::Floating>>,
    crh: &mut gpio::gpioa::CRH,
    afio: &mut afio::Parts, 
    clocks: &stm32f1xx_hal::rcc::Clocks, 
    mut apb2: stm32f1xx_hal::rcc::APB2
  ) -> Option<Usart1Serial> {
    let tx = pa9.into_alternate_push_pull(crh);
    let rx = pa10;

    let serial = Serial::usart1(
      usart1,
      (tx, rx),
      &mut afio.mapr,
      Config::default().baudrate(115200.bps()),
      *clocks,
      &mut apb2,
    );
    return Some(serial);
  }
}
