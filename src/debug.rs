use cortex_m::interrupt::{ Mutex };
use core::cell::{ RefCell };
use numtoa::NumToA;

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

#[macro_export]
macro_rules! debug_init {
  ($($serial:tt)*) => {
    #[cfg(feature = "debug")]
    debug_init($($serial)*)
  }
}

#[cfg(feature = "debug")]
pub fn debug_print(s: &str) {
  cortex_m::interrupt::free(|cs| {
    let mut serial = G_SERIAL.borrow(cs).borrow_mut();
    serial.as_mut().map(|w| {
      w.write_str("[DEBUG] ").ok();
      w.write_str(s).ok();
      w.write_str("\n\r").ok();
    });
  });
}

#[cfg(feature = "debug")]
pub fn debug_init(serial: SerialWriter) {
  cortex_m::interrupt::free(|cs| G_SERIAL.borrow(cs).replace(Some(serial)));
}

pub fn num_to_string<'a>(number: u16) -> &'a str {
  static mut STRING_BUFFER : [u8; 4] = [0; 4];
  unsafe { 
    STRING_BUFFER = [0; 4];
    return number.numtoa_str(10, &mut STRING_BUFFER); 
  }
}
