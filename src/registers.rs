use num::ToPrimitive;

pub struct Registers {
    registers: [u8; 16],
    pub i: u16,
}

impl Registers {
    pub fn default() -> Self {
        Registers {
            registers: [0; 16],
            i: 0,
        }
    }

    pub fn get<T: ToPrimitive>(&self, x: T) -> u8 {
        self.registers[x.to_usize().unwrap()]
    }

    pub fn set<T: ToPrimitive>(&mut self, x: T, val: u8) {
        self.registers[x.to_usize().unwrap()] = val;
    }

    pub fn or<T: ToPrimitive>(&mut self, x: T, y: T) {
        self.registers[x.to_usize().unwrap()] |= self.get(y);
    }

    pub fn and<T: ToPrimitive>(&mut self, x: T, y: T) {
        self.registers[x.to_usize().unwrap()] &= self.get(y);
    }

    pub fn xor<T: ToPrimitive>(&mut self, x: T, y: T) {
        self.registers[x.to_usize().unwrap()] ^= self.get(y);
    }

    pub fn shift_left<T: ToPrimitive>(&mut self, x: T, val: u8) {
        self.registers[x.to_usize().unwrap()] <<= val;
    }

    pub fn shift_right<T: ToPrimitive>(&mut self, x: T, val: u8) {
        self.registers[x.to_usize().unwrap()] >>= val;
    }

    pub fn set_vf(&mut self, val: u8) {
        self.set(0xf, val);
    }
}