use std::path::PathBuf;

use yac8::chip8::{Chip8, Chip8Vram};
use yac8::chip8_runner::run_stop_on_blocked;
use yac8::cli::load_rom_from_path;

pub fn dump_vram_when_blocked(data: &[u8], memory_start: u16) -> Chip8Vram {
    let mut chip8 = ch8_with_test_rom();
    chip8.load_to_memory(data, memory_start);
    run_stop_on_blocked(chip8)
}

pub fn ch8_with_test_rom() -> Chip8 {
    let rom_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "roms", "chip8-test-suite.ch8"].iter().collect();
    let rom = load_rom_from_path(rom_path.to_str().unwrap());
    Chip8::new(rom)
}