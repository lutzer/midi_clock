/*
 * Initializes all used peripherals on the stm32
 */

use stm32f1xx_hal::{
  pac, 
  prelude::*,
  gpio,
  afio,
  serial::{Serial, Config},
  i2c::{BlockingI2c, DutyCycle, Mode}
};

use crate::timers::*;
use crate::encoder::*;

use stm32f1xx_hal::pac::{USART1, USART2};

pub use embedded_hal::digital::v2::{OutputPin, InputPin};

// types for Initialized peripherals
pub type Led1Gpio = gpio::gpioc::PC13<gpio::Output<gpio::PushPull>>;

pub type Button1Gpio = gpio::gpiob::PB12<gpio::Input<gpio::PullUp>>;
pub type Button2Gpio = gpio::gpiob::PB13<gpio::Input<gpio::PullUp>>;
pub type Button3Gpio = gpio::gpioa::PA7<gpio::Input<gpio::PullUp>>;
pub type Button4Gpio = gpio::gpioa::PA6<gpio::Input<gpio::PullUp>>;

pub type DisplayI2C = BlockingI2c<pac::I2C1, (
  gpio::gpiob::PB8<gpio::Alternate<gpio::OpenDrain>>,
  gpio::gpiob::PB9<gpio::Alternate<gpio::OpenDrain>>)>;

pub type Usart1Serial = Serial<
  USART1, (gpio::gpioa::PA9<gpio::Alternate<gpio::PushPull>>, 
  gpio::gpioa::PA10<gpio::Input<gpio::Floating>>)>;

pub type Usart2Serial = Serial<
    USART2, (gpio::gpioa::PA2<gpio::Alternate<gpio::PushPull>>, 
    gpio::gpioa::PA3<gpio::Input<gpio::Floating>>)>;

// holds all peripherals
pub struct Peripherals {
  pub led: Option<Led1Gpio>,
  pub button1: Option<Button1Gpio>,
  pub button2: Option<Button2Gpio>,
  pub button3: Option<Button3Gpio>,
  pub button4: Option<Button4Gpio>,
  pub usart1: Option<Usart1Serial>,
  pub usart2: Option<Usart2Serial>,
  pub displayi2c: Option<DisplayI2C>
}

impl Peripherals {
  pub fn init() -> Peripherals {

    let dp = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr
      .use_hse(8.mhz()) // set clock frequency to external 8mhz oscillator
      .sysclk(72.mhz()) // set sysclock 
      .pclk1(36.mhz()) // clock for apb1 prescaler -> TIM1
      .pclk2(36.mhz()) // clock for apb2 prescaler -> TIM2,3,4
      .adcclk(12.mhz()) // clock for analog digital converters
      .freeze(&mut flash.acr);

    // access PGIOC and PGIOB registers and prepare the alternate function I/O registers
    let mut apb1 = rcc.apb1;
    let mut apb2 = rcc.apb2;
    let mut gpioa = dp.GPIOA.split(&mut apb2);
    let mut gpiob = dp.GPIOB.split(&mut apb2);
    let mut gpioc = dp.GPIOC.split(&mut apb2);
    let mut afio = dp.AFIO.constrain(&mut apb2);


    // init timers
    Timer2::init(dp.TIM2, &clocks, &mut apb1);
    Timer3::init(dp.TIM3, &clocks, &mut apb1);

    // init encoder interrupts
    Encoder::init(&dp.EXTI, gpioa.pa0, gpioa.pa1, &mut gpioa.crl, &mut afio );

    return Peripherals {
      led: Peripherals::init_led(gpioc.pc13, &mut gpioc.crh),

      button1: Peripherals::init_button1(gpiob.pb12, &mut gpiob.crh),
      button2: Peripherals::init_button2(gpiob.pb13, &mut gpiob.crh),
      button3: Peripherals::init_button3(gpioa.pa7, &mut gpioa.crl),
      button4: Peripherals::init_button4(gpioa.pa6, &mut gpioa.crl),

      usart1: Peripherals::init_usart1(dp.USART1, gpioa.pa9, gpioa.pa10, &mut gpioa.crh, &mut afio, &clocks, &mut apb2),
      usart2: Peripherals::init_usart2(dp.USART2, gpioa.pa2, gpioa.pa3, &mut gpioa.crl, &mut afio, &clocks, &mut apb1),
      displayi2c: Peripherals::init_displayi2c(dp.I2C1, gpiob.pb8, gpiob.pb9, &mut gpiob.crh, &mut afio, &clocks, &mut apb1)
    };
  }

  fn init_led(
    pc13: gpio::gpioc::PC13<gpio::Input<gpio::Floating>>, 
    crh: &mut gpio::gpioc::CRH
  ) -> Option<Led1Gpio> {
    let mut led = pc13.into_push_pull_output(crh);
    led.set_high().ok();
    return Some(led);
  }

  fn init_button1(
    pb12: gpio::gpiob::PB12<gpio::Input<gpio::Floating>>, 
    crh: &mut gpio::gpiob::CRH
  ) -> Option<Button1Gpio> {
    let button = pb12.into_pull_up_input(crh);
    return Some(button);
  }

  fn init_button2(
    pb13: gpio::gpiob::PB13<gpio::Input<gpio::Floating>>, 
    crh: &mut gpio::gpiob::CRH
  ) -> Option<Button2Gpio> {
    let button = pb13.into_pull_up_input(crh);
    return Some(button);
  }

  fn init_button3(
    pa7: gpio::gpioa::PA7<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpioa::CRL
  ) -> Option<Button3Gpio> {
    let button = pa7.into_pull_up_input(crl);
    return Some(button);
  }

  fn init_button4(
    pa6: gpio::gpioa::PA6<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpioa::CRL
  ) -> Option<Button4Gpio> {
    let button = pa6.into_pull_up_input(crl);
    return Some(button);
  }

  fn init_usart1(
    usart1: USART1, 
    pa9: gpio::gpioa::PA9<gpio::Input<gpio::Floating>>,
    pa10: gpio::gpioa::PA10<gpio::Input<gpio::Floating>>,
    crh: &mut gpio::gpioa::CRH,
    afio: &mut afio::Parts, 
    clocks: &stm32f1xx_hal::rcc::Clocks, 
    apb2: &mut stm32f1xx_hal::rcc::APB2
  ) -> Option<Usart1Serial> {
    let tx = pa9.into_alternate_push_pull(crh);
    let rx = pa10;

    let serial = Serial::usart1(
      usart1,
      (tx, rx),
      &mut afio.mapr,
      Config::default().baudrate(115200.bps()),
      *clocks,
      apb2,
    );
    return Some(serial);
  }

  fn init_usart2(
    usart2: USART2, 
    pa2: gpio::gpioa::PA2<gpio::Input<gpio::Floating>>,
    pa3: gpio::gpioa::PA3<gpio::Input<gpio::Floating>>,
    crl: &mut gpio::gpioa::CRL,
    afio: &mut afio::Parts, 
    clocks: &stm32f1xx_hal::rcc::Clocks, 
    apb1: &mut stm32f1xx_hal::rcc::APB1
  ) -> Option<Usart2Serial> {
    let tx = pa2.into_alternate_push_pull(crl);
    let rx = pa3;

    let serial = Serial::usart2(
      usart2,
      (tx, rx),
      &mut afio.mapr,
      Config::default().baudrate(31250.bps()),
      *clocks,
      apb1,
    );
    return Some(serial);
  }

  fn init_displayi2c(
    i2c: pac::I2C1,
    pb8: gpio::gpiob::PB8<gpio::Input<gpio::Floating>>,
    pb9: gpio::gpiob::PB9<gpio::Input<gpio::Floating>>,
    crh: &mut gpio::gpiob::CRH,
    afio: &mut afio::Parts,
    clocks: &stm32f1xx_hal::rcc::Clocks,
    apb1: &mut stm32f1xx_hal::rcc::APB1
  ) -> Option<DisplayI2C> {
    // init i2c
    let scl = pb8.into_alternate_open_drain(crh);
    let sda = pb9.into_alternate_open_drain(crh);

    let i2c = BlockingI2c::i2c1(
      i2c,
      (scl, sda),
      &mut afio.mapr,
      Mode::Fast {
          frequency: 400_000.hz(),
          duty_cycle: DutyCycle::Ratio16to9,
      },
      *clocks,
      apb1,
      100, // start timeout
      5, // start retries
      100, // addr timeout
      100 // data timeout
     );
     return Some(i2c);
  }
}
