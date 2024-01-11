use super::{bus::Bus, cpu::CPU, instructions::Instruction, utils::*};

#[derive(Debug)]
pub struct GameBoyEmulator {
    pub cpu: CPU,
    pub bus: Bus,
    pub ime: bool,
}

impl GameBoyEmulator {
    pub fn step(&mut self) {
        todo!();
    }

    /// Read and return a byte from address `r16`, then increment `r16`.
    #[inline]
    pub fn read_u8(&mut self, r16: RegisterPair) -> u8 {
        let address = self.cpu.get_register_pair(r16);
        self.cpu.increment_register_pair(r16);
        return self.bus.read(address);
    }

    /// Run [`GameBoyEmulator::read_u8`] twice, returning the two bytes and incrementing `r16` twice.
    #[inline]
    pub fn read_u16(&mut self, r16: RegisterPair) -> u16 {
        return join_u16(self.read_u8(r16), self.read_u8(r16));
    }

    /// Write a value to the stack, decrementing the `SP` twice.
    pub fn write_stack(&mut self, val: u16) {
        let (lsb, msb) = split_u16(val);
        self.cpu.decrement_register_pair(RegisterPair::SP);
        let address = self.cpu.get_register_pair(RegisterPair::SP);
        self.bus.write(address, msb);
        self.cpu.decrement_register_pair(RegisterPair::SP);
        let address = self.cpu.get_register_pair(RegisterPair::SP);
        self.bus.write(address, lsb);
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            // * 0X0_
            Instruction::NOP => {}
            Instruction::LD_BC_n16 => self.LD_r16_n16(RegisterPair::BC),
            Instruction::LD_BC_A => self.LD_r16_r8(RegisterPair::BC, Register::A),
            Instruction::INC_BC => self.INC_r16(RegisterPair::BC),
            Instruction::INC_B => self.INC_r8(Register::B),
            Instruction::DEC_B => self.DEC_r8(Register::B),
            Instruction::LD_B_n8 => self.LD_r8_n8(Register::B),
            Instruction::RLCA => {
                self.RLC_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RLC A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }
            Instruction::LD_a16_SP => self.LD_a16_r16(RegisterPair::SP),
            Instruction::ADD_HL_BC => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::BC),
            Instruction::LD_A_BC => self.LD_r8_r16(Register::A, RegisterPair::BC),
            Instruction::DEC_BC => self.DEC_r16(RegisterPair::BC),
            Instruction::INC_C => self.INC_r8(Register::C),
            Instruction::DEC_C => self.DEC_r8(Register::C),
            Instruction::LD_C_n8 => self.LD_r8_n8(Register::C),
            Instruction::RRCA => {
                self.RRC_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RRC A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }

            // * 0x1_
            Instruction::STOP => todo!(),
            Instruction::LD_DE_n16 => self.LD_r16_n16(RegisterPair::DE),
            Instruction::LD_DE_A => self.LD_r16_r8(RegisterPair::DE, Register::A),
            Instruction::INC_DE => self.INC_r16(RegisterPair::DE),
            Instruction::INC_D => self.INC_r8(Register::D),
            Instruction::DEC_D => self.DEC_r8(Register::D),
            Instruction::LD_D_n8 => self.LD_r8_n8(Register::D),
            Instruction::RLA => {
                self.RL_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RL A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }
            Instruction::JR_e8 => self.JR_e8(),
            Instruction::ADD_HL_DE => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::DE),
            Instruction::LD_A_DE => self.LD_r8_r16(Register::A, RegisterPair::DE),
            Instruction::DEC_DE => self.DEC_r16(RegisterPair::DE),
            Instruction::INC_E => self.INC_r8(Register::E),
            Instruction::DEC_E => self.DEC_r8(Register::E),
            Instruction::LD_E_n8 => self.LD_r8_n8(Register::E),
            Instruction::RRA => {
                self.RR_r8(Register::A);
                // ! This instruction is slightly different to the prefixed `RR A` ! (Flag Z is unset vs dependent)
                self.cpu.set_flag(Flag::Z, false);
            }

            // * 0x2_
            Instruction::JR_NZ_e8 => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.JR_e8();
                }
            }
            Instruction::LD_HL_n16 => self.LD_r16_n16(RegisterPair::HL),
            Instruction::LDI_HL_A => {
                self.LD_r16_r8(RegisterPair::HL, Register::A);
                self.INC_r16(RegisterPair::HL);
            }
            Instruction::INC_HL => self.INC_r16(RegisterPair::HL),
            Instruction::INC_H => self.INC_r8(Register::H),
            Instruction::DEC_H => self.DEC_r8(Register::H),
            Instruction::LD_H_n8 => self.LD_r8_n8(Register::H),
            Instruction::DAA => todo!(),
            Instruction::JR_Z_e8 => {
                if self.cpu.get_flag(Flag::Z) {
                    self.JR_e8();
                }
            }
            Instruction::ADD_HL_HL => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::HL),
            Instruction::LDI_A_HL => {
                self.LD_r8_r16(Register::A, RegisterPair::HL);
                self.INC_r16(RegisterPair::HL);
            }
            Instruction::DEC_HL => self.DEC_r16(RegisterPair::HL),
            Instruction::INC_L => self.INC_r8(Register::L),
            Instruction::DEC_L => self.DEC_r8(Register::L),
            Instruction::LD_L_n8 => self.LD_r8_n8(Register::L),
            Instruction::CPL => {
                let v = self.cpu.get_register(Register::A);
                self.cpu.set_register(Register::A, !v);
                self.cpu.set_flag(Flag::N | Flag::H, true);
            }

            // * 0x3_
            Instruction::JR_NC_e8 => {
                if !self.cpu.get_flag(Flag::C) {
                    self.JR_e8();
                }
            }
            Instruction::LD_SP_n16 => self.LD_r16_n16(RegisterPair::SP),
            Instruction::LDD_HL_A => {
                self.LD_r16_r8(RegisterPair::HL, Register::A);
                self.DEC_r16(RegisterPair::HL);
            }
            Instruction::INC_SP => self.INC_r16(RegisterPair::SP),
            Instruction::INCP_HL => {
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                let value = self.bus.read(address) + 1;
                self.bus.write(address, value);
            }
            Instruction::DECP_HL => {
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                let value = self.bus.read(address) - 1;
                self.bus.write(address, value);
            }
            Instruction::LD_HL_n8 => {
                let value = self.read_u8(RegisterPair::PC);
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                self.bus.write(address, value);
            }
            Instruction::SCF => self.cpu.set_flag(Flag::C, true),
            Instruction::JR_C_e8 => {
                if self.cpu.get_flag(Flag::C) {
                    self.JR_e8();
                }
            }
            Instruction::ADD_HL_SP => self.ADD_r16_r16(RegisterPair::HL, RegisterPair::SP),
            Instruction::LDD_A_HL => {
                self.LD_r8_r16(Register::A, RegisterPair::HL);
                self.DEC_r16(RegisterPair::HL);
            }
            Instruction::DEC_SP => self.DEC_r16(RegisterPair::SP),
            Instruction::INC_A => self.INC_r8(Register::A),
            Instruction::DEC_A => self.DEC_r8(Register::A),
            Instruction::LD_A_n8 => self.LD_r8_n8(Register::A),
            Instruction::CCF => {
                self.cpu.toggle_flag(Flag::C);
                self.cpu.set_flag(Flag::N | Flag::H, false);
            }

            // * 0x4_
            Instruction::LD_B_B => self.LD_r8_r8(Register::B, Register::B),
            Instruction::LD_B_C => self.LD_r8_r8(Register::B, Register::C),
            Instruction::LD_B_D => self.LD_r8_r8(Register::B, Register::D),
            Instruction::LD_B_E => self.LD_r8_r8(Register::B, Register::E),
            Instruction::LD_B_H => self.LD_r8_r8(Register::B, Register::H),
            Instruction::LD_B_L => self.LD_r8_r8(Register::B, Register::L),
            Instruction::LD_B_HL => self.LD_r8_r16(Register::B, RegisterPair::HL),
            Instruction::LD_B_A => self.LD_r8_r8(Register::B, Register::A),
            Instruction::LD_C_B => self.LD_r8_r8(Register::C, Register::B),
            Instruction::LD_C_C => self.LD_r8_r8(Register::C, Register::C),
            Instruction::LD_C_D => self.LD_r8_r8(Register::C, Register::D),
            Instruction::LD_C_E => self.LD_r8_r8(Register::C, Register::E),
            Instruction::LD_C_H => self.LD_r8_r8(Register::C, Register::H),
            Instruction::LD_C_L => self.LD_r8_r8(Register::C, Register::L),
            Instruction::LD_C_HL => self.LD_r8_r16(Register::C, RegisterPair::HL),
            Instruction::LD_C_A => self.LD_r8_r8(Register::C, Register::A),

            // * 0x5_
            Instruction::LD_D_B => self.LD_r8_r8(Register::D, Register::B),
            Instruction::LD_D_C => self.LD_r8_r8(Register::D, Register::C),
            Instruction::LD_D_D => self.LD_r8_r8(Register::D, Register::D),
            Instruction::LD_D_E => self.LD_r8_r8(Register::D, Register::E),
            Instruction::LD_D_H => self.LD_r8_r8(Register::D, Register::H),
            Instruction::LD_D_L => self.LD_r8_r8(Register::D, Register::L),
            Instruction::LD_D_HL => self.LD_r8_r16(Register::D, RegisterPair::HL),
            Instruction::LD_D_A => self.LD_r8_r8(Register::D, Register::A),
            Instruction::LD_E_B => self.LD_r8_r8(Register::E, Register::B),
            Instruction::LD_E_C => self.LD_r8_r8(Register::E, Register::C),
            Instruction::LD_E_D => self.LD_r8_r8(Register::E, Register::D),
            Instruction::LD_E_E => self.LD_r8_r8(Register::E, Register::E),
            Instruction::LD_E_H => self.LD_r8_r8(Register::E, Register::H),
            Instruction::LD_E_L => self.LD_r8_r8(Register::E, Register::L),
            Instruction::LD_E_HL => self.LD_r8_r16(Register::E, RegisterPair::HL),
            Instruction::LD_E_A => self.LD_r8_r8(Register::E, Register::A),

            // * 0x6_
            Instruction::LD_H_B => self.LD_r8_r8(Register::H, Register::B),
            Instruction::LD_H_C => self.LD_r8_r8(Register::H, Register::C),
            Instruction::LD_H_D => self.LD_r8_r8(Register::H, Register::D),
            Instruction::LD_H_E => self.LD_r8_r8(Register::H, Register::E),
            Instruction::LD_H_H => self.LD_r8_r8(Register::H, Register::H),
            Instruction::LD_H_L => self.LD_r8_r8(Register::H, Register::L),
            Instruction::LD_H_HL => self.LD_r8_r16(Register::H, RegisterPair::HL),
            Instruction::LD_H_A => self.LD_r8_r8(Register::H, Register::A),
            Instruction::LD_L_B => self.LD_r8_r8(Register::L, Register::B),
            Instruction::LD_L_C => self.LD_r8_r8(Register::L, Register::C),
            Instruction::LD_L_D => self.LD_r8_r8(Register::L, Register::D),
            Instruction::LD_L_E => self.LD_r8_r8(Register::L, Register::E),
            Instruction::LD_L_H => self.LD_r8_r8(Register::L, Register::H),
            Instruction::LD_L_L => self.LD_r8_r8(Register::L, Register::L),
            Instruction::LD_L_HL => self.LD_r8_r16(Register::L, RegisterPair::HL),
            Instruction::LD_L_A => self.LD_r8_r8(Register::L, Register::A),

            // * 0x7_
            Instruction::LD_HL_B => self.LD_r16_r8(RegisterPair::HL, Register::B),
            Instruction::LD_HL_C => self.LD_r16_r8(RegisterPair::HL, Register::C),
            Instruction::LD_HL_D => self.LD_r16_r8(RegisterPair::HL, Register::D),
            Instruction::LD_HL_E => self.LD_r16_r8(RegisterPair::HL, Register::E),
            Instruction::LD_HL_H => self.LD_r16_r8(RegisterPair::HL, Register::H),
            Instruction::LD_HL_L => self.LD_r16_r8(RegisterPair::HL, Register::L),
            Instruction::HALT => todo!(),
            Instruction::LD_HL_A => self.LD_r16_r8(RegisterPair::HL, Register::A),
            Instruction::LD_A_B => self.LD_r8_r8(Register::A, Register::B),
            Instruction::LD_A_C => self.LD_r8_r8(Register::A, Register::C),
            Instruction::LD_A_D => self.LD_r8_r8(Register::A, Register::D),
            Instruction::LD_A_E => self.LD_r8_r8(Register::A, Register::E),
            Instruction::LD_A_H => self.LD_r8_r8(Register::A, Register::H),
            Instruction::LD_A_L => self.LD_r8_r8(Register::A, Register::L),
            Instruction::LD_A_HL => self.LD_r8_r16(Register::A, RegisterPair::HL),
            Instruction::LD_A_A => self.LD_r8_r8(Register::A, Register::A),

            // * 0x8_
            Instruction::ADD_A_B => self.ADD_r8_r8(Register::A, Register::B),
            Instruction::ADD_A_C => self.ADD_r8_r8(Register::A, Register::C),
            Instruction::ADD_A_D => self.ADD_r8_r8(Register::A, Register::D),
            Instruction::ADD_A_E => self.ADD_r8_r8(Register::A, Register::E),
            Instruction::ADD_A_H => self.ADD_r8_r8(Register::A, Register::H),
            Instruction::ADD_A_L => self.ADD_r8_r8(Register::A, Register::L),
            Instruction::ADD_A_HL => self.ADD_r8_r16(Register::A, RegisterPair::HL),
            Instruction::ADD_A_A => self.ADD_r8_r8(Register::A, Register::A),
            Instruction::ADC_A_B => self.ADC_r8_r8(Register::A, Register::B),
            Instruction::ADC_A_C => self.ADC_r8_r8(Register::A, Register::C),
            Instruction::ADC_A_D => self.ADC_r8_r8(Register::A, Register::D),
            Instruction::ADC_A_E => self.ADC_r8_r8(Register::A, Register::E),
            Instruction::ADC_A_H => self.ADC_r8_r8(Register::A, Register::H),
            Instruction::ADC_A_L => self.ADC_r8_r8(Register::A, Register::L),
            Instruction::ADC_A_HL => self.ADC_r8_r16(Register::A, RegisterPair::HL),
            Instruction::ADC_A_A => self.ADC_r8_r8(Register::A, Register::A),

            // * 0x9_
            Instruction::SUB_A_B => self.SUB_r8_r8(Register::A, Register::B),
            Instruction::SUB_A_C => self.SUB_r8_r8(Register::A, Register::C),
            Instruction::SUB_A_D => self.SUB_r8_r8(Register::A, Register::D),
            Instruction::SUB_A_E => self.SUB_r8_r8(Register::A, Register::E),
            Instruction::SUB_A_H => self.SUB_r8_r8(Register::A, Register::H),
            Instruction::SUB_A_L => self.SUB_r8_r8(Register::A, Register::L),
            Instruction::SUB_A_HL => self.SUB_r8_r16(Register::A, RegisterPair::HL),
            Instruction::SUB_A_A => self.SUB_r8_r8(Register::A, Register::A),
            Instruction::SBC_A_B => self.SBC_r8_r8(Register::A, Register::B),
            Instruction::SBC_A_C => self.SBC_r8_r8(Register::A, Register::C),
            Instruction::SBC_A_D => self.SBC_r8_r8(Register::A, Register::D),
            Instruction::SBC_A_E => self.SBC_r8_r8(Register::A, Register::E),
            Instruction::SBC_A_H => self.SBC_r8_r8(Register::A, Register::H),
            Instruction::SBC_A_L => self.SBC_r8_r8(Register::A, Register::L),
            Instruction::SBC_A_HL => self.SBC_r8_r16(Register::A, RegisterPair::HL),
            Instruction::SBC_A_A => self.SBC_r8_r8(Register::A, Register::A),

            // * 0xA_
            Instruction::AND_A_B => self.AND_r8_r8(Register::A, Register::B),
            Instruction::AND_A_C => self.AND_r8_r8(Register::A, Register::C),
            Instruction::AND_A_D => self.AND_r8_r8(Register::A, Register::D),
            Instruction::AND_A_E => self.AND_r8_r8(Register::A, Register::E),
            Instruction::AND_A_H => self.AND_r8_r8(Register::A, Register::H),
            Instruction::AND_A_L => self.AND_r8_r8(Register::A, Register::L),
            Instruction::AND_A_HL => self.AND_r8_r16(Register::A, RegisterPair::HL),
            Instruction::AND_A_A => self.AND_r8_r8(Register::A, Register::A),
            Instruction::XOR_A_B => self.XOR_r8_r8(Register::A, Register::B),
            Instruction::XOR_A_C => self.XOR_r8_r8(Register::A, Register::C),
            Instruction::XOR_A_D => self.XOR_r8_r8(Register::A, Register::D),
            Instruction::XOR_A_E => self.XOR_r8_r8(Register::A, Register::E),
            Instruction::XOR_A_H => self.XOR_r8_r8(Register::A, Register::H),
            Instruction::XOR_A_L => self.XOR_r8_r8(Register::A, Register::L),
            Instruction::XOR_A_HL => self.XOR_r8_r16(Register::A, RegisterPair::HL),
            Instruction::XOR_A_A => self.XOR_r8_r8(Register::A, Register::A),

            // * 0xB_
            Instruction::OR_A_B => self.OR_r8_r8(Register::A, Register::B),
            Instruction::OR_A_C => self.OR_r8_r8(Register::A, Register::C),
            Instruction::OR_A_D => self.OR_r8_r8(Register::A, Register::D),
            Instruction::OR_A_E => self.OR_r8_r8(Register::A, Register::E),
            Instruction::OR_A_H => self.OR_r8_r8(Register::A, Register::H),
            Instruction::OR_A_L => self.OR_r8_r8(Register::A, Register::L),
            Instruction::OR_A_HL => self.OR_r8_r16(Register::A, RegisterPair::HL),
            Instruction::OR_A_A => self.OR_r8_r8(Register::A, Register::A),
            Instruction::CP_A_B => self.CP_r8_r8(Register::A, Register::B),
            Instruction::CP_A_C => self.CP_r8_r8(Register::A, Register::C),
            Instruction::CP_A_D => self.CP_r8_r8(Register::A, Register::D),
            Instruction::CP_A_E => self.CP_r8_r8(Register::A, Register::E),
            Instruction::CP_A_H => self.CP_r8_r8(Register::A, Register::H),
            Instruction::CP_A_L => self.CP_r8_r8(Register::A, Register::L),
            Instruction::CP_A_HL => self.CP_r8_r16(Register::A, RegisterPair::HL),
            Instruction::CP_A_A => self.CP_r8_r8(Register::A, Register::A),

            // * 0xC_
            Instruction::RET_NZ => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.RET();
                }
            }
            Instruction::POP_BC => self.POP_r16(RegisterPair::BC),
            Instruction::JP_NZ_a16 => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.JP_a16();
                }
            }
            Instruction::JP_a16 => self.JP_a16(),
            Instruction::CALL_NZ_a16 => {
                if !self.cpu.get_flag(Flag::Z) {
                    self.CALL_a16();
                }
            }
            Instruction::PUSH_BC => self.PUSH_r16(RegisterPair::BC),
            Instruction::ADD_A_n8 => self.ADD_r8_n8(Register::A),
            Instruction::RST_0x00 => self.RST_a16(0x0000),
            Instruction::RET_Z => {
                if self.cpu.get_flag(Flag::Z) {
                    self.RET();
                }
            }
            Instruction::RET => self.RET(),
            Instruction::JP_Z_a16 => {
                if self.cpu.get_flag(Flag::Z) {
                    self.JP_a16();
                }
            }
            Instruction::PREFIX => todo!(),
            Instruction::CALL_Z_a16 => {
                if self.cpu.get_flag(Flag::Z) {
                    self.CALL_a16();
                }
            }
            Instruction::CALL_a16 => self.CALL_a16(),
            Instruction::ADC_A_n8 => self.ADC_r8_n8(Register::A),
            Instruction::RST_0x08 => self.RST_a16(0x0008),

            // * 0xD_
            Instruction::RET_NC => {
                if !self.cpu.get_flag(Flag::C) {
                    self.RET();
                }
            }
            Instruction::POP_DE => self.POP_r16(RegisterPair::DE),
            Instruction::JP_NC_a16 => {
                if !self.cpu.get_flag(Flag::C) {
                    self.JP_a16();
                }
            }
            Instruction::CALL_NC_a16 => {
                if !self.cpu.get_flag(Flag::C) {
                    self.CALL_a16();
                }
            }
            Instruction::PUSH_DE => self.PUSH_r16(RegisterPair::DE),
            Instruction::SUB_A_n8 => self.SUB_r8_n8(Register::A),
            Instruction::RST_0x10 => self.RST_a16(0x0010),
            Instruction::RET_C => {
                if self.cpu.get_flag(Flag::C) {
                    self.RET();
                }
            }
            Instruction::RETI => {
                self.RET();
                self.ime = true;
            }
            Instruction::JP_C_a16 => {
                if self.cpu.get_flag(Flag::C) {
                    self.JP_a16();
                }
            }
            Instruction::CALL_C_a16 => {
                if self.cpu.get_flag(Flag::C) {
                    self.CALL_a16();
                }
            }
            Instruction::SBC_A_n8 => self.SBC_r8_n8(Register::A),
            Instruction::RST_0x18 => self.RST_a16(0x0018),

            // * 0xE_
            Instruction::LDH_a8_A => {
                let address = join_u16(self.read_u8(RegisterPair::PC), 0xFF);
                let value = self.cpu.get_register(Register::A);
                self.bus.write(address, value);
            }
            Instruction::POP_HL => self.POP_r16(RegisterPair::HL),
            Instruction::LDH_C_A => {
                let address = join_u16(self.cpu.get_register(Register::C), 0xFF);
                let value = self.cpu.get_register(Register::A);
                self.bus.write(address, value);
            }
            Instruction::PUSH_HL => self.PUSH_r16(RegisterPair::HL),
            Instruction::AND_A_n8 => self.ADD_r8_n8(Register::A),
            Instruction::RST_0x20 => self.RST_a16(0x0020),
            Instruction::ADD_SP_e8 => {
                // ? Same as JR_e8 but for `SP` instead of `PC`.
                let v = self.read_u8(RegisterPair::SP) as i8;
                let r = self.cpu.get_register_pair(RegisterPair::SP);
                self.cpu
                    .set_register_pair(RegisterPair::SP, r.wrapping_add_signed(v.into()));
            }
            Instruction::JP_HL => {
                let v = self.cpu.get_register_pair(RegisterPair::HL);
                self.cpu.set_register_pair(RegisterPair::PC, v);
            }
            Instruction::LD_a16_A => {
                let address = self.read_u16(RegisterPair::PC);
                let value = self.cpu.get_register(Register::A);
                self.bus.write(address, value);
            }
            Instruction::XOR_A_n8 => {
                let val = self.read_u8(RegisterPair::PC);
                let v = self.cpu.xor_register(Register::A, val);
                self.cpu.set_register(Register::A, v);

                self.cpu.set_flag(Flag::Z, v == 0);
                self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
            }
            Instruction::RST_0x28 => self.RST_a16(0x0028),

            // * 0xF_
            Instruction::LDH_A_a8 => {
                let address = join_u16(self.read_u8(RegisterPair::PC), 0xFF);
                let v = self.bus.read(address);
                self.cpu.set_register(Register::A, v);
            }
            Instruction::POP_AF => self.POP_r16(RegisterPair::AF),
            Instruction::LDH_A_C => {
                let address = join_u16(self.cpu.get_register(Register::C), 0xFF);
                let v = self.bus.read(address);
                self.cpu.set_register(Register::A, v);
            }
            Instruction::DI => self.ime = false,
            Instruction::PUSH_AF => self.PUSH_r16(RegisterPair::AF),
            Instruction::OR_A_n8 => {
                let val = self.read_u8(RegisterPair::PC);
                let v = self.cpu.or_register(Register::A, val);
                self.cpu.set_register(Register::A, v);

                self.cpu.set_flag(Flag::Z, v == 0);
                self.cpu.set_flag(Flag::N | Flag::H | Flag::C, false);
            }
            Instruction::RST_0x30 => self.RST_a16(0x0030),
            Instruction::LD_HL_SP_e8 => {
                let v = self.read_u8(RegisterPair::PC) as i8;
                let r = self.cpu.get_register_pair(RegisterPair::SP);
                self.cpu
                    .set_register_pair(RegisterPair::HL, r.wrapping_add_signed(v.into()));
            }
            Instruction::LD_SP_HL => {
                let v = self.cpu.get_register_pair(RegisterPair::HL);
                self.cpu.set_register_pair(RegisterPair::SP, v);
            }
            Instruction::LD_A_a16 => {
                let address = self.read_u16(RegisterPair::PC);
                let v = self.bus.read(address);
                self.cpu.set_register(Register::A, v);
            }
            Instruction::EI => todo!(),
            Instruction::CP_A_n8 => {
                let val = self.read_u8(RegisterPair::PC);
                let _ = self.cpu.sub_register(Register::A, val);
            }
            Instruction::RST_0x38 => self.RST_a16(0x0038),
        }
    }
}
