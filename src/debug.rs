use cortex_m::interrupt::{ Mutex };
use core::cell::{ RefCell };

use crate::SerialWriter;

use crate::utils::{u16_to_string, i16_to_string};

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
pub trait Stringable<'a> {
  fn into_string(self) -> &'a str;
}

#[cfg(feature = "debug")]
impl<'a> Stringable<'a> for &'a str {
  fn into_string(self) -> &'a str {
    return self
  }
}

#[cfg(feature = "debug")]
impl<'a> Stringable<'a> for u16 {
  fn into_string(self) -> &'a str {
    let s = u16_to_string(self);
    return s
  }
}

#[cfg(feature = "debug")]
impl<'a> Stringable<'a> for i16 {
  fn into_string(self) -> &'a str {
    let s = i16_to_string(self);
    return s
  }
}

#[cfg(feature = "debug")]
impl<'a> Stringable<'a> for bool {
  fn into_string(self) -> &'a str {
    return if self { "1" } else { "0" }
  }
}

#[cfg(feature = "debug")]
pub fn debug_print<'a, T>(s: T) where T : Stringable<'a> {
  cortex_m::interrupt::free(|cs| {
    let mut serial = G_SERIAL.borrow(cs).borrow_mut();
    serial.as_mut().map(|w| {
      w.write_str("[DEBUG] ").ok();
      w.write_str(s.into_string()).ok();
      w.write_str("\n\r").ok();
    });
  });
}

#[cfg(feature = "debug")]
pub fn debug_init(serial: SerialWriter) {
  cortex_m::interrupt::free(|cs| G_SERIAL.borrow(cs).replace(Some(serial)));
}
