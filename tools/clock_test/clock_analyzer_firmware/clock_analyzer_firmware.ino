/* 
 *  This script sends the timings in us of the clock pulses it received on its serial port.
 *  Runs on arduino uno. Connect Rx pin with midi pin and GND with GND.
 *
 */


void setup() {
  Serial.begin(38400);

  Serial.println("start");
  delay(500);

  // clear serial buffer
  while (Serial.available()) {
    Serial.read();
  }
}

void loop() {
  while (Serial.read() < 0) { }
  Serial.println(micros());
}
