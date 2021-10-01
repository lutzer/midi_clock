use numtoa::NumToA;
use cortex_m::interrupt::{ CriticalSection };
use core::cell::{ UnsafeCell };

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

/* Struct holds a thread safe value to be shared between interrupts */
pub struct CSCell<T>( UnsafeCell<T> );
impl<T> CSCell<T> {
  pub const fn new(value: T) -> CSCell<T> {
    return CSCell(UnsafeCell::new(value));
  }

  pub fn set(&self, value: T, _: &CriticalSection ) {
    unsafe {
      (*self.0.get()) = value;
    }
  }


  pub fn get(&self, _: &CriticalSection) -> &mut T {
    unsafe {
      return &mut *self.0.get();
    }
  }

  pub unsafe fn set_unsafe(&self, value: T) {
    (*self.0.get()) = value;
  }

  pub unsafe fn get_unsafe(&self) -> &mut T {
    return &mut *self.0.get();
  }

}
unsafe impl<T> Sync for CSCell<T> {}