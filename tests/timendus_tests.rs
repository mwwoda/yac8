use crate::common::test_runner::dump_vram_after_blocked;
use crate::common::vram_dumps::{FLAGS, IBM_LOGO, OPCODES};

mod common;

#[test]
fn ibm_logo() {
    assert_eq!(dump_vram_after_blocked(&[1], 0x1FF), IBM_LOGO);
}

#[test]
fn opcodes() {
    assert_eq!(dump_vram_after_blocked(&[2], 0x1FF), OPCODES);
}

#[test]
fn flags() {
    assert_eq!(dump_vram_after_blocked(&[3], 0x1FF), FLAGS);
}