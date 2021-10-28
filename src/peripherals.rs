/*
 * Initializes all used peripherals on the stm32
 */

use stm32f1xx_hal::{
  pac, 
  prelude::*,
  gpio,
  afio,
  serial::{Serial, Config},
  delay::{Delay}
};

use embedded_hal::spi::{Mode, Phase, Polarity};
pub const SPI_MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
};

use crate::timers::*;
use crate::encoder::*;

use stm32f1xx_hal::pac::{USART1, USART2};

pub use embedded_hal::digital::v2::{OutputPin, InputPin};

// types for Initialized peripherals
pub type Trigger1Gpio = gpio::gpiob::PB5<gpio::Output<gpio::PushPull>>;
pub type Trigger2Gpio = gpio::gpiob::PB4<gpio::Output<gpio::PushPull>>;
pub type Trigger3Gpio = gpio::gpiob::PB0<gpio::Output<gpio::PushPull>>;
pub type Trigger4Gpio = gpio::gpiob::PB1<gpio::Output<gpio::PushPull>>;

pub type Button1Gpio = gpio::gpiob::PB12<gpio::Input<gpio::PullUp>>;
pub type Button2Gpio = gpio::gpiob::PB13<gpio::Input<gpio::PullUp>>;
pub type Button3Gpio = gpio::gpioa::PA7<gpio::Input<gpio::PullUp>>;
pub type Button4Gpio = gpio::gpioa::PA6<gpio::Input<gpio::PullUp>>;

pub struct DisplayPins {
  pub rs: gpio::gpioa::PA8<gpio::Output<gpio::PushPull>>,
  pub enable: gpio::gpiob::PB15<gpio::Output<gpio::PushPull>>,
  pub d4: gpio::gpiob::PB14<gpio::Output<gpio::PushPull>>,
  pub d5: gpio::gpiob::PB10<gpio::Output<gpio::PushPull>>,
  pub d6: gpio::gpioa::PA4<gpio::Output<gpio::PushPull>>,
  pub d7: gpio::gpioa::PA5<gpio::Output<gpio::PushPull>>,
  pub delay: Delay
}


pub type Usart1Serial = Serial<
  USART1, (gpio::gpioa::PA9<gpio::Alternate<gpio::PushPull>>, 
  gpio::gpioa::PA10<gpio::Input<gpio::Floating>>)>;

pub type Usart2Serial = Serial<
    USART2, (gpio::gpioa::PA2<gpio::Alternate<gpio::PushPull>>, 
    gpio::gpioa::PA3<gpio::Input<gpio::Floating>>)>;

// holds all peripherals
pub struct Peripherals {
  pub trigger1: Option<Trigger1Gpio>,
  pub trigger2: Option<Trigger2Gpio>,
  pub trigger3: Option<Trigger3Gpio>,
  pub trigger4: Option<Trigger4Gpio>,
  pub button1: Option<Button1Gpio>,
  pub button2: Option<Button2Gpio>,
  pub button3: Option<Button3Gpio>,
  pub button4: Option<Button4Gpio>,
  pub usart1: Option<Usart1Serial>,
  pub usart2: Option<Usart2Serial>,
  pub display: Option<DisplayPins>
}

impl Peripherals {
  pub fn init() -> Peripherals {

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr
      .use_hse(8.mhz()) // set clock frequency to external 8mhz oscillator
      .sysclk(72.mhz()) // set sysclock 
      .pclk1(8.mhz()) // clock for apb1 prescaler -> TIM1
      .pclk2(8.mhz()) // clock for apb2 prescaler -> TIM2,3,4
      .adcclk(12.mhz()) // clock for analog digital converters
      .freeze(&mut flash.acr);

    // access PGIOC and PGIOB registers and prepare the alternate function I/O registers
    let mut apb1 = rcc.apb1;
    let mut apb2 = rcc.apb2;
    let mut gpioa = dp.GPIOA.split(&mut apb2);
    let mut gpiob = dp.GPIOB.split(&mut apb2);
    let mut gpioc = dp.GPIOC.split(&mut apb2);
    let mut afio = dp.AFIO.constrain(&mut apb2);

    // disable jtag debugging on pa15,pb3,pb4
    let (_, _, gpiob_pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

    // init timers
    Timer2::init(dp.TIM2, &clocks, &mut apb1);
    Timer3::init(dp.TIM3, &clocks, &mut apb1);

    // init encoder interrupts
    Encoder::init(&dp.EXTI, gpioa.pa0, gpioa.pa1, &mut gpioa.crl, &mut afio );

    // setup delay
    let delay = Delay::new(cp.SYST, clocks);

    return Peripherals {
      trigger1: Peripherals::init_trigger1(gpiob.pb5, &mut gpiob.crl),
      trigger2: Peripherals::init_trigger2(gpiob_pb4, &mut gpiob.crl),
      trigger3: Peripherals::init_trigger3(gpiob.pb0, &mut gpiob.crl),
      trigger4: Peripherals::init_trigger4(gpiob.pb1, &mut gpiob.crl),

      button1: Peripherals::init_button1(gpiob.pb12, &mut gpiob.crh),
      button2: Peripherals::init_button2(gpiob.pb13, &mut gpiob.crh),
      button3: Peripherals::init_button3(gpioa.pa7, &mut gpioa.crl),
      button4: Peripherals::init_button4(gpioa.pa6, &mut gpioa.crl),

      usart1: Peripherals::init_usart1(dp.USART1, gpioa.pa9, gpioa.pa10, &mut gpioa.crh, &mut afio, &clocks, &mut apb2),
      usart2: Peripherals::init_usart2(dp.USART2, gpioa.pa2, gpioa.pa3, &mut gpioa.crl, &mut afio, &clocks, &mut apb1),
      
      display: Peripherals::init_display(gpioa.pa8, gpiob.pb15, gpiob.pb14, gpiob.pb10, 
        gpioa.pa4, gpioa.pa5, &mut gpiob.crh, &mut gpioa.crh, &mut gpioa.crl, delay)
    };
  }

  fn init_trigger1(
    pb5: gpio::gpiob::PB5<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpiob::CRL
  ) -> Option<Trigger1Gpio> {
    let mut trigger = pb5.into_push_pull_output(crl);
    trigger.set_low().ok();
    return Some(trigger);
  }

  fn init_trigger2(
    pb4: gpio::gpiob::PB4<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpiob::CRL
  ) -> Option<Trigger2Gpio> {
    let mut trigger = pb4.into_push_pull_output(crl);
    trigger.set_low().ok();
    return Some(trigger);
  }

  fn init_trigger3(
    pb0: gpio::gpiob::PB0<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpiob::CRL
  ) -> Option<Trigger3Gpio> {
    let mut trigger = pb0.into_push_pull_output(crl);
    trigger.set_low().ok();
    return Some(trigger);
  }

  fn init_trigger4(
    pb1: gpio::gpiob::PB1<gpio::Input<gpio::Floating>>, 
    crl: &mut gpio::gpiob::CRL
  ) -> Option<Trigger4Gpio> {
    let mut trigger = pb1.into_push_pull_output(crl);
    trigger.set_low().ok();
    return Some(trigger);
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
      Config::default().baudrate(if cfg!(feature="debug") { 115200.bps() } else { 31250.bps() }),
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
      Config::default().baudrate( if cfg!(feature="clock_test") { 38400.bps() } else { 31250.bps() }),
      *clocks,
      apb1,
    );
    return Some(serial);
  }

  fn init_display(
    pa8: gpio::gpioa::PA8<gpio::Input<gpio::Floating>>,
    pb15: gpio::gpiob::PB15<gpio::Input<gpio::Floating>>,
    pb14: gpio::gpiob::PB14<gpio::Input<gpio::Floating>>,
    pb10: gpio::gpiob::PB10<gpio::Input<gpio::Floating>>,
    pa4: gpio::gpioa::PA4<gpio::Input<gpio::Floating>>,
    pa5: gpio::gpioa::PA5<gpio::Input<gpio::Floating>>,
    crhb: &mut gpio::gpiob::CRH,
    crha: &mut gpio::gpioa::CRH,
    crla: &mut gpio::gpioa::CRL,
    delay: Delay
  ) -> Option<DisplayPins> {

    let display: DisplayPins = DisplayPins{
      rs: pa8.into_push_pull_output(crha),
      enable: pb15.into_push_pull_output(crhb),
      d4: pb14.into_push_pull_output(crhb),
      d5: pb10.into_push_pull_output(crhb),
      d6: pa4.into_push_pull_output(crla),
      d7: pa5.into_push_pull_output(crla),
      delay: delay
    };

    return Some(display);
  }
}
