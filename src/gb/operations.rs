#![allow(non_snake_case)]

use super::utils::*;
use crate::gb::emu::GameBoyEmulator;

impl GameBoyEmulator {
    // * LD

    /// Load register `r8_2` into register `r8_1`.
    pub fn LD_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let v = self.cpu.get_register(r8_2);
        self.cpu.set_register(r8_1, v);
    }

    /// Load immediate value `n8` into register `r8`.
    pub fn LD_r8_n8(&mut self, r8: Register) {
        let v = self.read_u8(RegisterPair::PC);
        self.cpu.set_register(r8, v);
    }

    /// Load immediate value `n16` into register pair `r16`.
    pub fn LD_r16_n16(&mut self, r16: RegisterPair) {
        let v = self.read_u16(RegisterPair::PC);
        self.cpu.set_register_pair(r16, v);
    }

    /// Load the value at address `r16` into register `r8`.
    pub fn LD_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        self.cpu.set_register(r8, v);
    }

    /// Load register `r8` into the location of address `r16`.
    pub fn LD_r16_r8(&mut self, r16: RegisterPair, r8: Register) {
        let address = self.cpu.get_register_pair(r16);
        let value = self.cpu.get_register(r8);
        self.bus.write(address, value);
    }

    /// Load the register pair `r16` into the location of immediate address `a16`.
    pub fn LD_a16_r16(&mut self, r16: RegisterPair) {
        let address = self.read_u16(RegisterPair::PC);
        let (lsb, msb) = split_u16(self.cpu.get_register_pair(r16));
        self.bus.write(address, lsb);
        self.bus.write(address + 1, msb);
    }

    // * INC/DEC

    /// Increment register pair `r16`.
    pub fn INC_r16(&mut self, r16: RegisterPair) {
        self.cpu.increment_register_pair(r16);
    }

    /// Decrement register pair `r16`.
    pub fn DEC_r16(&mut self, r16: RegisterPair) {
        self.cpu.decrement_register_pair(r16);
    }

    /// Increment register `r8`.
    pub fn INC_r8(&mut self, r8: Register) {
        self.cpu.increment_register(r8);
    }

    /// Decrement register `r8`.
    pub fn DEC_r8(&mut self, r8: Register) {
        self.cpu.increment_register(r8);
    }

    // * ADD

    /// Add registers `r8_1` and `r8_2`, storing the result in `r8_1`.
    pub fn ADD_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2);
        let v = self.cpu.add_register(r8_1, val);
        self.cpu.set_register(r8_1, v);
    }

    /// Add register `r8` and immediate value `n8`, storing the result in `r8`.
    pub fn ADD_r8_n8(&mut self, r8: Register) {
        let val = self.read_u8(RegisterPair::PC);
        let v = self.cpu.add_register(r8, val);
        self.cpu.set_register(r8, v);
    }

    // Add the value at address `r16` to register `r8`.
    pub fn ADD_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address);
        let res = self.cpu.add_register(r8, val);
        self.cpu.set_register(r8, res);
    }

    /// Add register pairs `r16_1` and `r16_2`, storing the result in `r16_1`.
    pub fn ADD_r16_r16(&mut self, r16_1: RegisterPair, r16_2: RegisterPair) {
        let val = self.cpu.get_register_pair(r16_2);
        let v = self.cpu.add_register_pair(r16_1, val);
        self.cpu.set_register_pair(r16_1, v);
    }

    // * ADC

    /// Add the carry flag, and registers `r8_1` and `r8_2`, storing the result in `r8_1`.
    pub fn ADC_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2) + self.cpu.get_flag(Flag::C) as u8;
        let res = self.cpu.add_register(r8_1, val);
        self.cpu.set_register(r8_1, res);
    }

    /// Add the carry flag, register `r8` and immediate value `n8`, storing the result in `r8`.
    pub fn ADC_r8_n8(&mut self, r8: Register) {
        let val = self.read_u8(RegisterPair::PC) + self.cpu.get_flag(Flag::C) as u8;
        let v = self.cpu.add_register(r8, val);
        self.cpu.set_register(r8, v);
    }

    /// Add the carry flag, register `r8` and the value at addresss `r16`, storing the result in `r8`.
    pub fn ADC_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address) + self.cpu.get_flag(Flag::C) as u8;
        let res = self.cpu.add_register(r8, val);
        self.cpu.set_register(r8, res);
    }

    // * SUB

    /// Subtract register `r8_2` from register `r8_1`, storing the result in `r8_1`.
    pub fn SUB_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2);
        let v = self.cpu.sub_register(r8_1, val);
        self.cpu.set_register(r8_1, v);
    }

    /// Subtract immediate value `n8` from register `r8`, storing the result in `r8`.
    pub fn SUB_r8_n8(&mut self, r8: Register) {
        let val = self.read_u8(RegisterPair::PC);
        let v = self.cpu.sub_register(r8, val);
        self.cpu.set_register(r8, v);
    }

    /// Subtract the value at `r16` from register `r8`, storing the result in `r8`.
    pub fn SUB_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address);
        let res = self.cpu.sub_register(r8, val);
        self.cpu.set_register(r8, res);
    }

    // * SBC

    /// Subtract the carry flag and register `r8_2` from register `r8_1`, storing the result in `r8_1`.
    pub fn SBC_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2) + self.cpu.get_flag(Flag::C) as u8;
        let res = self.cpu.sub_register(r8_1, val);
        self.cpu.set_register(r8_1, res);
    }

    /// Subtract the carry flag and the immediate value `n8` from register `r8`, storing the result in `r8`.
    pub fn SBC_r8_n8(&mut self, r8: Register) {
        let val = self.read_u8(RegisterPair::PC) + self.cpu.get_flag(Flag::C) as u8;
        let res = self.cpu.sub_register(r8, val);
        self.cpu.set_register(r8, res);
    }

    /// Subtract the carry flag and the value at `r16` from register `r8`, storing the result in `r8`.
    pub fn SBC_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address) + self.cpu.get_flag(Flag::C) as u8;
        let res = self.cpu.sub_register(r8, val);
        self.cpu.set_register(r8, res);
    }

    // * AND

    /// Bitwise AND registers `r8_1` and `r8_2`, storing the result in `r8_1`.
    pub fn AND_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2);
        let v = self.cpu.and_register(r8_1, val);
        self.cpu.set_register(r8_1, v);

        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::H, true);
        self.cpu.set_flag(Flag::N | Flag::C, false);
    }

    /// Bitwise AND register `r8` and the value at address`r16`, storing the result in `r8`.
    pub fn AND_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address);
        let v = self.cpu.and_register(r8, val);
        self.cpu.set_register(r8, v);

        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::H, true);
        self.cpu.set_flag(Flag::N | Flag::C, false);
    }

    // * XOR

    /// Bitwise XOR registers `r8_1` and `r8_2`, storing the result in `r8_1`.
    pub fn XOR_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2);
        let v = self.cpu.xor_register(r8_1, val);
        self.cpu.set_register(r8_1, v);

        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
    }

    /// Bitwise XOR register `r8` and the value at address`r16`, storing the result in `r8`.
    pub fn XOR_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address);
        let v = self.cpu.xor_register(r8, val);
        self.cpu.set_register(r8, v);

        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
    }

    // * OR

    /// Bitwise OR registers `r8_1` and `r8_2`, storing the result in `r8_1`.
    pub fn OR_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2);
        let v = self.cpu.or_register(r8_1, val);
        self.cpu.set_register(r8_1, v);

        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
    }

    /// Bitwise OR register `r8` and the value at address`r16`, storing the result in `r8`.
    pub fn OR_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address);
        let v = self.cpu.or_register(r8, val);
        self.cpu.set_register(r8, v);

        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
    }

    // * CP

    /// Subtract register `r8_2` from register `r8_1`, but do not store the result.
    pub fn CP_r8_r8(&mut self, r8_1: Register, r8_2: Register) {
        let val = self.cpu.get_register(r8_2);
        let _ = self.cpu.sub_register(r8_1, val);
    }

    /// Subtract the value at `r16` from register `r8`, but do not store the result.
    pub fn CP_r8_r16(&mut self, r8: Register, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let val = self.bus.read(address);
        let _ = self.cpu.sub_register(r8, val);
    }

    // * JR & JP

    /// Add the signed immediate value `e8` to the `PC` and jump to it.
    pub fn JR_e8(&mut self) {
        let v = self.read_u8(RegisterPair::PC) as i8;
        let r = self.cpu.get_register_pair(RegisterPair::PC);
        self.cpu
            .set_register_pair(RegisterPair::PC, r.wrapping_add_signed(v.into()));
    }

    /// Jump to the immediate address `a16`.
    pub fn JP_a16(&mut self) {
        let v = self.read_u16(RegisterPair::PC);
        self.cpu.set_register_pair(RegisterPair::PC, v);
    }

    // * RET, CALL & RST

    /// Return from subroutine.
    pub fn RET(&mut self) {
        let v = self.read_u16(RegisterPair::SP);
        self.cpu.set_register_pair(RegisterPair::PC, v);
    }

    /// Call function at the immediate address `a16`.
    pub fn CALL_a16(&mut self) {
        let v = self.read_u16(RegisterPair::PC);
        let prev_pc = self.cpu.get_register_pair(RegisterPair::PC);
        self.write_stack(prev_pc);
        self.cpu.set_register_pair(RegisterPair::PC, v);
    }

    /// Call fixed address `a16`.
    pub fn RST_a16(&mut self, a16: u16) {
        let prev_pc = self.cpu.get_register_pair(RegisterPair::PC);
        self.write_stack(prev_pc);
        self.cpu.set_register_pair(RegisterPair::PC, a16);
    }

    // * PUSH & POP

    /// Push register pair `r16` into the stack.
    pub fn PUSH_r16(&mut self, r16: RegisterPair) {
        let val = self.cpu.get_register_pair(r16);
        self.write_stack(val);
    }

    /// Pop from the stack to register pair `r16`.
    pub fn POP_r16(&mut self, r16: RegisterPair) {
        let v = self.read_u16(RegisterPair::SP);
        self.cpu.set_register_pair(r16, v);
    }

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

    /// Rotate register `r8` left, setting the carry flag to the previous bit 7.
    pub fn RLC_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let new_carry = get_bit(&v, 0b1000_0000);
        let v = (v << 1) | new_carry as u8;

        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }
    
    /// Rotate register `r8` right, setting the carry flag to the previous bit 0.
    pub fn RRC_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = (v >> 1) | ((new_carry as u8) << 7);

        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }
    
    /// Rotate the value at `r16` left, setting the carry flag to the previous bit 7.
    pub fn RLC_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let new_carry = get_bit(&v, 0b1000_0000);
        let v = (v << 1) | new_carry as u8;

        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Rotate the value at `r16` right, setting the carry flag to the previous bit 0.
    pub fn RRC_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = (v >> 1) | ((new_carry as u8) << 7);

        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
    }

    // * RL & RR

    /// Rotate register `r8` and the carry flag left.
    pub fn RL_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let prev_carry = self.cpu.get_flag(Flag::C);
        let new_carry = get_bit(&v, 0b1000_0000);
        let v = (v << 1) | prev_carry as u8;

        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Rotate register `r8` and the carry flag right.
    pub fn RR_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let prev_carry = self.cpu.get_flag(Flag::C);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = (v >> 1) | ((prev_carry as u8) << 7);

        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Rotate the value at address `r16` and the carry flag left.
    pub fn RL_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let prev_carry = self.cpu.get_flag(Flag::C);
        let new_carry = get_bit(&v, 0b1000_0000);
        let v = (v << 1) | prev_carry as u8;

        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Rotate the value at address `r16` and the carry flag right.
    pub fn RR_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let prev_carry = self.cpu.get_flag(Flag::C);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = (v >> 1) | ((prev_carry as u8) << 7);

        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    // * SLA, SRA & SRL

    /// Shift register `r8` left arithmetically.
    pub fn SLA_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let new_carry = get_bit(&v, 0b1000_0000);
        let v = v << 1;

        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Shift register `r8` right arithmetically.
    pub fn SRA_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = (v >> 1) | (v & 0b1000_0000);

        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Shift register `r8` right logically.
    pub fn SRL_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = v >> 1;

        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Shift the value at address `r16` left arithmetically.
    pub fn SLA_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let new_carry = get_bit(&v, 0b1000_0000);
        let v = v << 1;

        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Shift the value at address `r16` right arithmetically.
    pub fn SRA_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = (v >> 1) | (v & 0b1000_0000);

        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    /// Shift the value at address `r16` right logically.
    pub fn SRL_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let new_carry = get_bit(&v, 0b0000_0001);
        let v = v >> 1;

        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::C, new_carry);
        self.cpu.set_flag(Flag::N | Flag::H, false);
    }

    // * SWAP & BIT

    /// Swap the upper and lower 4 bits of register `r8`.
    pub fn SWAP_r8(&mut self, r8: Register) {
        let v = self.cpu.get_register(r8);
        let v = (v << 4) | (v >> 4);
        self.cpu.set_register(r8, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
    }

    /// Swap the upper and lower 4 bits of the value at address `r16`.
    pub fn SWAP_r16(&mut self, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        let v = (v << 4) | (v >> 4);
        self.bus.write(address, v);
        self.cpu.set_flag(Flag::Z, v == 0);
        self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
    }

    /// Set the zero flag if bit `b` of register `r8` is not set.
    pub fn BIT_b_r8(&mut self, b: u8, r8: Register) {
        let v = self.cpu.get_register(r8);
        self.cpu.set_flag(Flag::Z, get_bit(&v, 1 << b));
        self.cpu.set_flag(Flag::N, false);
        self.cpu.set_flag(Flag::H, true);
    }

    /// Set the zero flag if bit `b` of the value at address `r16` is not set.
    pub fn BIT_b_r16(&mut self, b: u8, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let v = self.bus.read(address);
        self.cpu.set_flag(Flag::Z, get_bit(&v, 1 << b));
        self.cpu.set_flag(Flag::N, false);
        self.cpu.set_flag(Flag::H, true);
    }

    // * RES & SET

    /// Set bit `b` of register `r8` to 0.
    pub fn RES_b_r8(&mut self, b: u8, r8: Register) {
        let mut v = self.cpu.get_register(r8);
        set_bit(&mut v, 1 << b, false);
        self.cpu.set_register(r8, v);
    }

    /// Set bit `b` of the value at address `r16` to 0.
    pub fn RES_b_r16(&mut self, b: u8, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let mut v = self.bus.read(address);
        set_bit(&mut v, 1 << b, false);
        self.bus.write(address, v);
    }

    /// Set bit `b` of register `r8` to 1.
    pub fn SET_b_r8(&mut self, b: u8, r8: Register) {
        let mut v = self.cpu.get_register(r8);
        set_bit(&mut v, 1 << b, true);
        self.cpu.set_register(r8, v);
    }

    /// Set bit `b` of the value at address `r16` to 1.
    pub fn SET_b_r16(&mut self, b: u8, r16: RegisterPair) {
        let address = self.cpu.get_register_pair(r16);
        let mut v = self.bus.read(address);
        set_bit(&mut v, 1 << b, true);
        self.bus.write(address, v);
    }
}
