#[derive(Copy, Clone, PartialEq)]
pub enum RunState {
  STOPPED,
  RUNNING,
  PAUSED
}

#[derive(Copy, Clone)]
pub struct State {
  pub bpm: u16,
  pub running: RunState,
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
        bpm: 30,
        running: RunState::RUNNING,
      },
      changed: false
    }
  }

  pub fn on_change(&mut self) -> Option<State> {
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

  pub fn button1_pressed(&mut self) {
    self.state.running = if self.state.running == RunState::STOPPED { RunState::RUNNING } else { RunState::STOPPED };
    self.changed = true;
  }
}