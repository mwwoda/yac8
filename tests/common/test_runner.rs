use std::path::PathBuf;

use yac8::chip8::{Chip8, Chip8Vram};
use yac8::cli::load_rom_from_path;

pub fn dump_vram_after_blocked(data: &[u8], memory_start: u16) -> Chip8Vram {
    let mut chip8 = load_test_suite();
    chip8.load_to_memory(data, memory_start);
    run_stop_on_blocked(chip8)
}

fn load_test_suite() -> Chip8 {
    let rom_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "roms", "chip8-test-suite.ch8"].iter().collect();
    let rom = load_rom_from_path(rom_path.to_str().unwrap());
    Chip8::new(rom)
}

fn run_stop_on_blocked(mut chip8: Chip8) -> Chip8Vram {
    loop {
        chip8.decrement_delay_timer();
        chip8.decrement_sound_timer();
        if chip8.blocked {
            return chip8.vram;
        }

        let instruction = chip8.fetch();
        chip8.handle_op_code(instruction, None);
    }
}