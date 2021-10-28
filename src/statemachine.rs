#[derive(Copy, Clone, PartialEq)]
pub enum RunState {
  STOPPED,
  STOPPING,
  RUNNING,
  PAUSED
}

#[derive(Copy, Clone, PartialEq)]
pub enum ClockSource {
  Internal,
  MidiIn,
  TriggerIn
}

#[derive(Copy, Clone)]
pub struct State {
  pub bpm: u16,
  pub clock_trigger_multiplier: u8, // multiply clock for both trigger outs
  pub clock_divisions: [u8; 2], // divisions for clock 0: midi out1+2, 1: midi out 2+3, 2: trigger1, 3: trigger2
  pub clock_bar_length: u8, // how many quarters per bar for resync
  pub clock_sync: bool,
  pub clock_source: ClockSource,
  pub running: RunState, // run state of the clock
}

pub struct Statemachine {
  state: State,
  changed: bool
}

// define state constants
const BPM_RANGE: (u16,u16) = (30, 320);
const DIVISION_STEPS: [u8;10] = [1,2,3,4,5,6,7,8,16,32]; // largest common multiple is 33600
const MULTIPLIERS: [u8;8] = [1,2,3,4,6,8,12,24];
const BAR_LENGTHS_RANGE: (u8,u8) = (1,15);

impl Statemachine {
  pub fn new() -> Statemachine {
    // set initial state
    return Statemachine { 
      state : State {
        bpm: 120,
        clock_trigger_multiplier: 4,
        clock_divisions: [1,4],
        clock_bar_length: 4,
        clock_sync: false,
        clock_source: ClockSource::Internal,
        running: RunState::RUNNING
      },
      changed: true
    }
  }

  pub fn on_change(&mut self) -> Option<State> {
    if self.changed {
      let state = self.state.clone();

      self.changed = false;

      return Some(state);
    } else {
      return None;
    }
  }

  pub fn get_state(&self) -> State {
    return self.state;
  }

  pub fn encoder_turn(&mut self, steps: i16) {
    let bpm = ((self.state.bpm as i16) + steps) as u16;
    self.state.bpm = bpm.min(BPM_RANGE.1).max(BPM_RANGE.0);
    self.changed = true;
  }

  pub fn button1_pressed(&mut self, pressed : bool) {
    if pressed {
      self.state.running = if self.state.running != RunState::RUNNING { RunState::RUNNING } else { RunState::PAUSED };
      self.changed = true;
    }
  }

  pub fn button2_pressed(&mut self, pressed : bool) {
    self.state.running = if pressed { RunState::STOPPING } else { RunState::STOPPED };
    self.changed = true;
  }

  pub fn button3_pressed(&mut self, pressed : bool) {
    if pressed {
      self.state.clock_sync = true;
    } else {
      self.state.clock_sync = false;
    }
    self.changed = true;
  }

  pub fn encoder_pressed(&mut self, _pressed : bool) {

  }
}