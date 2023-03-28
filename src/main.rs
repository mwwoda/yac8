extern crate core;

use yac8::chip8::{Chip8};
use yac8::chip8_runner::{run_with_sdl};
use yac8::cli::load_from_cli;

pub fn main() -> Result<(), String> {
    let rom = load_from_cli();
    let mut chip8 = Chip8::default(rom);
    run_with_sdl(&mut chip8, 20);

    Ok(())
}