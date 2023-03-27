use num::{PrimInt, ToPrimitive};

pub fn to_u8(b1: u8, b2: u8) -> u8 { ((b1) << 4) | b2 }

#[macro_export]
macro_rules! to_u16 {
    ($b1:expr, $b2:expr) => {
       (($b1 as u16) << 8) | $b2 as u16
    };
    ($b1:expr, $b2:expr, $b3:expr) => {
        (($b1 as u16) << 8) | (($b2 as u16) << 4) | $b3 as u16
    }
}

pub fn get_bit_at<T: PrimInt + ToPrimitive>(byte: T, n: T) -> bool {
    byte & (T::one() << n.to_usize().unwrap()) != T::zero()
}