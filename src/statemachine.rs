#[derive(Copy, Clone, PartialEq)]
pub enum RunState {
  STOPPED,
  RESTART,
  RUNNING,
  PAUSED,
  SYNC
}

#[derive(Copy, Clone)]
pub struct State {
  pub bpm: u16,
  pub trigger_clock_multiplier: u8, // multiply clock for both trigger outs
  pub clock_divisions: [u8; 2], // divisions for clock 0: midi out1+2, 1: midi out 2+3, 2: trigger1, 3: trigger2
  pub running: RunState // run state of the clock
}

pub struct Statemachine {
  state: State,
  changed: bool
}

// define state constants
const MIN_BPM: u16 = 30;
const MAX_BPM: u16 = 320;
const DIVISION_STEPS: [u8;10] = [1,2,3,4,5,6,7,8,16,32]; // lcm is 33600
const MULTIPLIERS: [u8;8] = [1,2,3,4,6,8,12,24];

impl Statemachine {
  pub fn new() -> Statemachine {
    // set initial state
    return Statemachine { 
      state : State {
        bpm: 320,
        trigger_clock_multiplier: 24,
        clock_divisions: [1,4],
        running: RunState::RUNNING
      },
      changed: true
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
    let bpm = ((self.state.bpm as i16) + steps) as u16;
    self.state.bpm = bpm.min(MAX_BPM).max(MIN_BPM);
    self.changed = true;
  }

  pub fn button1_pressed(&mut self, pressed : bool) {
    if pressed {
      self.state.running = if self.state.running != RunState::RUNNING { RunState::RUNNING } else { RunState::PAUSED };
      self.changed = true;
    }
  }

  pub fn button2_pressed(&mut self, pressed : bool) {
    if pressed {
      self.state.running = RunState::STOPPED;
    } else {
      self.state.running = RunState::RESTART;
    }
   self.changed = true;
  }

  pub fn button3_pressed(&mut self, pressed : bool) {
    if pressed {
      self.state.running = RunState::SYNC;
      self.changed = true;
    }
  }

  pub fn encoder_pressed(&mut self, _pressed : bool) {

  }
}