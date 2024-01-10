use std::ops::BitOr;

use num_enum::IntoPrimitive;

#[derive(Debug)]
pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

#[derive(Debug, IntoPrimitive)]
#[repr(u8)]
pub enum Flag {
    /// Zero flag
    Z = 0b10000000,
    /// Subtraction flag
    N = 0b01000000,
    /// Half-carry flag
    H = 0b00100000,
    /// Carry flag
    C = 0b00010000,
}

impl BitOr<Self> for Flag {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        u8::from(self) | u8::from(rhs)
    }
}

impl BitOr<u8> for Flag {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        u8::from(self) | rhs
    }
}

/// Sets all bits of `v` in the `mask` to `state`.
#[inline]
pub fn set_bit<M: Into<u8>>(v: &mut u8, mask: M, state: bool) {
    if state {
        *v |= mask.into();
    } else {
        *v &= !mask.into();
    }
}

/// Returns true if any of the bits of `mask` in `v` are true.
#[inline]
pub fn get_bit<M: Into<u8>>(v: &u8, mask: M) -> bool {
    *v & mask.into() != 0
}

#[inline]
pub fn split_u16(v: u16) -> (u8, u8) {
    ((v >> 8) as u8, v as u8)
}

#[inline]
pub fn join_u16(high: u8, low: u8) -> u16 {
    (high as u16) << 8 | (low as u16)
}
