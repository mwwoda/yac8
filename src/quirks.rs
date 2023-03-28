use crate::chip8::VBLank;
use crate::registers::Registers;

pub const CH8_QUIRKS: Quirks = Quirks {
    vf_reset: true,
    memory: true,
    shifting: false,
    jumping: false,
    display_wait: true,
};

pub struct Quirks {
    vf_reset: bool,
    memory: bool,
    shifting: bool,
    jumping: bool,
    display_wait: bool,
    //TODO clipping
    //clipping: bool,
}

impl Quirks {
    pub fn vf_reset(&self, registers: &mut Registers) {
        if self.vf_reset { registers.set_vf(0) }
    }

    pub fn memory(&self, registers: &mut Registers, x: u16) {
        if self.memory { registers.i += x + 1 }
    }

    pub fn shifting(&self, registers: &mut Registers, x: u8, y: u8) {
        if !self.shifting { registers.set(x, registers.get(y)) }
    }

    pub fn jumping(&self, registers: &mut Registers, n: u8) -> u8 {
        if self.jumping { registers.get(n) } else { registers.get(0) }
    }

    pub fn display_wait(&self, vblank: &mut VBLank) -> bool {
        if !self.display_wait { return false; }

        match vblank {
            VBLank::WaitForDraw => {
                *vblank = VBLank::WaitForInterrupt;
                true
            }
            VBLank::WaitForInterrupt => {
                true
            }
            VBLank::Free => {
                *vblank = VBLank::WaitForDraw;
                false
            }
        }
    }
}