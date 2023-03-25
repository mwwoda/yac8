extern crate core;

use crate::chip8::Chip8;
use crate::chip8_runner::run_with_sdl;
use crate::cli::load_from_cli;

mod graphics;
mod chip8;
mod bit_ops;
mod input;
mod chip8_runner;
mod cli;

pub fn main() -> Result<(), String> {
    let rom = load_from_cli();
    let chip8 = Chip8::new(rom);
    run_with_sdl(chip8, 20);

    Ok(())
}
