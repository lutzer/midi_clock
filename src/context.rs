use core::cell::UnsafeCell;
use cortex_m::interrupt::{CriticalSection};
use core::mem::MaybeUninit;

use crate::triggers::*;

pub struct Context {
  pub triggers: Triggers
}


pub type CSContext = MaybeUninit<Context>;

pub const CS_CONTEXT_INIT: CSContext = MaybeUninit::uninit();

// impl CSContext {
//   pub fn set(&self, context: Context, _cs: &CriticalSection) {
//     unsafe { *self.0.get() = Some(context) };
//   }

//   pub fn get(&self, _cs: &CriticalSection) -> &mut Option<Context> {
//     let context = unsafe { self.0.get().as_mut().unwrap() };
//     return context;
//   }
// }

// unsafe impl Sync for CSContext {}