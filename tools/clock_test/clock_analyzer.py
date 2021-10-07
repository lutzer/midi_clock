import serial as pyserial
import threading
import time
import numpy as np
from os import system

SERIAL_PORT = '/dev/tty.usbmodem142101'
BAUDRATE = 38400
BUFFER_SIZE = 25

buffer = [0 for i in range(BUFFER_SIZE)]
buffer_mutex = threading.Semaphore()

# producer thread reads serial lines and puts them into a buffer
def read_serial():
  global buffer, buffer_mutex, BUFFER_SIZE

  with pyserial.Serial(SERIAL_PORT, BAUDRATE) as serial:

    last_reading = 0
    buffer_index = 0

    while True:
      try:
        line = serial.readline()
        time = int(line)
        with buffer_mutex:
          buffer[buffer_index] = time - last_reading
          buffer_index = (buffer_index + 1) % BUFFER_SIZE
        last_reading = time
      except ValueError:
        pass

producer = threading.Thread(target=read_serial)
producer.start()

# main thread is the consumer that calculates the running average and prints it
while True:
  time_diffs = [0]
  with buffer_mutex:
    time_diffs = np.array(buffer)
  diffs = time_diffs[time_diffs > 0]
  system('clear')
  if len(diffs) > 0:
    mean = np.mean(diffs)
    dev = np.std(diffs)
    print("mean in us:   ", int(mean))
    print("std dev:      ", dev)
    print("freq:         ", 1000000/mean)
    print("bpm:          ", int(60000000/(mean*24)))
  time.sleep(0.5)
    


