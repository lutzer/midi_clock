// use cortex_m::interrupt::{self, Mutex};

type TimerCallback<'a> = &'a mut dyn FnMut();

const MAX_CALLBACKS: usize = 5;

pub struct Timers<'a> {
  handlers: [Option<TimerCallback<'a>>; MAX_CALLBACKS]
}

impl<'a> Timers<'a> {
  pub fn new() -> Timers<'a> {
    return Timers {
      handlers: Default::default()
    };
  }

  pub fn add_handler(&mut self, cb: TimerCallback<'a>) {
    static mut NUMBER_OF_CALLBACKS: usize = 0;
    unsafe {
      self.handlers[NUMBER_OF_CALLBACKS] = Some(cb);
      NUMBER_OF_CALLBACKS += 1;
    }
  }

  pub fn emit(&mut self) {
    for i in 0..MAX_CALLBACKS {
      self.handlers[i].as_mut().map(|f| f() );
    }
  }
}