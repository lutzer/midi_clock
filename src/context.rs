use core::cell::UnsafeCell;
use cortex_m::interrupt::{CriticalSection};
use core::mem::MaybeUninit;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use crate::triggers::*;

pub struct Context {
  pub triggers: Triggers
}


pub type CSContext = Mutex<RefCell<Option<Context>>>;

pub const CS_CONTEXT_INIT: CSContext = Mutex::new(RefCell::new(None));

// impl CSContext {
//   pub fn set(&self, context: Context, _cs: &CriticalSection) {
//     unsafe { *self.0.write(context) };
//   }

//   pub fn get(&self, _cs: &CriticalSection) -> &mut Context {
//     let context = unsafe { &mut *self.0.as_mut_ptr() };
//     return context;
//   }
// }

// unsafe impl Sync for CSContext {}