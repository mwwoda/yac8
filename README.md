# yac8

![Screenshot](/cover.png)

yac8 is Yet Another [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) interpreter (or emulator if you prefer) written in
Rust. I started this project to learn
more about emulator development in general. It's also my first attempt of writing a slightly more complex project in
Rust.

## Usage

[SDL2](https://www.libsdl.org) is required to run the desktop version. On Windows you need to put SDL2.dll in the root
directory.\
You also need to obtain a game rom to play.

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
    - [x] Quirks test
    - [x] Keypad test
- [ ] Sound support
- [ ] Configuration (clock speed, timers, keyboard layout etc.)
- [ ] Load rom from URL
- [ ] WASM version
- [ ] GUI
- [ ] Debugger
- [ ] Disassembler