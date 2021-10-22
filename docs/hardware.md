# Hardware

## Bluepill pinout

![](images/pinout.png)

## Connections

| Pin  | Connection                                |
| ---- | ----------------------------------------- |
| A9   | Debug TX1                                 |
| A10  | Debug RX1 / MIDI IN                       |
| PB12 | Button1     		                           |
| PB13 | Button2     		                           |
| PA7  | Button3     		                           |
| PA6  | Button4 (Enc)                             |
| PA0  | Enc1                                      |
| PA1  | Enc2                                      |
| PA2  | MIDI OUT1+2 (Hex Buffer1+2)               |
| PB5  | TRIGGER1 (LED MIDI OUT1+2)                |
| PB10 | MIDI OUT3+4 (Hex Buffer3+4)               |
| PB4  | TRIGGER2 (LED MIDI OUT3+4)                |
| PB0  | TRIGGER3 (Hex Buffer5) & LED3             |
| PB1  | TRIGGER4 RESET (Hex Buffer6) & LED4       |
| PA11 | USB D-                                    |
| PA12 | USB D+                                    |

### Debugging with UART Adapter

Only connect TX to A9, RX to A10 and GND, no need for 3.3V connection

### BOM

* 74HC367 or DM7417N ?

