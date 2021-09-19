use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use crate::triggers::*;
use crate::serial::*;

// holds structs that need to be globally accesible (that are called by interrupt functions)
pub struct Context {
  pub triggers: Triggers,
  pub serial: SerialWriter
}

pub type CSContext = Mutex<RefCell<Option<Context>>>;
pub const CS_CONTEXT_INIT: CSContext = Mutex::new(RefCell::new(None));
