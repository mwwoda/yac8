# yac8

![Screenshot](/cover.png)

yac8 is Yet Another Chip8 interpreter (or emulator if you prefer) written in Rust. The goal of this project is to make
as accurate interpreter as possible. It is also my first attempt
of writing a slightly more complex project in Rust, and also my first attempt at writing an emulator so I treat is as a
learning experience.

## Usage

[SDL2](https://github.com/Rust-SDL2/rust-sdl2) is required to run the desktop version\
You also need to obtain a game rom to play

Clone this repo and run the following command in the CLI

```bash
cargo run /path/to/rom
```

## Checklist

- [x] All opcodes supported
- [x] Default keyboard layout support
- [x] Run from CLI
- [x] [Timendus test suite](https://github.com/Timendus/chip8-test-suite)
    - [x] IBM Logo
    - [x] Corax89's opcode test
    - [x] Flags test
    - [ ] Quirks test
    - [x] Keypad test
- [ ] Sound support
- [ ] Configuration (clock speed, timers, keyboard layout etc.)
- [ ] Load rom from URL
- [ ] GUI
- [ ] WASM version
- [ ] Debugger
- [ ] Disassembler