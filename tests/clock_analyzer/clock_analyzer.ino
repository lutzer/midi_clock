#define READINGS 1000


void setup() {
  Serial.begin(115200);

  Serial.println("start");
  delay(500);

  // clear serial buffer
  while (Serial.available()) {
    Serial.read();
  }

  for (int i=0;i<READINGS;i++) {
    while (Serial.read() < 0) { }
    Serial.println(micros());
  }
  Serial.println("end");
}

void loop() {

}
