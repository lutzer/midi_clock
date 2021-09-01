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
* for debugging connect seria adapter, see in [docs/hardware.md](docs/hardware.md)

## Flash Chip

* run `./flash`

## Debugging

* see debugging output with `screen /dev/tty.<adapter> 115200`, (Ctrl+A, K to close monitor)
