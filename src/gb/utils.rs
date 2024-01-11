use std::ops::BitOr;

use num_enum::IntoPrimitive;

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

#[derive(Debug, Clone, Copy, IntoPrimitive)]
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

// ? Instruction macros

// * LD

/// Load register `r8_2` into register `r8_1`.
#[macro_export(local_inner_macros)]
macro_rules! LD_r8_r8 {
    ($emu:ident, $r8_1:expr, $r8_2:expr) => {
        {
            let v = $emu.cpu.get_register($r8_2);
            $emu.cpu.set_register($r8_1, v);
        }
    };
}

/// Load immediate value `n8` into register `r8`.
#[macro_export(local_inner_macros)]
macro_rules! LD_r8_n8 {
    ($emu:ident, $r8:expr) => {
        {
            let v = $emu.read_pc_u8();
            $emu.cpu.set_register($r8, v);
        }
    };
}

/// Load immediate value `n16` into register pair `r16`.
#[macro_export(local_inner_macros)]
macro_rules! LD_r16_n16 {
    ($emu:ident, $r16:expr) => {
        {
            let v = $emu.read_pc_u16();
            $emu.cpu.set_register_pair($r16, v);
        }
    };
}

/// Load the value located at address `r16` into register `r8`.
#[macro_export(local_inner_macros)]
macro_rules! LD_r8_r16 {
    ($emu:ident, $r8:expr, $r16:expr) => {
        {
            let address = $emu.cpu.get_register_pair($r16);
            let v = $emu.bus.read(address);
            $emu.cpu.set_register($r8, v);
        }
    };
}

/// Load register `r8` into the location of address `r16`.
#[macro_export(local_inner_macros)]
macro_rules! LD_r16_r8 {
    ($emu:ident, $r16:expr, $r8:expr) => {
        {
            let address = $emu.cpu.get_register_pair($r16);
            let value = $emu.cpu.get_register($r8);
            $emu.bus.write(address, value);
        }
    };
}

/// Load the register pair `r16` into the location of immediate address `a16`.
#[macro_export(local_inner_macros)]
macro_rules! LD_a16_r16 {
    ($emu:ident, $r16:expr) => {
        {
            let address = $emu.read_pc_u16();
            let (lsb, msb) = split_u16($emu.cpu.get_register_pair($r16));
            $emu.bus.write(address, lsb);
            $emu.bus.write(address + 1, msb);
        }
    };
}

// * INC/DEC

/// Increment register pair `r16`.
#[macro_export(local_inner_macros)]
macro_rules! INC_r16 {
    ($emu:ident, $r16:expr) => {
        {
            $emu.cpu.increment_register_pair($r16);
        }
    };
}

/// Decrement register pair `r16`.
#[macro_export(local_inner_macros)]
macro_rules! DEC_r16 {
    ($emu:ident, $r16:expr) => {
        {
            $emu.cpu.decrement_register_pair($r16);
        }
    };
}

/// Increment register `r8`.
#[macro_export(local_inner_macros)]
macro_rules! INC_r8 {
    ($emu:ident, $r8:expr) => {
        {
            $emu.cpu.increment_register($r8);
        }
    };
}

/// Decrement register `r8`.
#[macro_export(local_inner_macros)]
macro_rules! DEC_r8 {
    ($emu:ident, $r8:expr) => {
        {
            $emu.cpu.increment_register($r8);
        }
    };
}

// * ADD

/// Add registers `r8_1` and `r8_2`, storing the result in `r8_1`.
#[macro_export(local_inner_macros)]
macro_rules! ADD_r8_r8 {
    ($emu:ident, $r8_1:expr, $r8_2:expr) => {
        {
            let v = $emu.cpu.add_registers($r8_1, $r8_2);
            $emu.cpu.set_register($r8_1, v);
        }
    };
}

/// Add register pairs `r16_1` and `r16_2`, storing the result in `r16_1`.
#[macro_export(local_inner_macros)]
macro_rules! ADD_r16_r16 {
    ($emu:ident, $r16_1:expr, $r16_2:expr) => {
        {
            let v = $emu.cpu.add_register_pairs($r16_1, $r16_2);
            $emu.cpu.set_register_pair($r16_1, v);
        }
    };
}

// Add the value located at address `r16` to register `r8`.
#[macro_export(local_inner_macros)]
macro_rules! ADD_r8_r16 {
    ($emu:ident, $r8:expr, $r16:expr) => {
        {
            let address = $emu.cpu.get_register_pair($r16);
            let val = $emu.bus.read(address);
            let res = $emu.cpu.add_value_register($r8, val);
            $emu.cpu.set_register($r8, res);
        }
    };
}

// * ADC

/// Add the carry flag, and registers `r8_1` and `r8_2`, storing the result in `r8_1`.
#[macro_export(local_inner_macros)]
macro_rules! ADC_r8_r8 {
    ($emu:ident, $r8_1:expr, $r8_2:expr) => {
        {
            let val = $emu.cpu.get_register($r8_2) + $emu.cpu.get_flag(Flag::C) as u8;
            let res = $emu.cpu.add_value_register($r8_1, val);
            $emu.cpu.set_register($r8_1, res);
        }
    };
}

/// Add the carry flag, register `r8` and the value located at addresss `r16`, storing the result in `r8`.
#[macro_export(local_inner_macros)]
macro_rules! ADC_r8_r16 {
    ($emu:ident, $r8:expr, $r16:expr) => {
        {
            let address = $emu.cpu.get_register_pair($r16);
            let val = $emu.bus.read(address) + $emu.cpu.get_flag(Flag::C) as u8;
            let res = $emu.cpu.add_value_register($r8, val);
            $emu.cpu.set_register($r8, res);
        }
    };
}

// * SUB

/// Subtract register `r8_2` from register `r8_1`, storing the result in `r8_1`.
#[macro_export(local_inner_macros)]
macro_rules! SUB_r8_r8 {
    ($emu:ident, $r8_1:expr, $r8_2:expr) => {
        {
            let v = $emu.cpu.sub_registers($r8_1, $r8_2);
            $emu.cpu.set_register($r8_1, v);
        }
    };
}

/// Subtract the value located at `r16` from register `r8`, storing the result in `r8`.
#[macro_export(local_inner_macros)]
macro_rules! SUB_r8_r16 {
    ($emu:ident, $r8:expr, $r16:expr) => {
        {
            let address = $emu.cpu.get_register_pair($r16);
            let val = $emu.bus.read(address);
            let res = $emu.cpu.sub_value_register($r8, val);
            $emu.cpu.set_register($r8, res);
        }
    };
}

// * SBC

/// Subtract register `r8_2` and the carry flag from register `r8_1`, storing the result in `r8_1`.
#[macro_export(local_inner_macros)]
macro_rules! SBC_r8_r8 {
    ($emu:ident, $r8_1:expr, $r8_2:expr) => {
        {
            let val = $emu.cpu.get_register($r8_2) + $emu.cpu.get_flag(Flag::C) as u8;
            let res = $emu.cpu.sub_value_register($r8_1, val);
            $emu.cpu.set_register($r8_1, res);
        }
    };
}

/// Subtract the value located at `r16` and the carry flag from register `r8_1`, storing the result in `r8`.
#[macro_export(local_inner_macros)]
macro_rules! SBC_r8_r16 {
    ($emu:ident, $r8:expr, $r16:expr) => {
        {
            let address = $emu.cpu.get_register_pair($r16);
            let val = $emu.bus.read(address) + $emu.cpu.get_flag(Flag::C) as u8;
            let res = $emu.cpu.sub_value_register($r8, val);
            $emu.cpu.set_register($r8, res);
        }
    };
}

// * AND

/// Bitwise AND registers `r8_1` and `r8_2`, storing the result in `r8_1`.
#[macro_export(local_inner_macros)]
macro_rules! AND_r8_r8 {
    ($emu:ident, $r8_1:expr, $r8_2:expr) => {
        {
            let v = $emu.cpu.sub_registers($r8_1, $r8_2);
            $emu.cpu.set_register($r8_1, v);
        }
    };
}

// * JR

/// Add the signed immediate value `e8` to the `PC` and jump to it.
#[macro_export(local_inner_macros)]
macro_rules! JR_e8 {
    ($emu:ident) => {
        {
            let v = $emu.read_pc_u8() as i8;
            let r = $emu.cpu.get_register_pair(RegisterPair::PC);
            $emu.cpu.set_register_pair(RegisterPair::PC, r.wrapping_add_signed(v.into()));
        }
    };
}