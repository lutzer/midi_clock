# Clock Analyzer

* upload clock_analyzer firmware on arduino uno
* connect the Rx port of the arduino with the Tx of the midi signal you want to analyze
* flash midi clock with `./flash clock_test`
* set serial port of arduino in *clock_analyzer.py* and run python script with: `python3 clock_analyzer.py`