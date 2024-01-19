use std::ops::{BitAnd, BitOr};

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
pub enum InterruptMask {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}
impl InterruptMask {
    pub fn get_handler_address(&self) -> u16 {
        match self {
            InterruptMask::VBlank => 0x0040,
            InterruptMask::LCDStat => 0x0048,
            InterruptMask::Timer => 0x0050,
            InterruptMask::Serial => 0x0058,
            InterruptMask::Joypad => 0x0060,
        }
    }

    pub fn get_interrupt_from_register(register: u8) -> Option<Self> {
        for interrupt in [
            Self::VBlank,
            Self::LCDStat,
            Self::Timer,
            Self::Serial,
            Self::Joypad,
        ] {
            if get_bit(register, interrupt) {
                return Some(interrupt);
            }
        }
        return None;
    }
}

impl From<InterruptMask> for u8 {
    fn from(value: InterruptMask) -> Self {
        match value {
            InterruptMask::VBlank  => 0b0000_0001,
            InterruptMask::LCDStat => 0b0000_0010,
            InterruptMask::Timer   => 0b0000_0100,
            InterruptMask::Serial  => 0b0000_1000,
            InterruptMask::Joypad  => 0b0001_0000,
        }
    }
}

impl BitOr<Self> for InterruptMask {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        u8::from(self) | u8::from(rhs)
    }
}

impl BitOr<u8> for InterruptMask {
    type Output = u8;

    fn bitor(self, rhs: u8) -> Self::Output {
        u8::from(self) | rhs
    }
}

impl BitOr<InterruptMask> for u8 {
    type Output = u8;

    fn bitor(self, rhs: InterruptMask) -> Self::Output {
        self | u8::from(rhs)
    }
}

/// Returns true if any of the bits of `mask` in `v` are true.
#[inline]
pub fn get_bit<B: BitAnd<B, Output = B> + PartialEq + From<u8>, M: Into<B>>(v: B, mask: M) -> bool {
    (v & mask.into()) != B::from(0)
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
