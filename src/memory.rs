use crate::eeprom::{Eeprom};
use crate::statemachine::{State, ClockSource, RunState};

const BPM_ADDRESS: u16 = 0x0004;

#[derive(Debug, Eq, PartialEq)]
pub enum MemoryError {
  WriteError,
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
    let bpm = self.eeprom.read_byte(BPM_ADDRESS).unwrap();
    return Some(State {
      bpm: bpm as u16,
      clock_trigger_multiplier: 4,
      clock_divisions: [1,4],
      clock_bar_length: 4,
      clock_sync: false,
      clock_source: ClockSource::Internal,
      running: RunState::RUNNING
    })
  }

  pub fn write_state(&mut self, state: &State) -> Result<(), MemoryError> {
    return self.eeprom.write_byte(BPM_ADDRESS, state.bpm as u8).map_err(|e| MemoryError::ReadError);
  }
}