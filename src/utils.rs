use numtoa::NumToA;

// pub fn min(n1: u8, n2: u8) -> u8  {
//   if n1 > n2 {
//     return n1
//   } else {
//     return n2
//   }
// }

pub fn u16_to_string<'a>(number: u16) -> &'a str {
  static mut STRING_BUFFER : [u8; 5] = [0; 5];
  unsafe { 
    STRING_BUFFER = [0; 5];
    return number.numtoa_str(10, &mut STRING_BUFFER); 
  }
}

pub fn i16_to_string<'a>(number: i16) -> &'a str {
  static mut STRING_BUFFER : [u8; 5] = [0; 5];
  unsafe { 
    STRING_BUFFER = [0; 5];
    return number.numtoa_str(10, &mut STRING_BUFFER); 
  }
}