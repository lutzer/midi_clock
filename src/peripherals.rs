use stm32f1xx_hal::{
  delay::Delay, 
  pac, 
  prelude::*,
  gpio,
  afio,
  serial::{Serial, Config},
};

extern crate alloc;
use alloc::boxed::{Box};

use stm32f1xx_hal::pac::{USART1};

use embedded_hal::digital::v2::{OutputPin};
use core::convert::Infallible;

type GpioOutput = Box<dyn OutputPin<Error = Infallible>>;

pub type Usart1Serial = Serial<
  USART1, (gpio::gpioa::PA9<gpio::Alternate<gpio::PushPull>>, 
  gpio::gpioa::PA10<gpio::Input<gpio::Floating>>)>;

pub struct Peripherals {
  pub led: Option<GpioOutput>,
  pub delay: Option<stm32f1xx_hal::delay::Delay>,
  pub usart1: Option<Usart1Serial>
}

impl Peripherals {
  pub fn init() -> Peripherals {

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // set clock frequency to internal 8mhz oscillator
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    // access PGIOC and PGIOB registers and prepare the alternate function I/O registers
    let mut apb2 = rcc.apb2;
    let gpioc = dp.GPIOC.split(&mut apb2);
    let gpioa = dp.GPIOA.split(&mut apb2);
    let afio = dp.AFIO.constrain(&mut apb2);

    return Peripherals{
      led: Peripherals::init_led(gpioc),
      delay: Some(Delay::new(cp.SYST, clocks)),
      usart1: Peripherals::init_usart1(dp.USART1, gpioa, afio, &clocks, apb2)
    }
  }

  fn init_led(mut gpioc: stm32f1xx_hal::gpio::gpioc::Parts) -> Option<GpioOutput> {
    let led = Box::new(gpioc.pc13.into_push_pull_output(&mut gpioc.crh));
    return Some(led as GpioOutput);
  }

  fn init_usart1(
    usart1: USART1, 
    mut gpioa: gpio::gpioa::Parts, 
    mut afio: afio::Parts, 
    clocks: &stm32f1xx_hal::rcc::Clocks, 
    mut apb2: stm32f1xx_hal::rcc::APB2
  ) -> Option<Usart1Serial> {
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

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