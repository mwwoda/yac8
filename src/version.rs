use crate::registers::Registers;

// for quirks
pub trait ChipVersion {
    fn handle_vf(&self, registers: &mut Registers);
    fn handle_i(&self, registers: &mut Registers, x: u16);
}

pub struct Chip8Ver {}

impl ChipVersion for Chip8Ver {
    fn handle_vf(&self, registers: &mut Registers) { registers.set_vf(0) }
    fn handle_i(&self, registers: &mut Registers, x: u16) { registers.i += x + 1 }
}