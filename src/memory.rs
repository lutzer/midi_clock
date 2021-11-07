use crate::eeprom::{Eeprom};
use crate::statemachine::{State, ClockSource, RunState, DEFAULT_STATE};

use crate::debug;

const BPM_ADDRESS: u16 = 0x0004;

#[derive(Debug, Eq, PartialEq)]
pub enum MemoryError {
  ReadError
}

pub struct Memory {
  eeprom: Eeprom
}

impl Memory {
  pub fn new(eeprom: Eeprom) -> Memory {
    return Memory {
      eeprom: eeprom
    }
  }

  pub fn load_state(&mut self) -> Option<State> {
    debug!("load state");
    let bpm = self.eeprom.read_u16(BPM_ADDRESS).ok()?;
    let mut state = DEFAULT_STATE;
    state.bpm = bpm;
    return Some(state);
  }

  pub fn write_state(&mut self, state: &State) -> Result<(), MemoryError> {
    debug!("store state");
    return self.eeprom.write_u16(BPM_ADDRESS, state.bpm).map_err(|_| MemoryError::ReadError);
  }
}