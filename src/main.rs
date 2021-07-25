//main.rs

#![no_std]
#![no_main]

use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;

use stm32f1xx_hal::{
    delay::Delay, 
    pac, 
    prelude::*, 
    serial::{Serial, Config}
};

#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    // access PGIOC and PGIOB registers and prepare the alternate function I/O registers
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // set clock frequency to internal 8mhz oscillator
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    // configure pc13 as output
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    //usart1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;
    let mut serial = Serial::usart1(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        clocks,
        &mut rcc.apb2,
    );

    // uses System Timer peripheral to create delay function
    let mut delay = Delay::new(cp.SYST, clocks);

    // Now, enjoy the lightshow!
    loop {
        led.set_high().ok();
        delay.delay_ms(300 as u32);
        led.set_low().ok();
        delay.delay_ms(2000 as u32);
        let sent = b'T';
        serial.write(sent).ok();
    }
}