use cortex_m_rt;

use stm32f1xx_hal::{
  delay::Delay, 
  pac, 
  prelude::*,
  gpio,
  afio,
  serial::{Serial, Config},
};

use crate::pac::{USART1};

type GpioOutput = gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>;

type Usart1Serial = Serial<
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
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    // access PGIOC and PGIOB registers and prepare the alternate function I/O registers
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    return Peripherals{
      led: Some(gpioc.pc13.into_push_pull_output(&mut gpioc.crh)),
      delay: Some(Delay::new(cp.SYST, clocks)),
      usart1: None
    }
  }

  // fn init_led(gpioc: &mut gpio::gpioc::Parts) -> Option<GpioOutput> {
  //   let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
  //   return Some(led);
  // }

  // fn init_usart1(
  //   dp: &pac::Peripherals, 
  //   gpioa: &mut gpio::gpioa::Parts, 
  //   afio: &mut afio::Parts, 
  //   clocks: &stm32f1xx_hal::rcc::Clocks, 
  //   rcc: &mut stm32f1xx_hal::rcc::Rcc
  // ) -> Option<Usart1Serial> {
  //   let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
  //   let rx = gpioa.pa10;

  //   let serial = Serial::usart1(
  //     dp.USART1,
  //     (tx, rx),
  //     &mut afio.mapr,
  //     Config::default().baudrate(115200.bps()),
  //     *clocks,
  //     &mut rcc.apb2,
  //   );
  //   return Some(serial);
  // }
}