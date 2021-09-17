#[derive(Copy, Clone)]
pub struct State {
  pub bpm: u16
}

pub struct Statemachine {
  state: State,
  changed: bool
}

impl Statemachine {
  pub fn new() -> Statemachine {
    // set initial state
    return Statemachine { 
      state : State {
        bpm: 30
      },
      changed: false
    }
  }

  pub fn update(&mut self) -> Option<State> {
    if self.changed {
      self.changed = false;
      return Some(self.state);
    } else {
      return None;
    }
  }

  pub fn get_state(&self) -> State {
    return self.state;
  }

  pub fn encoder_turn(&mut self, steps: i16) {
    self.state.bpm = ((self.state.bpm as i16) + steps) as u16;
    self.changed = true;
  }
}