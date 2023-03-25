//TODO macro
pub fn to_u8(b1: u8, b2: u8) -> u8 { ((b1) << 4) | b2 }

pub fn to_u16(b1: u8, b2: u8) -> u16 {
    ((b1 as u16) << 8) | b2 as u16
}

pub fn to_u16_from_three(b1: u8, b2: u8, b3: u8) -> u16 {
    ((b1 as u16) << 8) | ((b2 as u16) << 4) | b3 as u16
}

//TODO generic function
pub fn get_bit_at_u8(byte: u8, n: u8) -> bool {
    byte & (1 << n) != 0
}

pub fn get_bit_at_u16(byte: u16, n: u16) -> bool {
    byte & (1 << n) != 0
}
