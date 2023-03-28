use yac8::chip8_runner::{run_for_cycles, run_stop_on_blocked};

use crate::common::test_utils::{ch8_with_test_rom, dump_vram_when_blocked, schip_with_test_rom, xochip_with_test_rom};
use crate::common::vram_dumps::{EX9E_NOT_PRESSED, EXA1_NOT_PRESSED, FLAGS, IBM_LOGO, OPCODES, QUIRKS_CH8, QUIRKS_SCHIP, QUIRKS_XOCHIP};

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
fn flags() {
    assert_eq!(dump_vram_when_blocked(&[3], 0x1FF), FLAGS);
}

//TODO macro
#[test]
fn quirks_ch8() {
    let mut chip8 = ch8_with_test_rom();
    chip8.load_to_memory(&[4], 0x1ff);
    chip8.load_to_memory(&[1], 0x1fe);
    run_stop_on_blocked(&mut chip8);
    assert_eq!(chip8.vram, QUIRKS_CH8);
}

#[test]
fn quirks_schip() {
    let mut super_chip = schip_with_test_rom();
    super_chip.load_to_memory(&[4], 0x1ff);
    super_chip.load_to_memory(&[2], 0x1fe);
    run_stop_on_blocked(&mut super_chip);
    assert_eq!(super_chip.vram, QUIRKS_SCHIP);
}

#[test]
fn quirks_xochip() {
    let mut xo_chip = xochip_with_test_rom();
    xo_chip.load_to_memory(&[4], 0x1ff);
    xo_chip.load_to_memory(&[3], 0x1fe);
    run_stop_on_blocked(&mut xo_chip);
    assert_eq!(xo_chip.vram, QUIRKS_XOCHIP);
}

//TODO test with pressed keys
#[test]
fn ex9e_not_pressed() {
    let mut chip8 = ch8_with_test_rom();
    chip8.load_to_memory(&[5], 0x1ff);
    chip8.load_to_memory(&[1], 0x1fe);
    run_for_cycles(&mut chip8, 1000);
    assert_eq!(chip8.vram, EX9E_NOT_PRESSED);
}

//TODO test with pressed keys
#[test]
fn exa1_not_pressed() {
    let mut chip8 = ch8_with_test_rom();
    chip8.load_to_memory(&[5], 0x1ff);
    chip8.load_to_memory(&[2], 0x1fe);
    run_for_cycles(&mut chip8, 1000);
    assert_eq!(chip8.vram, EXA1_NOT_PRESSED);
}