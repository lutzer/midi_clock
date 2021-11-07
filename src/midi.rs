use crate::statemachine::{RunState};
use cortex_m::interrupt;
use crate::context::{Context};
use crate::triggers::{TRIGGER4_MASK};

// #[derive(Copy,Clone)]
pub enum MidiMessage {
  Start = 0xFA,
  TimingClock = 0xF8,
  Continue = 0xFB,
  Stop = 0xFC,
  // Reset = 0xFF
}