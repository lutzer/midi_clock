# Midi Clock

## Requirements

* install rust
  * run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install rust toolchain
  * use nightly toolchain: `rustup default nightly` (for heap allocator)
  * install target `rustup target install thumbv7m-none-eabi`
  * install cargo-edit `cargo install cargo-edit`, it allows to add dependencies with `cargo add <package>`

* install flash toolchain
  * install stlink on mac with `brew install stlink`
  * install *binutils-arm-none-eabi* with `brew tap PX4/homebrew-px4; brew install gcc-arm-none-eabi`


## Connect Board

* disconnect external power supply
* connect programmer *ST-Link V2* to programming pins of blue pill. Connect pins *SWDIO, GND, SWCLK, and 3.3V and RST to pin R*
* check if programmer is avalaible with `st-info --probe`

## Flash Chip

* run `./flash`
* compile with features with `./flash "feature1,feature2,..."`

## Debugging

* for debugging connect serial adapter, see in [docs/hardware.md](docs/hardware.md)
* flash chip with debug feature: `./flash debug`
* see debugging output with `screen /dev/tty.<adapter> 115200`, (Ctrl+A, K to close monitor)
* for analyzing the timings of the midi clock see [tools/clock_test/README.md](tools/clock_test/README.md)

## Links

* Midi specifications: https://www.midi.org/specifications-old/item/table-1-summary-of-midi-message
* STM32f103 Reference Manual: https://www.st.com//content/ccc/resource/technical/document/reference_manual/59/b9/ba/7f/11/af/43/d5/CD00171190.pdf/files/CD00171190.pdf/jcr:content/translations/en.CD00171190.pdf
* LCD 0802A: http://www.farnell.com/datasheets/50552.pdf
* Timer registers for stm32f103: https://docs.rs/stm32f1/0.14.0/stm32f1/stm32f103/tim2/index.html
