#[macro_export]
macro_rules! debug {
  ($($rest:tt)*) => {
    #[cfg(feature = "debug")]
    self::debug::debug_methods::debug_print($($rest)*)
  }
}

#[cfg(feature = "debug")] 
pub mod debug_methods {
  use crate::{CONTEXT};
  use crate::utils::{u16_to_string, i16_to_string, u32_to_string};
  use crate::statemachine::State;
  use core::str;

  pub trait Stringable<'a> {
    fn into_string(self) -> &'a str;
  }
  
  impl<'a> Stringable<'a> for &'a str {
    fn into_string(self) -> &'a str {
      return self
    }
  }
  
  impl<'a> Stringable<'a> for u16 {
    fn into_string(self) -> &'a str {
      let s = u16_to_string(self);
      return s
    }
  }

  impl<'a> Stringable<'a> for u32 {
    fn into_string(self) -> &'a str {
      let s = u32_to_string(self);
      return s
    }
  }
  
  impl<'a> Stringable<'a> for i16 {
    fn into_string(self) -> &'a str {
      let s = i16_to_string(self);
      return s
    }
  }
  
  impl<'a> Stringable<'a> for bool {
    fn into_string(self) -> &'a str {
      return if self { "1" } else { "0" }
    }
  }

  impl<'a> Stringable<'a> for State {
    fn into_string(self) -> &'a str {
      const BUFFER_LENGTH: usize = 16;
      static mut buffer: [u8;BUFFER_LENGTH] = [0; BUFFER_LENGTH];

      unsafe fn add_number(val: u16, i: &mut usize){
        let string = u16_to_string(val);
        buffer[*i..*i+string.len()].copy_from_slice(string.as_bytes());
        *i += string.len();
      }

      unsafe {
        let mut i = 0;
        add_number(self.bpm, &mut i);
        buffer[i] = ',' as u8; i += 1;
        add_number(self.running as u16, &mut i);
        buffer[i] = ',' as u8; i += 1;
        add_number(self.trigger_clock_multiplier as u16, &mut i);
        for i in i..BUFFER_LENGTH {
          buffer[i] = ' ' as u8;
        }
        return str::from_utf8_unchecked(&buffer);
      }
    }
  }

  // impl<'a> Stringable<'a> for State {
  //   fn into_string(self) -> &'a str {
  //     let bpm = u16_to_string(self.bpm);
  //     let s = concat!["run", bpm];
  //     return s;
  //   }
  // }
  
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
}


