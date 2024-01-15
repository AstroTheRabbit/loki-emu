#![allow(non_snake_case)]

use super::instructions::*;

/// Run immediate prefixed operation `n8`.
pub fn PREFIX_n8() -> Instruction {
    Instruction::new("PREFIX".to_string(), |emu| {
        let _op = emu.read_pc();
        InstructionStep::new(move |_emu| {
            // todo!("GB - Prefixed instructions");
            // match op {
            //     // * 0x0_
            //     // Rotate register `B` left, setting the carry flag to the previous bit 7.
            //     0x00 => self.RLC_r8(Register::B),
            //     // Rotate register `C` left, setting the carry flag to the previous bit 7.
            //     0x01 => self.RLC_r8(Register::C),
            //     // Rotate register `D` left, setting the carry flag to the previous bit 7.
            //     0x02 => self.RLC_r8(Register::D),
            //     // Rotate register `E` left, setting the carry flag to the previous bit 7.
            //     0x03 => self.RLC_r8(Register::E),
            //     // Rotate register `H` left, setting the carry flag to the previous bit 7.
            //     0x04 => self.RLC_r8(Register::H),
            //     // Rotate register `L` left, setting the carry flag to the previous bit 7.
            //     0x05 => self.RLC_r8(Register::L),
            //     // Rotate the value at address `HL` left, setting the carry flag to the previous bit 7.
            //     0x06 => self.RLC_r16(RegisterPair::HL),
            //     // Rotate register `A` left, setting the carry flag to the previous bit 7.
            //     0x07 => self.RLC_r8(Register::A),
            //     // Rotate register `B` right, setting the carry flag to the previous bit 0.
            //     0x08 => self.RRC_r8(Register::B),
            //     // Rotate register `C` right, setting the carry flag to the previous bit 0.
            //     0x09 => self.RRC_r8(Register::C),
            //     // Rotate register `D` right, setting the carry flag to the previous bit 0.
            //     0x0A => self.RRC_r8(Register::D),
            //     // Rotate register `E` right, setting the carry flag to the previous bit 0.
            //     0x0B => self.RRC_r8(Register::E),
            //     // Rotate register `H` right, setting the carry flag to the previous bit 0.
            //     0x0C => self.RRC_r8(Register::H),
            //     // Rotate register `L` right, setting the carry flag to the previous bit 0.
            //     0x0D => self.RRC_r8(Register::L),
            //     // Rotate the value at address `HL` right, setting the carry flag to the previous bit 0.
            //     0x0E => self.RRC_r16(RegisterPair::HL),
            //     // Rotate register `A` right, setting the carry flag to the previous bit 0.
            //     0x0F => self.RRC_r8(Register::A),
            //     // * 0x1_
            //     // Rotate register `B` and the carry flag left.
            //     0x10 => self.RL_r8(Register::B),
            //     // Rotate register `C` and the carry flag left.
            //     0x11 => self.RL_r8(Register::C),
            //     // Rotate register `D` and the carry flag left.
            //     0x12 => self.RL_r8(Register::D),
            //     // Rotate register `E` and the carry flag left.
            //     0x13 => self.RL_r8(Register::E),
            //     // Rotate register `H` and the carry flag left.
            //     0x14 => self.RL_r8(Register::H),
            //     // Rotate register `L` and the carry flag left.
            //     0x15 => self.RL_r8(Register::L),
            //     // Rotate the value at address `HL` and the carry flag left.
            //     0x16 => self.RL_r16(RegisterPair::HL),
            //     // Rotate register `A` and the carry flag left.
            //     0x17 => self.RL_r8(Register::A),
            //     // Rotate register `B` and the carry flag right.
            //     0x18 => self.RR_r8(Register::B),
            //     // Rotate register `C` and the carry flag right.
            //     0x19 => self.RR_r8(Register::C),
            //     // Rotate register `D` and the carry flag right.
            //     0x1A => self.RR_r8(Register::D),
            //     // Rotate register `E` and the carry flag right.
            //     0x1B => self.RR_r8(Register::E),
            //     // Rotate register `H` and the carry flag right.
            //     0x1C => self.RR_r8(Register::H),
            //     // Rotate register `L` and the carry flag right.
            //     0x1D => self.RR_r8(Register::L),
            //     // Rotate the value at address `HL` and the carry flag right.
            //     0x1E => self.RR_r16(RegisterPair::HL),
            //     // Rotate register `A` and the carry flag right.
            //     0x1F => self.RR_r8(Register::A),
            //     // * 0x2_
            //     // Shift register `B` left arithmetically.
            //     0x20 => self.SLA_r8(Register::B),
            //     // Shift register `C` left arithmetically.
            //     0x21 => self.SLA_r8(Register::C),
            //     // Shift register `D` left arithmetically.
            //     0x22 => self.SLA_r8(Register::D),
            //     // Shift register `E` left arithmetically.
            //     0x23 => self.SLA_r8(Register::E),
            //     // Shift register `H` left arithmetically.
            //     0x24 => self.SLA_r8(Register::H),
            //     // Shift register `L` left arithmetically.
            //     0x25 => self.SLA_r8(Register::L),
            //     // Shift the value at address `HL` left arithmetically.
            //     0x26 => self.SLA_r16(RegisterPair::HL),
            //     // Shift register `A` left arithmetically.
            //     0x27 => self.SLA_r8(Register::A),
            //     // Shift register `B` right arithmetically.
            //     0x28 => self.SRA_r8(Register::B),
            //     // Shift register `C` right arithmetically.
            //     0x29 => self.SRA_r8(Register::C),
            //     // Shift register `D` right arithmetically.
            //     0x2A => self.SRA_r8(Register::D),
            //     // Shift register `E` right arithmetically.
            //     0x2B => self.SRA_r8(Register::E),
            //     // Shift register `H` right arithmetically.
            //     0x2C => self.SRA_r8(Register::H),
            //     // Shift register `L` right arithmetically.
            //     0x2D => self.SRA_r8(Register::L),
            //     // Shift the value at address `HL` right arithmetically.
            //     0x2E => self.SRA_r16(RegisterPair::HL),
            //     // Shift register `A` right arithmetically.
            //     0x2F => self.SRA_r8(Register::A),
            //     // * 0x3_
            //     // Swap the upper and lower 4 bits of register `B`
            //     0x30 => self.SWAP_r8(Register::B),
            //     // Swap the upper and lower 4 bits of register `C`
            //     0x31 => self.SWAP_r8(Register::C),
            //     // Swap the upper and lower 4 bits of register `D`
            //     0x32 => self.SWAP_r8(Register::D),
            //     // Swap the upper and lower 4 bits of register `E`
            //     0x33 => self.SWAP_r8(Register::E),
            //     // Swap the upper and lower 4 bits of register `H`
            //     0x34 => self.SWAP_r8(Register::H),
            //     // Swap the upper and lower 4 bits of register `L`
            //     0x35 => self.SWAP_r8(Register::L),
            //     // Swap the upper and lower 4 bits of the value at address `HL`
            //     0x36 => self.SWAP_r16(RegisterPair::HL),
            //     // Swap the upper and lower 4 bits of register `A`
            //     0x37 => self.SWAP_r8(Register::A),
            //     // Shift register `B` right logically.
            //     0x38 => self.SRL_r8(Register::B),
            //     // Shift register `C` right logically.
            //     0x39 => self.SRL_r8(Register::C),
            //     // Shift register `D` right logically.
            //     0x3A => self.SRL_r8(Register::D),
            //     // Shift register `E` right logically.
            //     0x3B => self.SRL_r8(Register::E),
            //     // Shift register `H` right logically.
            //     0x3C => self.SRL_r8(Register::H),
            //     // Shift register `L` right logically.
            //     0x3D => self.SRL_r8(Register::L),
            //     // Shift the value at address `HL` right logically.
            //     0x3E => self.SRL_r16(RegisterPair::HL),
            //     // Shift register `A` right logically.
            //     0x3F => self.SRL_r8(Register::A),
            //     // * 0x4_
            //     // Set the zero flag if bit 0 of register `B` is not set.
            //     0x40 => self.BIT_b_r8(0, Register::B),
            //     // Set the zero flag if bit 0 of register `C` is not set.
            //     0x41 => self.BIT_b_r8(0, Register::C),
            //     // Set the zero flag if bit 0 of register `D` is not set.
            //     0x42 => self.BIT_b_r8(0, Register::D),
            //     // Set the zero flag if bit 0 of register `E` is not set.
            //     0x43 => self.BIT_b_r8(0, Register::E),
            //     // Set the zero flag if bit 0 of register `H` is not set.
            //     0x44 => self.BIT_b_r8(0, Register::H),
            //     // Set the zero flag if bit 0 of register `L` is not set.
            //     0x45 => self.BIT_b_r8(0, Register::L),
            //     // Set the zero flag if bit 0 of the value at address `HL` is not set.
            //     0x46 => self.BIT_b_r16(0, RegisterPair::HL),
            //     // Set the zero flag if bit 0 of register `A` is not set.
            //     0x47 => self.BIT_b_r8(0, Register::A),
            //     // Set the zero flag if bit 1 of register `B` is not set.
            //     0x48 => self.BIT_b_r8(1, Register::B),
            //     // Set the zero flag if bit 1 of register `C` is not set.
            //     0x49 => self.BIT_b_r8(1, Register::C),
            //     // Set the zero flag if bit 1 of register `D` is not set.
            //     0x4A => self.BIT_b_r8(1, Register::D),
            //     // Set the zero flag if bit 1 of register `E` is not set.
            //     0x4B => self.BIT_b_r8(1, Register::E),
            //     // Set the zero flag if bit 1 of register `H` is not set.
            //     0x4C => self.BIT_b_r8(1, Register::H),
            //     // Set the zero flag if bit 1 of register `L` is not set.
            //     0x4D => self.BIT_b_r8(1, Register::L),
            //     // Set the zero flag if bit 1 of the value at address `HL` is not set.
            //     0x4E => self.BIT_b_r16(1, RegisterPair::HL),
            //     // Set the zero flag if bit 1 of register `A` is not set.
            //     0x4F => self.BIT_b_r8(1, Register::A),
            //     // * 0x5_
            //     // Set the zero flag if bit 2 of register `B` is not set.
            //     0x50 => self.BIT_b_r8(2, Register::B),
            //     // Set the zero flag if bit 2 of register `C` is not set.
            //     0x51 => self.BIT_b_r8(2, Register::C),
            //     // Set the zero flag if bit 2 of register `D` is not set.
            //     0x52 => self.BIT_b_r8(2, Register::D),
            //     // Set the zero flag if bit 2 of register `E` is not set.
            //     0x53 => self.BIT_b_r8(2, Register::E),
            //     // Set the zero flag if bit 2 of register `H` is not set.
            //     0x54 => self.BIT_b_r8(2, Register::H),
            //     // Set the zero flag if bit 2 of register `L` is not set.
            //     0x55 => self.BIT_b_r8(2, Register::L),
            //     // Set the zero flag if bit 2 of the value at address `HL` is not set.
            //     0x56 => self.BIT_b_r16(2, RegisterPair::HL),
            //     // Set the zero flag if bit 2 of register `A` is not set.
            //     0x57 => self.BIT_b_r8(2, Register::A),
            //     // Set the zero flag if bit 3 of register `B` is not set.
            //     0x58 => self.BIT_b_r8(3, Register::B),
            //     // Set the zero flag if bit 3 of register `C` is not set.
            //     0x59 => self.BIT_b_r8(3, Register::C),
            //     // Set the zero flag if bit 3 of register `D` is not set.
            //     0x5A => self.BIT_b_r8(3, Register::D),
            //     // Set the zero flag if bit 3 of register `E` is not set.
            //     0x5B => self.BIT_b_r8(3, Register::E),
            //     // Set the zero flag if bit 3 of register `H` is not set.
            //     0x5C => self.BIT_b_r8(3, Register::H),
            //     // Set the zero flag if bit 3 of register `L` is not set.
            //     0x5D => self.BIT_b_r8(3, Register::L),
            //     // Set the zero flag if bit 3 of the value at address `HL` is not set.
            //     0x5E => self.BIT_b_r16(3, RegisterPair::HL),
            //     // Set the zero flag if bit 3 of register `A` is not set.
            //     0x5F => self.BIT_b_r8(3, Register::A),
            //     // * 0x6_
            //     // Set the zero flag if bit 4 of register `B` is not set.
            //     0x60 => self.BIT_b_r8(4, Register::B),
            //     // Set the zero flag if bit 4 of register `C` is not set.
            //     0x61 => self.BIT_b_r8(4, Register::C),
            //     // Set the zero flag if bit 4 of register `D` is not set.
            //     0x62 => self.BIT_b_r8(4, Register::D),
            //     // Set the zero flag if bit 4 of register `E` is not set.
            //     0x63 => self.BIT_b_r8(4, Register::E),
            //     // Set the zero flag if bit 4 of register `H` is not set.
            //     0x64 => self.BIT_b_r8(4, Register::H),
            //     // Set the zero flag if bit 4 of register `L` is not set.
            //     0x65 => self.BIT_b_r8(4, Register::L),
            //     // Set the zero flag if bit 4 of the value at address `HL` is not set.
            //     0x66 => self.BIT_b_r16(4, RegisterPair::HL),
            //     // Set the zero flag if bit 4 of register `A` is not set.
            //     0x67 => self.BIT_b_r8(4, Register::A),
            //     // Set the zero flag if bit 5 of register `B` is not set.
            //     0x68 => self.BIT_b_r8(5, Register::B),
            //     // Set the zero flag if bit 5 of register `C` is not set.
            //     0x69 => self.BIT_b_r8(5, Register::C),
            //     // Set the zero flag if bit 5 of register `D` is not set.
            //     0x6A => self.BIT_b_r8(5, Register::D),
            //     // Set the zero flag if bit 5 of register `E` is not set.
            //     0x6B => self.BIT_b_r8(5, Register::E),
            //     // Set the zero flag if bit 5 of register `H` is not set.
            //     0x6C => self.BIT_b_r8(5, Register::H),
            //     // Set the zero flag if bit 5 of register `L` is not set.
            //     0x6D => self.BIT_b_r8(5, Register::L),
            //     // Set the zero flag if bit 5 of the value at address `HL` is not set.
            //     0x6E => self.BIT_b_r16(5, RegisterPair::HL),
            //     // Set the zero flag if bit 5 of register `A` is not set.
            //     0x6F => self.BIT_b_r8(5, Register::A),
            //     // * 0x7_
            //     // Set the zero flag if bit 6 of register `B` is not set.
            //     0x70 => self.BIT_b_r8(6, Register::B),
            //     // Set the zero flag if bit 6 of register `C` is not set.
            //     0x71 => self.BIT_b_r8(6, Register::C),
            //     // Set the zero flag if bit 6 of register `D` is not set.
            //     0x72 => self.BIT_b_r8(6, Register::D),
            //     // Set the zero flag if bit 6 of register `E` is not set.
            //     0x73 => self.BIT_b_r8(6, Register::E),
            //     // Set the zero flag if bit 6 of register `H` is not set.
            //     0x74 => self.BIT_b_r8(6, Register::H),
            //     // Set the zero flag if bit 6 of register `L` is not set.
            //     0x75 => self.BIT_b_r8(6, Register::L),
            //     // Set the zero flag if bit 6 of the value at address `HL` is not set.
            //     0x76 => self.BIT_b_r16(6, RegisterPair::HL),
            //     // Set the zero flag if bit 6 of register `A` is not set.
            //     0x77 => self.BIT_b_r8(6, Register::A),
            //     // Set the zero flag if bit 7 of register `B` is not set.
            //     0x78 => self.BIT_b_r8(7, Register::B),
            //     // Set the zero flag if bit 7 of register `C` is not set.
            //     0x79 => self.BIT_b_r8(7, Register::C),
            //     // Set the zero flag if bit 7 of register `D` is not set.
            //     0x7A => self.BIT_b_r8(7, Register::D),
            //     // Set the zero flag if bit 7 of register `E` is not set.
            //     0x7B => self.BIT_b_r8(7, Register::E),
            //     // Set the zero flag if bit 7 of register `H` is not set.
            //     0x7C => self.BIT_b_r8(7, Register::H),
            //     // Set the zero flag if bit 7 of register `L` is not set.
            //     0x7D => self.BIT_b_r8(7, Register::L),
            //     // Set the zero flag if bit 7 of the value at address `HL` is not set.
            //     0x7E => self.BIT_b_r16(7, RegisterPair::HL),
            //     // Set the zero flag if bit 7 of register `A` is not set.
            //     0x7F => self.BIT_b_r8(7, Register::A),
            //     // * 0x8_
            //     // Set bit 0 of register `B` to 0.
            //     0x80 => self.RES_b_r8(0, Register::B),
            //     // Set bit 0 of register `C` to 0.
            //     0x81 => self.RES_b_r8(0, Register::C),
            //     // Set bit 0 of register `D` to 0.
            //     0x82 => self.RES_b_r8(0, Register::D),
            //     // Set bit 0 of register `E` to 0.
            //     0x83 => self.RES_b_r8(0, Register::E),
            //     // Set bit 0 of register `H` to 0.
            //     0x84 => self.RES_b_r8(0, Register::H),
            //     // Set bit 0 of register `L` to 0.
            //     0x85 => self.RES_b_r8(0, Register::L),
            //     // Set bit 0 of the value at address `HL` to 0.
            //     0x86 => self.RES_b_r16(0, RegisterPair::HL),
            //     // Set bit 0 of register `A` to 0.
            //     0x87 => self.RES_b_r8(0, Register::A),
            //     // Set bit 1 of register `B` to 0.
            //     0x88 => self.RES_b_r8(1, Register::B),
            //     // Set bit 1 of register `C` to 0.
            //     0x89 => self.RES_b_r8(1, Register::C),
            //     // Set bit 1 of register `D` to 0.
            //     0x8A => self.RES_b_r8(1, Register::D),
            //     // Set bit 1 of register `E` to 0.
            //     0x8B => self.RES_b_r8(1, Register::E),
            //     // Set bit 1 of register `H` to 0.
            //     0x8C => self.RES_b_r8(1, Register::H),
            //     // Set bit 1 of register `L` to 0.
            //     0x8D => self.RES_b_r8(1, Register::L),
            //     // Set bit 1 of the value at address `HL` to 0.
            //     0x8E => self.RES_b_r16(1, RegisterPair::HL),
            //     // Set bit 1 of register `A` to 0.
            //     0x8F => self.RES_b_r8(1, Register::A),
            //     // * 0x9_
            //     // Set bit 2 of register `B` to 0.
            //     0x90 => self.RES_b_r8(2, Register::B),
            //     // Set bit 2 of register `C` to 0.
            //     0x91 => self.RES_b_r8(2, Register::C),
            //     // Set bit 2 of register `D` to 0.
            //     0x92 => self.RES_b_r8(2, Register::D),
            //     // Set bit 2 of register `E` to 0.
            //     0x93 => self.RES_b_r8(2, Register::E),
            //     // Set bit 2 of register `H` to 0.
            //     0x94 => self.RES_b_r8(2, Register::H),
            //     // Set bit 2 of register `L` to 0.
            //     0x95 => self.RES_b_r8(2, Register::L),
            //     // Set bit 2 of the value at address `HL` to 0.
            //     0x96 => self.RES_b_r16(2, RegisterPair::HL),
            //     // Set bit 2 of register `A` to 0.
            //     0x97 => self.RES_b_r8(2, Register::A),
            //     // Set bit 3 of register `B` to 0.
            //     0x98 => self.RES_b_r8(3, Register::B),
            //     // Set bit 3 of register `C` to 0.
            //     0x99 => self.RES_b_r8(3, Register::C),
            //     // Set bit 3 of register `D` to 0.
            //     0x9A => self.RES_b_r8(3, Register::D),
            //     // Set bit 3 of register `E` to 0.
            //     0x9B => self.RES_b_r8(3, Register::E),
            //     // Set bit 3 of register `H` to 0.
            //     0x9C => self.RES_b_r8(3, Register::H),
            //     // Set bit 3 of register `L` to 0.
            //     0x9D => self.RES_b_r8(3, Register::L),
            //     // Set bit 3 of the value at address `HL` to 0.
            //     0x9E => self.RES_b_r16(3, RegisterPair::HL),
            //     // Set bit 3 of register `A` to 0.
            //     0x9F => self.RES_b_r8(3, Register::A),
            //     // * 0xA_
            //     // Set bit 4 of register `B` to 0.
            //     0xA0 => self.RES_b_r8(4, Register::B),
            //     // Set bit 4 of register `C` to 0.
            //     0xA1 => self.RES_b_r8(4, Register::C),
            //     // Set bit 4 of register `D` to 0.
            //     0xA2 => self.RES_b_r8(4, Register::D),
            //     // Set bit 4 of register `E` to 0.
            //     0xA3 => self.RES_b_r8(4, Register::E),
            //     // Set bit 4 of register `H` to 0.
            //     0xA4 => self.RES_b_r8(4, Register::H),
            //     // Set bit 4 of register `L` to 0.
            //     0xA5 => self.RES_b_r8(4, Register::L),
            //     // Set bit 4 of the value at address `HL` to 0.
            //     0xA6 => self.RES_b_r16(4, RegisterPair::HL),
            //     // Set bit 4 of register `A` to 0.
            //     0xA7 => self.RES_b_r8(4, Register::A),
            //     // Set bit 5 of register `B` to 0.
            //     0xA8 => self.RES_b_r8(5, Register::B),
            //     // Set bit 5 of register `C` to 0.
            //     0xA9 => self.RES_b_r8(5, Register::C),
            //     // Set bit 5 of register `D` to 0.
            //     0xAA => self.RES_b_r8(5, Register::D),
            //     // Set bit 5 of register `E` to 0.
            //     0xAB => self.RES_b_r8(5, Register::E),
            //     // Set bit 5 of register `H` to 0.
            //     0xAC => self.RES_b_r8(5, Register::H),
            //     // Set bit 5 of register `L` to 0.
            //     0xAD => self.RES_b_r8(5, Register::L),
            //     // Set bit 5 of the value at address `HL` to 0.
            //     0xAE => self.RES_b_r16(5, RegisterPair::HL),
            //     // Set bit 5 of register `A` to 0.
            //     0xAF => self.RES_b_r8(5, Register::A),
            //     // * 0xB_
            //     // Set bit 6 of register `B` to 0.
            //     0xB0 => self.RES_b_r8(6, Register::B),
            //     // Set bit 6 of register `C` to 0.
            //     0xB1 => self.RES_b_r8(6, Register::C),
            //     // Set bit 6 of register `D` to 0.
            //     0xB2 => self.RES_b_r8(6, Register::D),
            //     // Set bit 6 of register `E` to 0.
            //     0xB3 => self.RES_b_r8(6, Register::E),
            //     // Set bit 6 of register `H` to 0.
            //     0xB4 => self.RES_b_r8(6, Register::H),
            //     // Set bit 6 of register `L` to 0.
            //     0xB5 => self.RES_b_r8(6, Register::L),
            //     // Set bit 6 of the value at address `HL` to 0.
            //     0xB6 => self.RES_b_r16(6, RegisterPair::HL),
            //     // Set bit 6 of register `A` to 0.
            //     0xB7 => self.RES_b_r8(6, Register::A),
            //     // Set bit 7 of register `B` to 0.
            //     0xB8 => self.RES_b_r8(7, Register::B),
            //     // Set bit 7 of register `C` to 0.
            //     0xB9 => self.RES_b_r8(7, Register::C),
            //     // Set bit 7 of register `D` to 0.
            //     0xBA => self.RES_b_r8(7, Register::D),
            //     // Set bit 7 of register `E` to 0.
            //     0xBB => self.RES_b_r8(7, Register::E),
            //     // Set bit 7 of register `H` to 0.
            //     0xBC => self.RES_b_r8(7, Register::H),
            //     // Set bit 7 of register `L` to 0.
            //     0xBD => self.RES_b_r8(7, Register::L),
            //     // Set bit 7 of the value at address `HL` to 0.
            //     0xBE => self.RES_b_r16(7, RegisterPair::HL),
            //     // Set bit 7 of register `A` to 0.
            //     0xBF => self.RES_b_r8(7, Register::A),
            //     // * 0xC_
            //     // Set bit 0 of register `B` to 1.
            //     0xC0 => self.SET_b_r8(0, Register::B),
            //     // Set bit 0 of register `C` to 1.
            //     0xC1 => self.SET_b_r8(0, Register::C),
            //     // Set bit 0 of register `D` to 1.
            //     0xC2 => self.SET_b_r8(0, Register::D),
            //     // Set bit 0 of register `E` to 1.
            //     0xC3 => self.SET_b_r8(0, Register::E),
            //     // Set bit 0 of register `H` to 1.
            //     0xC4 => self.SET_b_r8(0, Register::H),
            //     // Set bit 0 of register `L` to 1.
            //     0xC5 => self.SET_b_r8(0, Register::L),
            //     // Set bit 0 of the value at address `HL` to 1.
            //     0xC6 => self.SET_b_r16(0, RegisterPair::HL),
            //     // Set bit 0 of register `A` to 1.
            //     0xC7 => self.SET_b_r8(0, Register::A),
            //     // Set bit 1 of register `B` to 1.
            //     0xC8 => self.SET_b_r8(1, Register::B),
            //     // Set bit 1 of register `C` to 1.
            //     0xC9 => self.SET_b_r8(1, Register::C),
            //     // Set bit 1 of register `D` to 1.
            //     0xCA => self.SET_b_r8(1, Register::D),
            //     // Set bit 1 of register `E` to 1.
            //     0xCB => self.SET_b_r8(1, Register::E),
            //     // Set bit 1 of register `H` to 1.
            //     0xCC => self.SET_b_r8(1, Register::H),
            //     // Set bit 1 of register `L` to 1.
            //     0xCD => self.SET_b_r8(1, Register::L),
            //     // Set bit 1 of the value at address `HL` to 1.
            //     0xCE => self.SET_b_r16(1, RegisterPair::HL),
            //     // Set bit 1 of register `A` to 1.
            //     0xCF => self.SET_b_r8(1, Register::A),
            //     // * 0xD_
            //     // Set bit 2 of register `B` to 1.
            //     0xD0 => self.SET_b_r8(2, Register::B),
            //     // Set bit 2 of register `C` to 1.
            //     0xD1 => self.SET_b_r8(2, Register::C),
            //     // Set bit 2 of register `D` to 1.
            //     0xD2 => self.SET_b_r8(2, Register::D),
            //     // Set bit 2 of register `E` to 1.
            //     0xD3 => self.SET_b_r8(2, Register::E),
            //     // Set bit 2 of register `H` to 1.
            //     0xD4 => self.SET_b_r8(2, Register::H),
            //     // Set bit 2 of register `L` to 1.
            //     0xD5 => self.SET_b_r8(2, Register::L),
            //     // Set bit 2 of the value at address `HL` to 1.
            //     0xD6 => self.SET_b_r16(2, RegisterPair::HL),
            //     // Set bit 2 of register `A` to 1.
            //     0xD7 => self.SET_b_r8(2, Register::A),
            //     // Set bit 3 of register `B` to 1.
            //     0xD8 => self.SET_b_r8(3, Register::B),
            //     // Set bit 3 of register `C` to 1.
            //     0xD9 => self.SET_b_r8(3, Register::C),
            //     // Set bit 3 of register `D` to 1.
            //     0xDA => self.SET_b_r8(3, Register::D),
            //     // Set bit 3 of register `E` to 1.
            //     0xDB => self.SET_b_r8(3, Register::E),
            //     // Set bit 3 of register `H` to 1.
            //     0xDC => self.SET_b_r8(3, Register::H),
            //     // Set bit 3 of register `L` to 1.
            //     0xDD => self.SET_b_r8(3, Register::L),
            //     // Set bit 3 of the value at address `HL` to 1.
            //     0xDE => self.SET_b_r16(3, RegisterPair::HL),
            //     // Set bit 3 of register `A` to 1.
            //     0xDF => self.SET_b_r8(3, Register::A),
            //     // * 0xE_
            //     // Set bit 4 of register `B` to 1.
            //     0xE0 => self.SET_b_r8(4, Register::B),
            //     // Set bit 4 of register `C` to 1.
            //     0xE1 => self.SET_b_r8(4, Register::C),
            //     // Set bit 4 of register `D` to 1.
            //     0xE2 => self.SET_b_r8(4, Register::D),
            //     // Set bit 4 of register `E` to 1.
            //     0xE3 => self.SET_b_r8(4, Register::E),
            //     // Set bit 4 of register `H` to 1.
            //     0xE4 => self.SET_b_r8(4, Register::H),
            //     // Set bit 4 of register `L` to 1.
            //     0xE5 => self.SET_b_r8(4, Register::L),
            //     // Set bit 4 of the value at address `HL` to 1.
            //     0xE6 => self.SET_b_r16(4, RegisterPair::HL),
            //     // Set bit 4 of register `A` to 1.
            //     0xE7 => self.SET_b_r8(4, Register::A),
            //     // Set bit 5 of register `B` to 1.
            //     0xE8 => self.SET_b_r8(5, Register::B),
            //     // Set bit 5 of register `C` to 1.
            //     0xE9 => self.SET_b_r8(5, Register::C),
            //     // Set bit 5 of register `D` to 1.
            //     0xEA => self.SET_b_r8(5, Register::D),
            //     // Set bit 5 of register `E` to 1.
            //     0xEB => self.SET_b_r8(5, Register::E),
            //     // Set bit 5 of register `H` to 1.
            //     0xEC => self.SET_b_r8(5, Register::H),
            //     // Set bit 5 of register `L` to 1.
            //     0xED => self.SET_b_r8(5, Register::L),
            //     // Set bit 5 of the value at address `HL` to 1.
            //     0xEE => self.SET_b_r16(5, RegisterPair::HL),
            //     // Set bit 5 of register `A` to 1.
            //     0xEF => self.SET_b_r8(5, Register::A),
            //     // * 0xF_
            //     // Set bit 6 of register `B` to 1.
            //     0xF0 => self.SET_b_r8(6, Register::B),
            //     // Set bit 6 of register `C` to 1.
            //     0xF1 => self.SET_b_r8(6, Register::C),
            //     // Set bit 6 of register `D` to 1.
            //     0xF2 => self.SET_b_r8(6, Register::D),
            //     // Set bit 6 of register `E` to 1.
            //     0xF3 => self.SET_b_r8(6, Register::E),
            //     // Set bit 6 of register `H` to 1.
            //     0xF4 => self.SET_b_r8(6, Register::H),
            //     // Set bit 6 of register `L` to 1.
            //     0xF5 => self.SET_b_r8(6, Register::L),
            //     // Set bit 6 of the value at address `HL` to 1.
            //     0xF6 => self.SET_b_r16(6, RegisterPair::HL),
            //     // Set bit 6 of register `A` to 1.
            //     0xF7 => self.SET_b_r8(6, Register::A),
            //     // Set bit 7 of register `B` to 1.
            //     0xF8 => self.SET_b_r8(7, Register::B),
            //     // Set bit 7 of register `C` to 1.
            //     0xF9 => self.SET_b_r8(7, Register::C),
            //     // Set bit 7 of register `D` to 1.
            //     0xFA => self.SET_b_r8(7, Register::D),
            //     // Set bit 7 of register `E` to 1.
            //     0xFB => self.SET_b_r8(7, Register::E),
            //     // Set bit 7 of register `H` to 1.
            //     0xFC => self.SET_b_r8(7, Register::H),
            //     // Set bit 7 of register `L` to 1.
            //     0xFD => self.SET_b_r8(7, Register::L),
            //     // Set bit 7 of the value at address `HL` to 1.
            //     0xFE => self.SET_b_r16(7, RegisterPair::HL),
            //     // Set bit 7 of register `A` to 1.
            //     0xFF => self.SET_b_r8(7, Register::A),
            // }
            InstructionStep::Complete
        })
    })
}
