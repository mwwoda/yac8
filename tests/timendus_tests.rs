use yac8::chip8_runner::run_stop_on_blocked;

use crate::common::test_utils::{ch8_with_test_rom, dump_vram_when_blocked};
use crate::common::vram_dumps::{FLAGS, IBM_LOGO, OPCODES, QUIRKS_CH8};

mod common;

#[test]
fn ibm_logo() {
    assert_eq!(dump_vram_when_blocked(&[1], 0x1FF), IBM_LOGO);
}

#[test]
fn opcodes() {
    assert_eq!(dump_vram_when_blocked(&[2], 0x1FF), OPCODES);
}

#[test]
fn flags() { assert_eq!(dump_vram_when_blocked(&[3], 0x1FF), FLAGS); }

#[test]
fn quirks_ch8() {
    let mut chip8 = ch8_with_test_rom();
    chip8.load_to_memory(&[4], 0x1ff);
    chip8.load_to_memory(&[1], 0x1fe);
    assert_eq!(run_stop_on_blocked(chip8), QUIRKS_CH8);
}