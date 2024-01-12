use std::ops::BitOr;

#[derive(Debug)]
pub enum IME {
    Disabled,
    Scheduled,
    Enabled,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum RegisterPair {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Flag {
    /// Zero flag
    Z,
    /// Subtraction flag
    N,
    /// Half-carry flag
    H,
    /// Carry flag
    C,
}

impl From<Flag> for u8 {
    fn from(value: Flag) -> Self {
        match value {
            Flag::Z => 0b10000000,
            Flag::N => 0b01000000,
            Flag::H => 0b00100000,
            Flag::C => 0b00010000,
        }
    }
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

impl BitOr<Flag> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Flag) -> Self::Output {
        self | u8::from(rhs)
    }
}

/// Returns true if any of the bits of `mask` in `v` are true.
#[inline]
pub fn get_bit<M: Into<u8>>(v: &u8, mask: M) -> bool {
    *v & mask.into() != 0
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

#[inline]
pub fn toggle_bit<M: Into<u8>>(v: &mut u8, mask: M) {
    *v ^= mask.into();
}

/// Returns (LSB, MSB).
#[inline]
pub fn split_u16(v: u16) -> (u8, u8) {
    (v as u8, (v >> 8) as u8)
}

#[inline]
pub fn join_u16(lsb: u8, msb: u8) -> u16 {
    (msb as u16) << 8 | (lsb as u16)
}
