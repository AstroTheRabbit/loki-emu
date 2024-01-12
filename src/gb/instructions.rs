use super::{emu::GameBoyEmulator, utils::*};

impl GameBoyEmulator {
    pub fn run_instruction(&mut self, instruction: u8) {
        match instruction {
            // * 0X0_
            // No operation.
            0x00 => {}
            // Load the immediate value `n16` into register pair `BC`.
            0x01 => self.LD_r16_n16(RegisterPair::BC),
            // Load the value of `A` into the location of address `BC`.
            0x02 => self.LD_r16_r8(RegisterPair::BC, Register::A),
            // Increment register pair `BC`.
            0x03 => self.INC_r16(RegisterPair::BC),
            // Increment register `B`.
            0x04 => self.INC_r8(Register::B),
            // Decrement register `B`.
            0x05 => self.DEC_r8(Register::B),
            // Load the immediate value `n8` into register `B`.
            0x06 => self.LD_r8_n8(Register::B),
            // Rotate register `A` left, setting the carry flag to the previous bit 7.
            0x07 => {
                self.RLC_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RLC A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }
            // Load the value of register `SP` into the immediate address `a16`.
            0x08 => self.LD_a16_r16(RegisterPair::SP),
            // Add register pairs `HL` and `BC`, storing the result in `HL`.
            0x09 => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::BC),
            // Load the value at address `BC` into register `A`.
            0x0A => self.LD_r8_r16(Register::A, RegisterPair::BC),
            // Decrement register pair `BC`.
            0x0B => self.DEC_r16(RegisterPair::BC),
            // Increment register `C`.
            0x0C => self.INC_r8(Register::C),
            // Decrement register `C`.
            0x0D => self.DEC_r8(Register::C),
            // Load immediate value `n8` into register `C`.
            0x0E => self.LD_r8_n8(Register::C),
            // Rotate register `A` right, setting the carry flag to the previous bit 0.
            0x0F => {
                self.RRC_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RRC A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }
            // * 0x1_
            // Stop the CPU & LCD display until a button is pressed (the byte after is also skipped).
            0x10 => todo!(),
            // Load the immediate value `n16` into register pair `DE`.
            0x11 => self.LD_r16_n16(RegisterPair::DE),
            // Load the value of `A` into the location of address `DE`.
            0x12 => self.LD_r16_r8(RegisterPair::DE, Register::A),
            // Increment register pair `DE`.
            0x13 => self.INC_r16(RegisterPair::DE),
            // Increment register `D`.
            0x14 => self.INC_r8(Register::D),
            // Decrement register `D`.
            0x15 => self.DEC_r8(Register::D),
            // Load the immediate value `n8` into register `D`.
            0x16 => self.LD_r8_n8(Register::D),
            // Rotate register `A` and the carry flag left.
            0x17 => {
                self.RL_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RL A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }
            // Add the signed immediate value `e8` to the `PC` and jump to it.
            0x18 => self.JR_e8(),
            // Add register pairs `HL` and `DE`, storing the result in `HL`.
            0x19 => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::DE),
            // Load the value at address `DE` into register `A`.
            0x1A => self.LD_r8_r16(Register::A, RegisterPair::DE),
            // Decrement register pair `DE`.
            0x1B => self.DEC_r16(RegisterPair::DE),
            // Increment register `D`.
            0x1C => self.INC_r8(Register::E),
            // Decrement register `D`.
            0x1D => self.DEC_r8(Register::E),
            // Load the immediate value `n8` into register `E`.
            0x1E => self.LD_r8_n8(Register::E),
            // Rotate register `A` and the carry flag right.
            0x1F => {
                self.RR_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RR A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }
            // * 0x2_
            // Add the signed immediate value `e8` to the `PC` and jump to it if the zero flag is not set.
            0x20 => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.JR_e8();
                }
            }
            // Load the immediate value `n16` into register pair `HL`.
            0x21 => self.LD_r16_n16(RegisterPair::HL),
            // Load the value of `A` into the location of address `HL`, then increment register pair `HL`.
            0x22 => {
                self.LD_r16_r8(RegisterPair::HL, Register::A);
                self.INC_r16(RegisterPair::HL);
            }
            // Increment register pair `HL`.
            0x23 => self.INC_r16(RegisterPair::HL),
            // Increment register `H`.
            0x24 => self.INC_r8(Register::H),
            // Decrement register `H`.
            0x25 => self.DEC_r8(Register::H),
            // Load the immediate value `n8` into register `H`.
            0x26 => self.LD_r8_n8(Register::H),
            // Decimal Adjust Accumulator.
            0x27 => todo!(),
            // Add the signed immediate value `e8` to the `PC` and jump to it if the zero flag is set.
            0x28 => {
                if self.cpu.get_flag(Flag::Z) {
                    self.JR_e8();
                }
            }
            // Add register pair `HL` to itself, storing the result in `HL`.
            0x29 => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::HL),
            // Load the value at address `HL` into register `A`, then increment register pair `HL`.
            0x2A => {
                self.LD_r8_r16(Register::A, RegisterPair::HL);
                self.INC_r16(RegisterPair::HL);
            }
            // Decrement register pair `HL`.
            0x2B => self.DEC_r16(RegisterPair::HL),
            // Increment register `L`.
            0x2C => self.INC_r8(Register::L),
            // Decrement register `L`.
            0x2D => self.DEC_r8(Register::L),
            // Load the immediate value `n8` into register `L`.
            0x2E => self.LD_r8_n8(Register::L),
            // Bitwise invert register `A`.
            0x2F => {
                let v = self.cpu.get_register(Register::A);
                self.cpu.set_register(Register::A, !v);
                self.cpu.set_flag(Flag::N | Flag::H, true);
            }
            // * 0x3_
            // Add the signed immediate value `e8` to the `PC` and jump to it if the carry flag is not set.
            0x30 => {
                if !self.cpu.get_flag(Flag::C) {
                    self.JR_e8();
                }
            }
            // Load the immdediate value `n16` into the `SP`.
            0x31 => self.LD_r16_n16(RegisterPair::SP),
            // Load the value of `A` into the location of address `HL`, then decrement register pair `HL`.
            0x32 => {
                self.LD_r16_r8(RegisterPair::HL, Register::A);
                self.DEC_r16(RegisterPair::HL);
            }
            // Increment the `SP`.
            0x33 => self.INC_r16(RegisterPair::SP),
            // Increment the value at address `HL`.
            0x34 => {
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                let value = self.bus.read(address) + 1;
                self.bus.write(address, value);
            }
            // Decrement the value at address `HL`.
            0x35 => {
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                let value = self.bus.read(address) - 1;
                self.bus.write(address, value);
            }
            // Load the immediate value `n8` into the value at address `HL`.
            0x36 => {
                let value = self.read_u8(RegisterPair::PC);
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                self.bus.write(address, value);
            }
            // Set the carry flag.
            0x37 => self.cpu.set_flag(Flag::C, true),
            // Add the signed immediate value `e8` to the `PC` and jump to it if the carry flag is set.
            0x38 => {
                if self.cpu.get_flag(Flag::C) {
                    self.JR_e8();
                }
            }
            // Add the `SP` to register pair `HL`.
            0x39 => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::SP),
            // Load the value at address `HL` into register `A`, then decrement register pair `HL`.
            0x3A => {
                self.LD_r8_r16(Register::A, RegisterPair::HL);
                self.DEC_r16(RegisterPair::HL);
            }
            // Decrement the `SP`.
            0x3B => self.DEC_r16(RegisterPair::SP),
            // Increment register `A`.
            0x3C => self.INC_r8(Register::A),
            // Decrement register `A`.
            0x3D => self.DEC_r8(Register::A),
            // Load the immediate value `n8` into register `A`.
            0x3E => self.LD_r8_n8(Register::A),
            // Invert the carry flag.
            0x3F => {
                self.cpu.toggle_flag(Flag::C);
                self.cpu.set_flag(Flag::N | Flag::H, false);
            }
            // * 0x4_
            // Load register `B` into itself.
            0x40 => self.LD_r8_r8(Register::B, Register::B),
            // Load register `C` into register `B`.
            0x41 => self.LD_r8_r8(Register::B, Register::C),
            // Load register `D` into register `B`.
            0x42 => self.LD_r8_r8(Register::B, Register::D),
            // Load register `E` into register `B`.
            0x43 => self.LD_r8_r8(Register::B, Register::E),
            // Load register `H` into register `B`.
            0x44 => self.LD_r8_r8(Register::B, Register::H),
            // Load register `L` into register `B`.
            0x45 => self.LD_r8_r8(Register::B, Register::L),
            // Load the value at address `HL` into register `B`.
            0x46 => self.LD_r8_r16(Register::B, RegisterPair::HL),
            // Load register `A` into register `B`.
            0x47 => self.LD_r8_r8(Register::B, Register::A),
            // Load register `B` into register `C`.
            0x48 => self.LD_r8_r8(Register::C, Register::B),
            // Load register `C` into itself.
            0x49 => self.LD_r8_r8(Register::C, Register::C),
            // Load register `D` into register `C`.
            0x4A => self.LD_r8_r8(Register::C, Register::D),
            // Load register `E` into register `C`.
            0x4B => self.LD_r8_r8(Register::C, Register::E),
            // Load register `H` into register `C`.
            0x4C => self.LD_r8_r8(Register::C, Register::H),
            // Load register `L` into register `C`.
            0x4D => self.LD_r8_r8(Register::C, Register::L),
            // Load the value at address `HL` into register `C`.
            0x4E => self.LD_r8_r16(Register::C, RegisterPair::HL),
            // Load register `A` into register `C`.
            0x4F => self.LD_r8_r8(Register::C, Register::A),
            // * 0x5_
            // Load register `B` into register `D`.
            0x50 => self.LD_r8_r8(Register::D, Register::B),
            // Load register `C` into register `D`.
            0x51 => self.LD_r8_r8(Register::D, Register::C),
            // Load register `D` into itself.
            0x52 => self.LD_r8_r8(Register::D, Register::D),
            // Load register `E` into register `D`.
            0x53 => self.LD_r8_r8(Register::D, Register::E),
            // Load register `H` into register `D`.
            0x54 => self.LD_r8_r8(Register::D, Register::H),
            // Load register `B` into register `D`.
            0x55 => self.LD_r8_r8(Register::D, Register::L),
            // Load the value at address `HL` into register `D`.
            0x56 => self.LD_r8_r16(Register::D, RegisterPair::HL),
            // Load register `A` into register `D`.
            0x57 => self.LD_r8_r8(Register::D, Register::A),
            // Load register `B` into register `E`.
            0x58 => self.LD_r8_r8(Register::E, Register::B),
            // Load register `C` into register `E`.
            0x59 => self.LD_r8_r8(Register::E, Register::C),
            // Load register `D` into register `E`.
            0x5A => self.LD_r8_r8(Register::E, Register::D),
            // Load register `E` into itself.
            0x5B => self.LD_r8_r8(Register::E, Register::E),
            // Load register `H` into register `E`.
            0x5C => self.LD_r8_r8(Register::E, Register::H),
            // Load register `L` into register `E`.
            0x5D => self.LD_r8_r8(Register::E, Register::L),
            // Load the value at address `HL` into register `E`.
            0x5E => self.LD_r8_r16(Register::E, RegisterPair::HL),
            // Load register `A` into register `E`.
            0x5F => self.LD_r8_r8(Register::E, Register::A),
            // * 0x6_
            // Load register `B` into register `H`.
            0x60 => self.LD_r8_r8(Register::H, Register::B),
            // Load register `C` into register `H`.
            0x61 => self.LD_r8_r8(Register::H, Register::C),
            // Load register `D` into register `H`.
            0x62 => self.LD_r8_r8(Register::H, Register::D),
            // Load register `E` into register `H`.
            0x63 => self.LD_r8_r8(Register::H, Register::E),
            // Load register `H` into itself.
            0x64 => self.LD_r8_r8(Register::H, Register::H),
            // Load register `L` into register `H`.
            0x65 => self.LD_r8_r8(Register::H, Register::L),
            // Load the value at address `HL` into register `H`.
            0x66 => self.LD_r8_r16(Register::H, RegisterPair::HL),
            // Load register `A` into register `H`.
            0x67 => self.LD_r8_r8(Register::H, Register::A),
            // Load register `B` into register `L`.
            0x68 => self.LD_r8_r8(Register::L, Register::B),
            // Load register `C` into register `L`.
            0x69 => self.LD_r8_r8(Register::L, Register::C),
            // Load register `D` into register `L`.
            0x6A => self.LD_r8_r8(Register::L, Register::D),
            // Load register `E` into register `L`.
            0x6B => self.LD_r8_r8(Register::L, Register::E),
            // Load register `H` into register `L`.
            0x6C => self.LD_r8_r8(Register::L, Register::H),
            // Load register `L` into itself.
            0x6D => self.LD_r8_r8(Register::L, Register::L),
            // Load the value at address `HL` into register `L`.
            0x6E => self.LD_r8_r16(Register::L, RegisterPair::HL),
            // Load register `A` into register `L`.
            0x6F => self.LD_r8_r8(Register::L, Register::A),
            // * 0x7_
            // Load register `B` into the location of address `HL`.
            0x70 => self.LD_r16_r8(RegisterPair::HL, Register::B),
            // Load register `C` into the location of address `HL`.
            0x71 => self.LD_r16_r8(RegisterPair::HL, Register::C),
            // Load register `D` into the location of address `HL`.
            0x72 => self.LD_r16_r8(RegisterPair::HL, Register::D),
            // Load register `E` into the location of address `HL`.
            0x73 => self.LD_r16_r8(RegisterPair::HL, Register::E),
            // Load register `H` into the location of address `HL`.
            0x74 => self.LD_r16_r8(RegisterPair::HL, Register::H),
            // Load register `L` into the location of address `HL`.
            0x75 => self.LD_r16_r8(RegisterPair::HL, Register::L),
            // Power down the CPU until an interrupt occurs.
            0x76 => todo!(),
            // Load register `A` into the location of address `HL`.
            0x77 => self.LD_r16_r8(RegisterPair::HL, Register::A),
            // Load register `B` into register `A`.
            0x78 => self.LD_r8_r8(Register::A, Register::B),
            // Load register `C` into register `A`.
            0x79 => self.LD_r8_r8(Register::A, Register::C),
            // Load register `D` into register `A`.
            0x7A => self.LD_r8_r8(Register::A, Register::D),
            // Load register `E` into register `A`.
            0x7B => self.LD_r8_r8(Register::A, Register::E),
            // Load register `H` into register `A`.
            0x7C => self.LD_r8_r8(Register::A, Register::H),
            // Load register `L` into register `A`.
            0x7D => self.LD_r8_r8(Register::A, Register::L),
            // Load the value at address `HL` into register `A`.
            0x7E => self.LD_r8_r16(Register::A, RegisterPair::HL),
            // Load register `A` into itself.
            0x7F => self.LD_r8_r8(Register::A, Register::A),
            // * 0x8_
            // Add register `B` to register `A`.
            0x80 => self.ADD_r8_r8(Register::A, Register::B),
            // Add register `C` to register `A`.
            0x81 => self.ADD_r8_r8(Register::A, Register::C),
            // Add register `D` to register `A`.
            0x82 => self.ADD_r8_r8(Register::A, Register::D),
            // Add register `E` to register `A`.
            0x83 => self.ADD_r8_r8(Register::A, Register::E),
            // Add register `H` to register `A`.
            0x84 => self.ADD_r8_r8(Register::A, Register::H),
            // Add register `L` to register `A`.
            0x85 => self.ADD_r8_r8(Register::A, Register::L),
            // Add the value at address `HL` to register `A`.
            0x86 => self.ADD_r8_r16(Register::A, RegisterPair::HL),
            // Add register `A` to itself.
            0x87 => self.ADD_r8_r8(Register::A, Register::A),
            // Add register `B` and the carry flag to register `A`.
            0x88 => self.ADC_r8_r8(Register::A, Register::B),
            // Add register `B` and the carry flag to register `A`.
            0x89 => self.ADC_r8_r8(Register::A, Register::C),
            // Add register `D` and the carry flag to register `A`.
            0x8A => self.ADC_r8_r8(Register::A, Register::D),
            // Add register `E` and the carry flag to register `A`.
            0x8B => self.ADC_r8_r8(Register::A, Register::E),
            // Add register `H` and the carry flag to register `A`.
            0x8C => self.ADC_r8_r8(Register::A, Register::H),
            // Add register `L` and the carry flag to register `A`.
            0x8D => self.ADC_r8_r8(Register::A, Register::L),
            // Add the value at address `HL` and the carry flag to register `A`.
            0x8E => self.ADC_r8_r16(Register::A, RegisterPair::HL),
            // Add register `A` and the carry flag to `A`.
            0x8F => self.ADC_r8_r8(Register::A, Register::A),
            // * 0x9_
            // Subtract register `B` from register `A`.
            0x90 => self.SUB_r8_r8(Register::A, Register::B),
            // Subtract register `C` from register `A`.
            0x91 => self.SUB_r8_r8(Register::A, Register::C),
            // Subtract register `D` from register `A`.
            0x92 => self.SUB_r8_r8(Register::A, Register::D),
            // Subtract register `E` from register `A`.
            0x93 => self.SUB_r8_r8(Register::A, Register::E),
            // Subtract register `H` from register `A`.
            0x94 => self.SUB_r8_r8(Register::A, Register::H),
            // Subtract register `L` from register `A`.
            0x95 => self.SUB_r8_r8(Register::A, Register::L),
            // Subtract the value at address `HL` from register `A`.
            0x96 => self.SUB_r8_r16(Register::A, RegisterPair::HL),
            // Subtract register `A` from itself.
            0x97 => self.SUB_r8_r8(Register::A, Register::A),
            // Subtract register `B` and the carry flag from register `A`.
            0x98 => self.SBC_r8_r8(Register::A, Register::B),
            // Subtract register `B` and the carry flag from register `A`.
            0x99 => self.SBC_r8_r8(Register::A, Register::C),
            // Subtract register `D` and the carry flag from register `A`.
            0x9A => self.SBC_r8_r8(Register::A, Register::D),
            // Subtract register `E` and the carry flag from register `A`.
            0x9B => self.SBC_r8_r8(Register::A, Register::E),
            // Subtract register `H` and the carry flag from register `A`.
            0x9C => self.SBC_r8_r8(Register::A, Register::H),
            // Subtract register `L` and the carry flag from register `A`.
            0x9D => self.SBC_r8_r8(Register::A, Register::L),
            // Subtract the value at address `HL` and the carry flag from register `A`.
            0x9E => self.SBC_r8_r16(Register::A, RegisterPair::HL),
            // Subtract register `A` and the carry flag from `A`.
            0x9F => self.SBC_r8_r8(Register::A, Register::A),
            // * 0xA_
            // Bitwise AND registers `A` and `B`, storing the result in `A`.
            0xA0 => self.AND_r8_r8(Register::A, Register::B),
            // Bitwise AND registers `A` and `C`, storing the result in `A`.
            0xA1 => self.AND_r8_r8(Register::A, Register::C),
            // Bitwise AND registers `A` and `D`, storing the result in `A`.
            0xA2 => self.AND_r8_r8(Register::A, Register::D),
            // Bitwise AND registers `A` and `E`, storing the result in `A`.
            0xA3 => self.AND_r8_r8(Register::A, Register::E),
            // Bitwise AND registers `A` and `H`, storing the result in `A`.
            0xA4 => self.AND_r8_r8(Register::A, Register::H),
            // Bitwise AND registers `A` and `L`, storing the result in `A`.
            0xA5 => self.AND_r8_r8(Register::A, Register::L),
            // Bitwise AND register `A` and the value at address `HL` storing the result in `A`.
            0xA6 => self.AND_r8_r16(Register::A, RegisterPair::HL),
            // Bitwise AND register `A` with itself, storing the result in `A`.
            0xA7 => self.AND_r8_r8(Register::A, Register::A),
            // Bitwise XOR registers `A` and `B`, storing the result in `A`.
            0xA8 => self.XOR_r8_r8(Register::A, Register::B),
            // Bitwise XOR registers `A` and `C`, storing the result in `A`.
            0xA9 => self.XOR_r8_r8(Register::A, Register::C),
            // Bitwise XOR registers `A` and `D`, storing the result in `A`.
            0xAA => self.XOR_r8_r8(Register::A, Register::D),
            // Bitwise XOR registers `A` and `E`, storing the result in `A`.
            0xAB => self.XOR_r8_r8(Register::A, Register::E),
            // Bitwise XOR registers `A` and `H`, storing the result in `A`.
            0xAC => self.XOR_r8_r8(Register::A, Register::H),
            // Bitwise XOR registers `A` and `L`, storing the result in `A`.
            0xAD => self.XOR_r8_r8(Register::A, Register::L),
            // Bitwise XOR register `A` and the value at address `HL` storing the result in `A`.
            0xAE => self.XOR_r8_r16(Register::A, RegisterPair::HL),
            // Bitwise XOR register `A` with itself, storing the result in `A`.
            0xAF => self.XOR_r8_r8(Register::A, Register::A),
            // * 0xB_
            // Bitwise OR registers `A` and `B`, storing the result in `A`.
            0xB0 => self.OR_r8_r8(Register::A, Register::B),
            // Bitwise OR registers `A` and `C`, storing the result in `A`.
            0xB1 => self.OR_r8_r8(Register::A, Register::C),
            // Bitwise OR registers `A` and `D`, storing the result in `A`.
            0xB2 => self.OR_r8_r8(Register::A, Register::D),
            // Bitwise OR registers `A` and `E`, storing the result in `A`.
            0xB3 => self.OR_r8_r8(Register::A, Register::E),
            // Bitwise OR registers `A` and `H`, storing the result in `A`.
            0xB4 => self.OR_r8_r8(Register::A, Register::H),
            // Bitwise OR registers `A` and `L`, storing the result in `A`.
            0xB5 => self.OR_r8_r8(Register::A, Register::L),
            // Bitwise OR register `A` and the value at address `HL` storing the result in `A`.
            0xB6 => self.OR_r8_r16(Register::A, RegisterPair::HL),
            // Bitwise OR register `A` with itself, storing the result in `A`.
            0xB7 => self.OR_r8_r8(Register::A, Register::A),
            // Subtract register `B` from register `A`, but do not store the result.
            0xB8 => self.CP_r8_r8(Register::A, Register::B),
            // Subtract register `C` from register `A`, but do not store the result.
            0xB9 => self.CP_r8_r8(Register::A, Register::C),
            // Subtract register `D` from register `A`, but do not store the result.
            0xBA => self.CP_r8_r8(Register::A, Register::D),
            // Subtract register `E` from register `A`, but do not store the result.
            0xBB => self.CP_r8_r8(Register::A, Register::E),
            // Subtract register `H` from register `A`, but do not store the result.
            0xBC => self.CP_r8_r8(Register::A, Register::H),
            // Subtract register `L` from register `A`, but do not store the result.
            0xBD => self.CP_r8_r8(Register::A, Register::L),
            // Subtract the value at address `HL` from register `A`, but do not store the result.
            0xBE => self.CP_r8_r16(Register::A, RegisterPair::HL),
            // Subtract register `A` from itself, but do not store the result.
            0xBF => self.CP_r8_r8(Register::A, Register::A),
            // * 0xC_
            // Return from subroutine if the zero flag is not set.
            0xC0 => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.RET();
                }
            }
            // Pop from the stack to register pair `BC`.
            0xC1 => self.POP_r16(RegisterPair::BC),
            // Jump to the immediate address `a16` if the zero flag is not set.
            0xC2 => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.JP_a16();
                }
            }
            // Jump to the immediate address `a16`.
            0xC3 => self.JP_a16(),
            // Call the immediate address `a16` if the zero flag is not set.
            0xC4 => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.CALL_a16();
                }
            }
            // Push register pair `BC` into the stack.
            0xC5 => self.PUSH_r16(RegisterPair::BC),
            // Add the immediate value `n8` to register `A`.
            0xC6 => self.ADD_r8_n8(Register::A),
            // Call the address `0x00`.
            0xC7 => self.RST_a16(0x0000),
            // Return from subroutine if the zero flag is set.
            0xC8 => {
                if self.cpu.get_flag(Flag::Z) {
                    self.RET();
                }
            }
            // Return from subroutine.
            0xC9 => self.RET(),
            // Jump to the immediate address `a16` if the zero flag is set.
            0xCA => {
                if self.cpu.get_flag(Flag::Z) {
                    self.JP_a16();
                }
            }
            // Access to secondary prefixed instructions.
            0xCB => self.PREFIX_n8(),
            // Call the immediate address `a16` if the zero flag is set.
            0xCC => {
                if self.cpu.get_flag(Flag::Z) {
                    self.CALL_a16();
                }
            }
            // Call the immediate address `a16`.
            0xCD => self.CALL_a16(),
            // Add the immediate value `n8` and the carry flag to register `A`.
            0xCE => self.ADC_r8_n8(Register::A),
            // Call the address `0x08`.
            0xCF => self.RST_a16(0x0008),
            // * 0xD_
            // Return from subroutine if the carry flag is not set.
            0xD0 => {
                if !self.cpu.get_flag(Flag::C) {
                    self.RET();
                }
            }
            // Pop from the stack to register pair `DE`.
            0xD1 => self.POP_r16(RegisterPair::DE),
            // Jump to the immediate address `a16` if the carry flag is not set.
            0xD2 => {
                if !self.cpu.get_flag(Flag::C) {
                    self.JP_a16();
                }
            }
            // Call the immediate address `a16` if the carry flag is not set.
            0xD4 => {
                if !self.cpu.get_flag(Flag::C) {
                    self.CALL_a16();
                }
            }
            // Push register pair `DE` into the stack.
            0xD5 => self.PUSH_r16(RegisterPair::DE),
            // Subtract the immediate value `n8` from register `A`.
            0xD6 => self.SUB_r8_n8(Register::A),
            // Call the address `0x10`.
            0xD7 => self.RST_a16(0x0010),
            // Return from subroutine if the carry flag is set.
            0xD8 => {
                if self.cpu.get_flag(Flag::C) {
                    self.RET();
                }
            }
            // Return from subroutine and enable interupts.
            0xD9 => {
                self.ime = IME::Enabled;
                self.RET();
            }
            // Jump to the immediate address `a16` if the carry flag is set.
            0xDA => {
                if self.cpu.get_flag(Flag::C) {
                    self.JP_a16();
                }
            }
            // Call the immediate address `a16` if the carry flag is set.
            0xDC => {
                if self.cpu.get_flag(Flag::C) {
                    self.CALL_a16();
                }
            }
            // Subtract the immediate value `n8` and the carry flag from register `A`.
            0xDE => self.SBC_r8_n8(Register::A),
            // Call the address `0x18`.
            0xDF => self.RST_a16(0x0018),
            // * 0xE_
            // Load register `A` into the location of immediate address `0xFF00 + a8`.
            0xE0 => {
                let address = join_u16(self.read_u8(RegisterPair::PC), 0xFF);
                let value = self.cpu.get_register(Register::A);
                self.bus.write(address, value);
            }
            // Pop from the stack to register pair `HL`.
            0xE1 => self.POP_r16(RegisterPair::HL),
            // Load register `A` into the location of address `0xFF00 + C`.
            0xE2 => {
                let address = join_u16(self.cpu.get_register(Register::C), 0xFF);
                let value = self.cpu.get_register(Register::A);
                self.bus.write(address, value);
            }
            // Push register pair `HL` into the stack.
            0xE5 => self.PUSH_r16(RegisterPair::HL),
            // Bitwise AND register `A` and the immediate value `n8`, storing the result in `A`.
            0xE6 => self.ADD_r8_n8(Register::A),
            // Call the address `0x20`.
            0xE7 => self.RST_a16(0x0020),
            // Add the immediate signed value `e8` to the `SP`.
            0xE8 => {
                // ? Same as JR_e8 but for `SP` instead of `PC`.
                let v = self.read_u8(RegisterPair::SP) as i8;
                let r = self.cpu.get_register_pair(RegisterPair::SP);
                self.cpu
                    .set_register_pair(RegisterPair::SP, r.wrapping_add_signed(v.into()));
            }
            // Jump to the location of address `HL`.
            0xE9 => {
                let v = self.cpu.get_register_pair(RegisterPair::HL);
                self.cpu.set_register_pair(RegisterPair::PC, v);
            }
            // Load register `A` into the location of immediate address `a16`.
            0xEA => {
                let address = self.read_u16(RegisterPair::PC);
                let value = self.cpu.get_register(Register::A);
                self.bus.write(address, value);
            }
            // Bitwise XOR register `A` and the immediate value `n8`, storing the result in `A`.
            0xEE => {
                let val = self.read_u8(RegisterPair::PC);
                let v = self.cpu.xor_register(Register::A, val);
                self.cpu.set_register(Register::A, v);

                self.cpu.set_flag(Flag::Z, v == 0);
                self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
            }
            // Call the address `0x28`.
            0xEF => self.RST_a16(0x0028),
            // * 0xF_
            // Load the value at immediate address `0xFF00 + a8` into register `A`.
            0xF0 => {
                let address = join_u16(self.read_u8(RegisterPair::PC), 0xFF);
                let v = self.bus.read(address);
                self.cpu.set_register(Register::A, v);
            }
            // Pop from the stack to register pair `AF`.
            0xF1 => self.POP_r16(RegisterPair::AF),
            // Load register `A` into the location of address `0xFF00 + C`.
            0xF2 => {
                let address = join_u16(self.cpu.get_register(Register::C), 0xFF);
                let v = self.bus.read(address);
                self.cpu.set_register(Register::A, v);
            }
            // Disable interrupts by clearing the IME flag.
            0xF3 => self.ime = IME::Disabled,
            // Push register pair `AF` into the stack.
            0xF5 => self.PUSH_r16(RegisterPair::AF),
            // Bitwise OR register `A` and the immediate value `n8`, storing the result in `A`.
            0xF6 => {
                let val = self.read_u8(RegisterPair::PC);
                let v = self.cpu.or_register(Register::A, val);
                self.cpu.set_register(Register::A, v);

                self.cpu.set_flag(Flag::Z, v == 0);
                self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
            }
            // Call the address `0x30`.
            0xF7 => self.RST_a16(0x0030),
            // Load the addition of the signed immediate value `e8` and the `SP` into register pair `HL`.
            0xF8 => {
                let v = self.read_u8(RegisterPair::PC) as i8;
                let r = self.cpu.get_register_pair(RegisterPair::SP);
                self.cpu
                    .set_register_pair(RegisterPair::HL, r.wrapping_add_signed(v.into()));
            }
            // Load register pair `HL` into the `SP`.
            0xF9 => {
                let v = self.cpu.get_register_pair(RegisterPair::HL);
                self.cpu.set_register_pair(RegisterPair::SP, v);
            }
            // Load the value at immediate address `a16` into register `A`.
            0xFA => {
                let address = self.read_u16(RegisterPair::PC);
                let v = self.bus.read(address);
                self.cpu.set_register(Register::A, v);
            }
            // Enable interrupts by setting the IME flag after the next instruction.
            0xFB => self.ime = IME::Scheduled,
            // Subtract the immediate value `n8` from register `A`, but do not store the result.
            0xFE => {
                let val = self.read_u8(RegisterPair::PC);
                let _ = self.cpu.sub_register(Register::A, val);
            }
            // Call the address `0x38`.
            0xFF => self.RST_a16(0x0038),
            _ => panic!("GB - Invalid instruction!"),
        }
    }
}