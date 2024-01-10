use super::utils::{get_bit, join_u16, set_bit, RegisterPair};
use crate::gb::utils::{split_u16, Register};

/// [pandocs](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
#[derive(Debug)]
pub struct CPU {
    /// Accumulator / register A
    a: u8,
    /// Flags
    f: u8,
    /// Register B
    b: u8,
    /// Register C
    c: u8,
    /// Register D
    d: u8,
    /// Register E
    e: u8,
    /// Register H
    h: u8,
    /// Register L
    l: u8,
    /// Program counter/pointer
    pc: u16,
    /// Stack pointer
    sp: u16,
}

impl CPU {
    pub fn new_init() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn get_register(&mut self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::F => self.f,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    pub fn get_register_pair(&mut self, reg_pair: RegisterPair) -> u16 {
        match reg_pair {
            RegisterPair::AF => join_u16(self.a, self.f),
            RegisterPair::BC => join_u16(self.b, self.c),
            RegisterPair::DE => join_u16(self.d, self.e),
            RegisterPair::HL => join_u16(self.h, self.l),
            RegisterPair::PC => self.pc,
            RegisterPair::SP => self.sp,
        }
    }

    pub fn get_flag<F: Into<u8>>(&self, flag: F) -> bool {
        get_bit(&self.f, flag)
    }

    pub fn set_register(&mut self, reg: Register, v: u8) {
        match reg {
            Register::A => self.a = v,
            Register::F => self.f = v,
            Register::B => self.b = v,
            Register::C => self.c = v,
            Register::D => self.d = v,
            Register::E => self.e = v,
            Register::H => self.h = v,
            Register::L => self.l = v,
        }
    }

    pub fn set_register_pair(&mut self, reg_pair: RegisterPair, v: u16) {
        match reg_pair {
            RegisterPair::AF => (self.a, self.f) = split_u16(v),
            RegisterPair::BC => (self.b, self.c) = split_u16(v),
            RegisterPair::DE => (self.d, self.e) = split_u16(v),
            RegisterPair::HL => (self.h, self.l) = split_u16(v),
            RegisterPair::PC => self.pc = v,
            RegisterPair::SP => self.sp = v,
        }
    }

    pub fn set_flag<F: Into<u8>>(&mut self, flag: F, v: bool) {
        set_bit(&mut self.f, flag, v)
    }
}

// impl AddAssign<(Register, u8)> for CPU {
//     fn add_assign(&mut self, rhs: (Register, u8)) {
//         match rhs.0 {
//             Register::A => self.a += rhs.1,
//             Register::F => self.f += rhs.1,
//             Register::B => self.b += rhs.1,
//             Register::C => self.c += rhs.1,
//             Register::D => self.d += rhs.1,
//             Register::E => self.e += rhs.1,
//             Register::H => self.h += rhs.1,
//             Register::L => self.l += rhs.1,
//         }
//     }
// }

// impl SubAssign<(Register, u8)> for CPU {
//     fn sub_assign(&mut self, rhs: (Register, u8)) {
//         match rhs.0 {
//             Register::A => self.a -= rhs.1,
//             Register::F => self.f -= rhs.1,
//             Register::B => self.b -= rhs.1,
//             Register::C => self.c -= rhs.1,
//             Register::D => self.d -= rhs.1,
//             Register::E => self.e -= rhs.1,
//             Register::H => self.h -= rhs.1,
//             Register::L => self.l -= rhs.1,
//         }
//     }
// }
