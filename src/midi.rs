// #[derive(Copy,Clone)]
pub enum MidiMessage {
  Start = 0xFA,
  TimingClock = 0xF8,
  Continue = 0xFB,
  Stop = 0xFC,
  Reset = 0xFF
}

// #[derive(Copy,Clone)]
// pub enum MidiMessage {
//   Start = 0x30,
//   TimingClock = 0x31,
//   Continue = 0x32,
//   Stop = 0x33
// }

// pub fn generate_midi_msg<'a>(msg: MidiMessage, data1: Option<u8>, data2: Option<u8>) -> u8 {
//   return msg as u8;
// }