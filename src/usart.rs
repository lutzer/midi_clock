use stm32f1xx_hal::{
  gpio::{gpioa, Alternate, PushPull, Input, Floating},
  serial::{Serial, Config},
  pac,
  prelude::*,
  rcc::{Rcc}
};
use crate::pac::{USART1};

struct Usart1Interface {
  serial: Serial<USART1, (gpioa::PA9<Alternate<PushPull>>, gpioa::PA10<Input<Floating>>)>
}

impl Usart1Interface {
  fn init(&self, dp: &pac::Peripherals, rcc: &Rcc, gpioa: &mut gpioa::Parts, afio: &stm32f1xx_hal::afio::Parts, clocks: &stm32f1xx_hal::rcc::Clocks) {
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // let mut serial = Serial::usart1(
    //   dp.USART1,
    //   (tx, rx),
    //   &mut afio.mapr,
    //   Config::default().baudrate(115200.bps()),
    //   *clocks,
    //   &mut rcc.apb2,
    // );
  }
}