use stm32f1xx_hal::{
  gpio::{gpioa}
  serial::{Serial, Config}
};

struct SerialInterface {
  serial: Serial
}

impl Usart1Interface {
  fn init(&self, gpioa: &gpioa::Parts, afio: &stm32f1xx_hal::afio, clocks: &stm32f1xx_hal::rcc::Clocks) {
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    self.serial = Serial::usart1(
      dp.USART1,
      (tx, rx),
      &mut afio.mapr,
      Config::default().baudrate(115200.bps()),
      clocks,
      &mut rcc.apb2,
    );
  }
}