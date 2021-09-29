use crate::{CONTEXT};

use crate::utils::{u16_to_string, i16_to_string};

#[macro_export]
macro_rules! debug {
  ($($rest:tt)*) => {
    #[cfg(feature = "debug")]
    debug_print($($rest)*)
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
    let mut context = CONTEXT.borrow(cs).borrow_mut();
    context.as_mut().map(|ctx| {
      let serial =  &mut ctx.serial;
      serial.write_str(1,"[DEBUG] ").ok();
      serial.write_str(1,s.into_string()).ok();
      serial.write_str(1,"\n\r").ok();
    });
  });
}
