// #[derive(Copy,Clone)]
pub enum MidiMessage {
  Start = 0xFA,
  TimingClock = 0xF8,
  Continue = 0xFB,
  Stop = 0xFC,
  // Reset = 0xFF
}