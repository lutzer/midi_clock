use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::interrupt::{ CriticalSection };

use crate::triggers::*;
use crate::serial::*;

pub static CONTEXT: CSContext = CS_CONTEXT_INIT;

// holds structs that need to be globally accesible (that are called by interrupt functions)
pub struct Context {
  pub triggers: Triggers,
  pub serial: SerialWriter
}

pub type CSContext<'a> = Mutex<RefCell<Option<Context>>>;
pub const CS_CONTEXT_INIT: CSContext = Mutex::new(RefCell::new(None));

impl Context {
  pub fn get_instance(cs: &CriticalSection, func: &dyn Fn(&mut Context)) {
    let mut context = CONTEXT.borrow(cs).borrow_mut();
    context.as_mut().map(|ctx| func(ctx));
  }
}
