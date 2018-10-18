# `chiprs` - a Rust Chip-8 emulator

By Paweł Marczewski <pwmarcz@gmail.com>.

## Directory structure

* [`chiprs`](chiprs) - emulator engine
* [`chiprs-sdl`](chiprs) - SDL emulator
* [`games`] - game ROMs, taken from http://devernay.free.fr/hacks/chip8/

## Usage

You likely need `libsdl2-dev` installed.

    cd chiprs-sdl
    cargo run ../games/<game>.ch8
