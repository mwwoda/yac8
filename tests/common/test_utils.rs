use std::path::PathBuf;

use yac8::chip8::{Chip8, Chip8Vram, Config};
use yac8::chip8_runner::run_stop_on_blocked;
use yac8::cli::load_rom_from_path;
use yac8::quirks::{SCHIP_QUIRKS, XOCHIP_QUIRKS};

pub fn dump_vram_when_blocked(data: &[u8], memory_start: u16) -> Chip8Vram {
    let mut chip8 = ch8_with_test_rom();
    chip8.load_to_memory(data, memory_start);
    run_stop_on_blocked(&mut chip8);
    chip8.vram
}

fn chip_with_test_rom(config: Config) -> Chip8 {
    let rom_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "roms", "chip8-test-suite.ch8"].iter().collect();
    let rom = load_rom_from_path(rom_path.to_str().unwrap());
    Chip8::new(rom, config)
}

pub fn ch8_with_test_rom() -> Chip8 {
    chip_with_test_rom(Config::ch8())
}

pub fn schip_with_test_rom() -> Chip8 {
    chip_with_test_rom(Config::new(false, SCHIP_QUIRKS))
}

pub fn xochip_with_test_rom() -> Chip8 {
    chip_with_test_rom(Config::new(false, XOCHIP_QUIRKS))
}