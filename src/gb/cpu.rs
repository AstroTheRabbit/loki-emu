use super::utils::*;
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
    #[cfg(test)]
    pub fn new(a: u8, f: u8, b: u8, c: u8, d: u8, e: u8, h: u8, l: u8, pc: u16, sp: u16) -> Self {
        Self {
            a,
            f,
            b,
            c,
            d,
            e,
            h,
            l,
            pc,
            sp,
        }
    }

    pub fn new_init() -> Self {
        Self {
            a: 0x00,
            f: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0000,
            sp: 0x0000,
        }
    }

    pub fn get_register(&self, reg: Register) -> u8 {
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
            Register::F => self.f = value & 0xF0,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        }
    }

    pub fn get_register_pair(&self, pair: RegisterPair) -> u16 {
        match pair {
            RegisterPair::AF => join_u16(self.f, self.a),
            RegisterPair::BC => join_u16(self.c, self.b),
            RegisterPair::DE => join_u16(self.e, self.d),
            RegisterPair::HL => join_u16(self.l, self.h),
            RegisterPair::PC => self.pc,
            RegisterPair::SP => self.sp,
        }
    }

    pub fn set_register_pair(&mut self, pair: RegisterPair, value: u16) {
        match pair {
            RegisterPair::AF => (self.f, self.a) = {
                let (lsb, msb) = split_u16(value);
                (lsb & 0xF0, msb)
            },
            RegisterPair::BC => (self.c, self.b) = split_u16(value),
            RegisterPair::DE => (self.e, self.d) = split_u16(value),
            RegisterPair::HL => (self.l, self.h) = split_u16(value),
            RegisterPair::PC => self.pc = value,
            RegisterPair::SP => self.sp = value,
        }
    }

    #[inline]
    pub fn get_flag<F: Into<u8>>(&self, flag: F) -> bool {
        get_bit(self.f, flag)
    }

    #[inline]
    pub fn set_flag<F: Into<u8>>(&mut self, flag: F, value: bool) {
        set_bit(&mut self.f, flag, value)
    }

    #[inline]
    pub fn toggle_flag<F: Into<u8>>(&mut self, flag: F) {
        toggle_bit(&mut self.f, flag);
    }

    // * ADC/ADDition

    /// Adds a value and a register together, handling flags and returning the result.
    ///
    /// Note: Does not set the register to the new value, [`CPU::set_register`] must be called seperately.
    pub fn add_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let (new, overflow) = reg_val.overflowing_add(value);

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, get_bit((reg_val & 0xF) + (value & 0xF), 0x10));
        self.set_flag(Flag::C, overflow);
        new
    }

    /// Adds a value, a register and the carry flags together, handling flags and returning the result.
    ///
    /// Note: Does not set the register to the new value, [`CPU::set_register`] must be called seperately.
    pub fn adc_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let carry = self.get_flag(Flag::C) as u8;
        let (new, overflow_1) = reg_val.overflowing_add(value);
        let (new, overflow_2) = new.overflowing_add(carry);
        let overflow = overflow_1 | overflow_2;

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(
            Flag::H,
            get_bit((reg_val & 0xF) + (value & 0xF) + (carry & 0xF), 0x10),
        );
        self.set_flag(Flag::C, overflow);
        new
    }

    /// Adds a value and a register pair together, handling flags and returning the result.
    ///
    /// Note: Does not set either register pair to the new value, [`CPU::set_register_pair`] must be called seperately.
    pub fn add_register_pair(&mut self, pair: RegisterPair, value: u16) -> u16 {
        let reg_val = self.get_register_pair(pair);
        let (new, overflow) = reg_val.overflowing_add(value);

        self.set_flag(Flag::N, false);
        self.set_flag(
            Flag::H,
            get_bit((reg_val & 0xFFF) + (value & 0xFFF), 0x1000u16),
        );
        self.set_flag(Flag::C, overflow);
        new
    }

    // * SBC/SUBtraction

    /// Subtracts a value from a register, handling flags and returning the result.
    ///
    /// Note: Does not set either register pair to the new value, [`CPU::set_register_pair`] must be called seperately.
    pub fn sub_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let (new, overflow) = reg_val.overflowing_sub(value);

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, get_bit((reg_val & 0xF) - (value & 0xF), 0x10));
        self.set_flag(Flag::C, overflow);
        new
    }

    /// Subtracts a value and the carry flag from a register, handling flags and returning the result.
    ///
    /// Note: Does not set either register pair to the new value, [`CPU::set_register_pair`] must be called seperately.
    pub fn sbc_register(&mut self, reg: Register, value: u8) -> u8 {
        let reg_val = self.get_register(reg);
        let carry = self.get_flag(Flag::C) as u8;
        let (new, overflow_1) = reg_val.overflowing_sub(value);
        let (new, overflow_2) = new.overflowing_sub(carry);
        let overflow = overflow_1 | overflow_2;

        self.set_flag(Flag::Z, new == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(
            Flag::H,
            get_bit((reg_val & 0xF) - (value & 0xF) - (carry & 0xF), 0x10),
        );
        self.set_flag(Flag::C, overflow);
        new
    }

    // * Bitwise ops

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
        let reg_val = self.get_register(reg);
        let value = reg_val.wrapping_add(1);
        self.set_register(reg, value);

        self.set_flag(Flag::Z, value == 0);
        self.set_flag(Flag::N, false);
        self.set_flag(Flag::H, get_bit((reg_val & 0x0F) + 1, 0x10));
    }

    /// Adds one to the register pair, managing flags correctly.
    pub fn inc_register_pair(&mut self, pair: RegisterPair) {
        let reg_val = self.get_register_pair(pair).wrapping_add(1);
        self.set_register_pair(pair, reg_val);
    }

    /// Subtracts one from the register, managing flags correctly.
    pub fn dec_register(&mut self, reg: Register) {
        let reg_val = self.get_register(reg);
        let value = reg_val.wrapping_sub(1);
        self.set_register(reg, value);

        self.set_flag(Flag::Z, value == 0);
        self.set_flag(Flag::N, true);
        self.set_flag(Flag::H, get_bit((reg_val & 0x0F) - 1, 0x10));
    }

    /// Subtracts one to the register pair, managing flags correctly.
    pub fn dec_register_pair(&mut self, pair: RegisterPair) {
        let reg_val = self.get_register_pair(pair).wrapping_sub(1);
        self.set_register_pair(pair, reg_val);
    }
}
