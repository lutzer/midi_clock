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

pub fn send_midi_ctrl_msg(current: RunState, _: RunState) {
  interrupt::free(|cs| {
    Context::get_instance(cs, &|ctx| {
      match current {
        RunState::RUNNING => { 
          ctx.serial.write(2, MidiMessage::Continue as u8).ok(); 
        },
        RunState::PAUSED => { 
          ctx.serial.write(2, MidiMessage::Stop as u8).ok(); 
        },
        RunState::STOPPING => { 
          ctx.serial.write(2, MidiMessage::Stop as u8).ok(); 
        },
        RunState::STOPPED => { 
          ctx.serial.write(2, MidiMessage::Start as u8).ok();
          ctx.triggers.fire(TRIGGER4_MASK);
        }
      }
    });
  });
}