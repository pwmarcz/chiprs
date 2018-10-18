# `chiprs` - a Rust Chip-8 emulator

## Directory structure

* [`chiprs`](chiprs) - emulator engine
* [`chiprs-sdl`](chiprs) - SDL emulator
* [`games`] - game ROMs, taken from http://devernay.free.fr/hacks/chip8/

## Usage

You likely need `libsdl2-dev` installed.

    cd chiprs-sdl
    cargo run ../games/<game>.ch8

## License

By Paweł Marczewski <pwmarcz@gmail.com>.

Licensed under MIT (see [`LICENSE`](LICENSE)), except the `games` directory.
