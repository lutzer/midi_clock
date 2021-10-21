# Hardware

## Bluepill pinout

![](images/pinout.png)

## Connections

| Pin  | Connection             |
| ---- | ---------------------- |
| A9   | Debug TX1             |
| A10  | Debug RX1              |
| PB12 | Button1     		        |
| PB13 | Button2     		        |
| PA7  | Button3     		        |
| PA6  | Button4 (Enc)          |
| PA0  | Enc1                   |
| PA1  | Enc2                   |
| PA2  | MIDI OUT1+2 (Hex Buffer) |
| PB5  | LED1 (MIDI OUT1) |
| PB10 | 2x MIDI OUT2+3 (Hex Buffer) |
| PB4  |  LED2 (MIDI OUT2) |
| PB0  | TRIGGER1 (Hex Buffer) & LED3 |
| PB1  | TRIGGER2 (Hex Buffer) & LED4 |

### Debugging with UART Adapter

Only connect TX to A9, RX to A10 and GND, no need for 3.3V connection

### BOM

* 74HC367 or DM7417N ?

