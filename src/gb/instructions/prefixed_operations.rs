#![allow(non_snake_case)]

// * RLC & RRC

// ? Opcode | Carry | Bit 0
// ? RLC    | Bit 7 | Bit 7
// ? RL     | Bit 7 | Carry
// ? SL/SLA | Bit 7 | 0

// ? Opcode | Carry | Bit 7
// ? RRC    | Bit 0 | Bit 0
// ? RR     | Bit 0 | Carry
// ? SR/SRL | Bit 0 | 0
// ? CB SRA | Bit 0 | Bit 7

use crate::gb::{emu::GameboyEmulator, utils::*};

use super::instructions::InstructionStep;

/// Rotate register `r8` left, setting the carry flag to the previous bit 7.
pub fn RLC_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let new_carry = get_bit(v, 0b1000_0000);
    let v = (v << 1) | new_carry as u8;

    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::C, new_carry);
    emu.cpu.set_flag(Flag::N | Flag::H, false);
    InstructionStep::Complete
}

/// Rotate register `r8` right, setting the carry flag to the previous bit 0.
pub fn RRC_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let new_carry = get_bit(v, 0b0000_0001);
    let v = (v >> 1) | ((new_carry as u8) << 7);

    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::C, new_carry);
    emu.cpu.set_flag(Flag::N | Flag::H, false);
    InstructionStep::Complete
}

/// Rotate the value at `r16` left, setting the carry flag to the previous bit 7.
pub fn RLC_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let new_carry = get_bit(v, 0b1000_0000);
        let v = (v << 1) | new_carry as u8;

        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::C, new_carry);
            emu.cpu.set_flag(Flag::N | Flag::H, false);
            InstructionStep::Complete
        })
    })
}

/// Rotate the value at `r16` right, setting the carry flag to the previous bit 0.
pub fn RRC_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let new_carry = get_bit(v, 0b0000_0001);
        let v = (v >> 1) | ((new_carry as u8) << 7);

        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::C, new_carry);
            emu.cpu.set_flag(Flag::N | Flag::H, false);
            InstructionStep::Complete
        })
    })
}

// * RL & RR

/// Rotate register `r8` and the carry flag left.
pub fn RL_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let prev_carry = emu.cpu.get_flag(Flag::C);
    let new_carry = get_bit(v, 0b1000_0000);
    let v = (v << 1) | prev_carry as u8;

    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::C, new_carry);
    emu.cpu.set_flag(Flag::N | Flag::H, false);
    InstructionStep::Complete
}

/// Rotate register `r8` and the carry flag right.
pub fn RR_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let prev_carry = emu.cpu.get_flag(Flag::C);
    let new_carry = get_bit(v, 0b0000_0001);
    let v = (v >> 1) | ((prev_carry as u8) << 7);

    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::C, new_carry);
    emu.cpu.set_flag(Flag::N | Flag::H, false);
    InstructionStep::Complete
}

/// Rotate the value at address `r16` and the carry flag left.
pub fn RL_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let prev_carry = emu.cpu.get_flag(Flag::C);
        let new_carry = get_bit(v, 0b1000_0000);
        let v = (v << 1) | prev_carry as u8;

        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::C, new_carry);
            emu.cpu.set_flag(Flag::N | Flag::H, false);
            InstructionStep::Complete
        })
    })
}

/// Rotate the value at address `r16` and the carry flag right.
pub fn RR_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let prev_carry = emu.cpu.get_flag(Flag::C);
        let new_carry = get_bit(v, 0b0000_0001);
        let v = (v >> 1) | ((prev_carry as u8) << 7);

        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::C, new_carry);
            emu.cpu.set_flag(Flag::N | Flag::H, false);
            InstructionStep::Complete
        })
    })
}

// * SLA, SRA & SRL

/// Shift register `r8` left arithmetically.
pub fn SLA_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let new_carry = get_bit(v, 0b1000_0000);
    let v = v << 1;

    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::C, new_carry);
    emu.cpu.set_flag(Flag::N | Flag::H, false);
    InstructionStep::Complete
}

/// Shift register `r8` right arithmetically.
pub fn SRA_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let new_carry = get_bit(v, 0b0000_0001);
    let v = (v >> 1) | (v & 0b1000_0000);

    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::C, new_carry);
    emu.cpu.set_flag(Flag::N | Flag::H, false);
    InstructionStep::Complete
}

/// Shift register `r8` right logically.
pub fn SRL_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let new_carry = get_bit(v, 0b0000_0001);
    let v = v >> 1;

    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::C, new_carry);
    emu.cpu.set_flag(Flag::N | Flag::H, false);
    InstructionStep::Complete
}

/// Shift the value at address `r16` left arithmetically.
pub fn SLA_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let new_carry = get_bit(v, 0b1000_0000);
        let v = v << 1;

        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::C, new_carry);
            emu.cpu.set_flag(Flag::N | Flag::H, false);
            InstructionStep::Complete
        })
    })
}

/// Shift the value at address `r16` right arithmetically.
pub fn SRA_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let new_carry = get_bit(v, 0b0000_0001);
        let v = (v >> 1) | (v & 0b1000_0000);

        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::C, new_carry);
            emu.cpu.set_flag(Flag::N | Flag::H, false);
            InstructionStep::Complete
        })
    })
}

/// Shift the value at address `r16` right logically.
pub fn SRL_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let new_carry = get_bit(v, 0b0000_0001);
        let v = v >> 1;

        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::C, new_carry);
            emu.cpu.set_flag(Flag::N | Flag::H, false);
            InstructionStep::Complete
        })
    })
}

// * SWAP & BIT

/// Swap the upper and lower 4 bits of register `r8`.
pub fn SWAP_r8(emu: &mut GameboyEmulator, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    let v = (v << 4) | (v >> 4);
    emu.cpu.set_register(r8, v);
    emu.cpu.set_flag(Flag::Z, v == 0);
    emu.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
    InstructionStep::Complete
}

/// Swap the upper and lower 4 bits of the value at address `r16`.
pub fn SWAP_r16(_emu: &mut GameboyEmulator, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        let v = (v << 4) | (v >> 4);
        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            emu.cpu.set_flag(Flag::Z, v == 0);
            emu.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
            InstructionStep::Complete
        })
    })
}

/// Set the zero flag if bit `b` of register `r8` is not set.
pub fn BIT_b_r8(emu: &mut GameboyEmulator, b: u8, r8: Register) -> InstructionStep {
    let v = emu.cpu.get_register(r8);
    emu.cpu.set_flag(Flag::Z, !get_bit(v, 1 << b));
    emu.cpu.set_flag(Flag::N, false);
    emu.cpu.set_flag(Flag::H, true);
    InstructionStep::Complete
}

/// Set the zero flag if bit `b` of the value at address `r16` is not set.
pub fn BIT_b_r16(_emu: &mut GameboyEmulator, b: u8, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let v = emu.read_r16(r16);
        emu.cpu.set_flag(Flag::Z, !get_bit(v, 1 << b));
        emu.cpu.set_flag(Flag::N, false);
        emu.cpu.set_flag(Flag::H, true);
        InstructionStep::Complete
    })
}

// * RES & SET

/// Set bit `b` of register `r8` to 0.
pub fn RES_b_r8(emu: &mut GameboyEmulator, b: u8, r8: Register) -> InstructionStep {
    let mut v = emu.cpu.get_register(r8);
    set_bit(&mut v, 1 << b, false);
    emu.cpu.set_register(r8, v);
    InstructionStep::Complete
}

/// Set bit `b` of the value at address `r16` to 0.
pub fn RES_b_r16(_emu: &mut GameboyEmulator, b: u8, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let mut v = emu.read_r16(r16);
        set_bit(&mut v, 1 << b, false);
        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            InstructionStep::Complete
        })
    })
}

/// Set bit `b` of register `r8` to 1.
pub fn SET_b_r8(emu: &mut GameboyEmulator, b: u8, r8: Register) -> InstructionStep {
    let mut v = emu.cpu.get_register(r8);
    set_bit(&mut v, 1 << b, true);
    emu.cpu.set_register(r8, v);
    InstructionStep::Complete
}

/// Set bit `b` of the value at address `r16` to 1.
pub fn SET_b_r16(_emu: &mut GameboyEmulator, b: u8, r16: RegisterPair) -> InstructionStep {
    // ? One bus read or write per m-cycle.
    InstructionStep::new(move |emu| {
        let mut v = emu.read_r16(r16);
        set_bit(&mut v, 1 << b, true);
        InstructionStep::new(move |emu| {
            emu.write_r16(r16, v);
            InstructionStep::Complete
        })
    })
}
