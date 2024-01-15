use super::utils::{get_bit, join_u16, set_bit, toggle_bit, Flag, RegisterPair};
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
            pc: 0x0000,
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

    pub fn set_register(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.a = value,
            Register::F => self.f = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    pub fn get_register_pair(&mut self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::AF => join_u16(self.a, self.f),
            RegisterPair::BC => join_u16(self.b, self.c),
            RegisterPair::DE => join_u16(self.d, self.e),
            RegisterPair::HL => join_u16(self.h, self.l),
            RegisterPair::PC => self.pc,
            RegisterPair::SP => self.sp,
        }
    }

    pub fn set_register_pair(&mut self, pair: RegisterPair, value: u16) {
        match pair {
            RegisterPair::AF => (self.a, self.f) = split_u16(value),
            RegisterPair::BC => (self.b, self.c) = split_u16(value),
            RegisterPair::DE => (self.d, self.e) = split_u16(value),
            RegisterPair::HL => (self.h, self.l) = split_u16(value),
            RegisterPair::PC => self.pc = value,
            RegisterPair::SP => self.sp = value,
        }
    }

    #[inline]
    pub fn get_flag<F: Into<u8>>(&self, flag: F) -> bool {
        get_bit(&self.f, flag)
    }

    #[inline]
    pub fn set_flag<F: Into<u8>>(&mut self, flag: F, value: bool) {
        set_bit(&mut self.f, flag, value)
    }

    #[inline]
    pub fn toggle_flag<F: Into<u8>>(&mut self, flag: F) {
        toggle_bit(&mut self.f, flag);
    }

    // * ADDition

    /// Adds a value and a register together, handling flags and returning the result.
    ///
    /// Note: Does not set the register to the new value, [`CPU::set_register`] must be called seperately.
    pub fn add_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let (new, overflow) = reg_val.overflowing_add(value);

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, (reg_val & 0xF) + (value & 0xF) > 0xF);
        self.set_flag(Flag::C, overflow);
        new
    }

    /// Adds a value and a register pair together, handling flags and returning the result.
    ///
    /// Note: Does not set either register pair to the new value, [`CPU::set_register_pair`] must be called seperately.
    pub fn add_register_pair(&mut self, pair: RegisterPair, value: u16) -> u16 {
        let reg_val = self.get_register_pair(pair);
        let (new, overflow) = reg_val.overflowing_add(value);

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, (reg_val & 0xFFF) + (value & 0xFFF) > 0xFFF);
        self.set_flag(Flag::C, overflow);
        new
    }

    /// Subtracts a value from a register, handling flags and returning the result.
    ///
    /// Note: Does not set either register pair to the new value, [`CPU::set_register_pair`] must be called seperately.
    pub fn sub_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let (new, overflow) = reg_val.overflowing_sub(value);

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, (reg_val & 0xF) + (value & 0xF) > 0xF);
        self.set_flag(Flag::C, overflow);
        new
    }

    /// Bitwise AND a value and a register together, handling flags and returning the result.
    ///
    /// Note: Does not set either register to the new value, [`CPU::set_register`] must be called seperately.
    pub fn and_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let new = reg_val & value;

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N | Flag::C, false);
        self.set_flag(Flag::H, true);
        new
    }

    /// Bitwise XOR a value and register together, handling flags and returning the result.
    ///
    /// Note: Does not set either register to the new value, [`CPU::set_register`] must be called seperately.
    pub fn xor_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let new = reg_val ^ value;

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N | Flag::H | Flag::C, false);
        new
    }

    /// Bitwise OR a value and register together, handling flags and returning the result.
    ///
    /// Note: Does not set either register to the new value, [`CPU::set_register`] must be called seperately.
    pub fn or_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let new = reg_val | value;

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N | Flag::H | Flag::C, false);
        new
    }

    // * INC/DECrement

    /// Adds one to the register, managing flags correctly.
    pub fn inc_register(&mut self, reg: Register) {
        let v = self.get_register(reg).wrapping_add(1);
        self.set_register(reg, v);

        self.set_flag(Flag::Z, v == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, (v & 0xF) > 0xE); // same as (v & 0xF) + (1 & 0xF) > 0xF
    }

    /// Adds one to the register pair, managing flags correctly.
    pub fn inc_register_pair(&mut self, pair: RegisterPair) {
        let v = self.get_register_pair(pair).wrapping_add(1);
        self.set_register_pair(pair, v);
    }

    /// Subtracts one from the register, managing flags correctly.
    pub fn dec_register(&mut self, reg: Register) {
        let v = self.get_register(reg).wrapping_sub(1);
        self.set_register(reg, v);

        self.set_flag(Flag::Z, v == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, (v & 0xF) > 0x10); // same as (v & 0xF) - (1 & 0xF) > 0xF
    }

    /// Subtracts one to the register pair, managing flags correctly.
    pub fn dec_register_pair(&mut self, pair: RegisterPair) {
        let v = self.get_register_pair(pair).wrapping_sub(1);
        self.set_register_pair(pair, v);
    }
}
