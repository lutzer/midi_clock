use cortex_m::interrupt::{ Mutex };
use core::cell::{ RefCell };

use crate::SerialWriter;

#[cfg(feature = "debug")]
pub static G_SERIAL : Mutex<RefCell<Option<SerialWriter>>> = Mutex::new(RefCell::new(None));

#[macro_export]
macro_rules! debug {
  ($($rest:tt)*) => {
    #[cfg(feature = "debug")]
    debug_print($($rest)*)
  }
}

#[cfg(feature = "debug")]
pub fn debug_print(s: &str) {
  cortex_m::interrupt::free(|cs| {
    let mut serial = G_SERIAL.borrow(cs).borrow_mut();
    serial.as_mut().map(|w| {
      w.write(b'\n').ok();
      w.write_str(s).ok();
    });
  });
}

#[cfg(feature = "debug")]
pub fn debug_init(serial: SerialWriter) {
  cortex_m::interrupt::free(|cs| G_SERIAL.borrow(cs).replace(Some(serial)));
}
