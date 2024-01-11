use crate::*;

use super::{bus::Bus, cpu::CPU, instructions::Instruction, utils::*};

#[derive(Debug)]
pub struct GameBoyEmulator {
    pub cpu: CPU,
    pub bus: Bus,
}

impl GameBoyEmulator {
    pub fn step(&mut self) {
        todo!();
    }

    /// Read from the bus at the PC's address, then increment the PC.
    #[inline]
    pub fn read_pc_u8(&mut self) -> u8 {
        return self.bus.read(self.cpu.increment_pc());
    }

    /// Runs [`GameBoyEmulator::read_pc_u8`] twice, joining the results.
    #[inline]
    pub fn read_pc_u16(&mut self) -> u16 {
        return join_u16(self.read_pc_u8(), self.read_pc_u8());
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            // * 0X0_
            Instruction::NOP => {},
            Instruction::LD_BC_n16 => LD_r16_n16!(self, RegisterPair::BC),
            Instruction::LD_BC_A => LD_r16_r8!(self, RegisterPair::BC, Register::A),
            Instruction::INC_BC => INC_r16!(self, RegisterPair::BC),
            Instruction::INC_B => INC_r8!(self, Register::B),
            Instruction::DEC_B => DEC_r8!(self, Register::B),
            Instruction::LD_B_n8 => LD_r8_n8!(self, Register::B),
            Instruction::RLCA => {
                // carry becomes prev. bit 7, bit 0 becomes prev. bit 7
                let v = self.cpu.get_register(Register::A);
                let new_carry = get_bit(&v, 0b1000_0000);
                self.cpu.set_flag(Flag::C, new_carry);
                self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                self.cpu.set_register(Register::A, (v << 1) | new_carry as u8);
            },
            Instruction::LD_a16_SP => LD_a16_r16!(self, RegisterPair::SP),
            Instruction::ADD_HL_BC => ADD_r16_r16!(self, RegisterPair::HL, RegisterPair::BC),
            Instruction::LD_A_BC => LD_r8_r16!(self, Register::A, RegisterPair::BC),
            Instruction::DEC_BC => DEC_r16!(self, RegisterPair::BC),
            Instruction::INC_C => INC_r8!(self, Register::C),
            Instruction::DEC_C => DEC_r8!(self, Register::C),
            Instruction::LD_C_n8 => LD_r8_n8!(self, Register::C),
            Instruction::RRCA => {
                // carry becomes prev. bit 0, bit 7 becomes prev. bit 0
                let v = self.cpu.get_register(Register::A);
                let new_carry = get_bit(&v, 0b0000_0001);
                self.cpu.set_flag(Flag::C, new_carry);
                self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                self.cpu.set_register(Register::A, (v >> 1) | ((new_carry as u8) << 7));
            },

            // * 0x1_
            Instruction::STOP => todo!(),
            Instruction::LD_DE_n16 => LD_r16_n16!(self, RegisterPair::DE),
            Instruction::LD_DE_A => LD_r16_r8!(self, RegisterPair::DE, Register::A),
            Instruction::INC_DE => INC_r16!(self, RegisterPair::DE),
            Instruction::INC_D => INC_r8!(self, Register::D),
            Instruction::DEC_D => DEC_r8!(self, Register::D),
            Instruction::LD_D_n8 => LD_r8_n8!(self, Register::D),
            Instruction::RLA => {
                // carry becomes prev. bit 7, bit 0 becomes prev. carry
                let v = self.cpu.get_register(Register::A);
                let prev_carry = self.cpu.get_flag(Flag::C);
                let new_carry = get_bit(&v, 0b1000_0000);
                self.cpu.set_flag(Flag::C, new_carry);
                self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                self.cpu.set_register(Register::A, (v << 1) | prev_carry as u8);
            },
            Instruction::JR_e8 => JR_e8!(self),
            Instruction::ADD_HL_DE => ADD_r16_r16!(self, RegisterPair::HL, RegisterPair::DE),
            Instruction::LD_A_DE => LD_r8_r16!(self, Register::A, RegisterPair::DE),
            Instruction::DEC_DE => DEC_r16!(self, RegisterPair::DE),
            Instruction::INC_E => INC_r8!(self, Register::E),
            Instruction::DEC_E => DEC_r8!(self, Register::E),
            Instruction::LD_E_n8 => LD_r8_n8!(self, Register::E),
            Instruction::RRA => {
                // carry becomes prev. bit 0, bit 7 becomes prev. carry
                let v = self.cpu.get_register(Register::A);
                let prev_carry = self.cpu.get_flag(Flag::C);
                let new_carry = get_bit(&v, 0b0000_0001);
                self.cpu.set_flag(Flag::C, new_carry);
                self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                self.cpu.set_register(Register::A, (v << 1) | ((prev_carry as u8) << 7));
            },

            // * 0x2_
            Instruction::JR_NZ_e8 => if !self.cpu.get_flag(Flag::Z) { JR_e8!(self) },
            Instruction::LD_HL_n16 => LD_r16_n16!(self, RegisterPair::HL),
            Instruction::LDI_HL_A => {
                LD_r16_r8!(self, RegisterPair::HL, Register::A);
                INC_r16!(self, RegisterPair::HL);
            },
            Instruction::INC_HL => INC_r16!(self, RegisterPair::HL),
            Instruction::INC_H => INC_r8!(self, Register::H),
            Instruction::DEC_H => DEC_r8!(self, Register::H),
            Instruction::LD_H_n8 => LD_r8_n8!(self, Register::H),
            Instruction::DAA => todo!(),
            Instruction::JR_Z_e8 => if self.cpu.get_flag(Flag::Z) { JR_e8!(self) },
            Instruction::ADD_HL_HL => ADD_r16_r16!(self, RegisterPair::HL, RegisterPair::HL),
            Instruction::LDI_A_HL => {
                LD_r8_r16!(self, Register::A, RegisterPair::HL);
                INC_r16!(self, RegisterPair::HL);
            },
            Instruction::DEC_HL => DEC_r16!(self, RegisterPair::HL),
            Instruction::INC_L => INC_r8!(self, Register::L),
            Instruction::DEC_L => DEC_r8!(self, Register::L),
            Instruction::LD_L_n8 => LD_r8_n8!(self, Register::L),
            Instruction::CPL => {
                let v = self.cpu.get_register(Register::A);
                self.cpu.set_register(Register::A, !v);
                self.cpu.set_flag(Flag::N | Flag::H, true);
            },

            // * 0x3_
            Instruction::JR_NC_e8 => if !self.cpu.get_flag(Flag::C) { JR_e8!(self) },
            Instruction::LD_SP_n16 => LD_r16_n16!(self, RegisterPair::SP),
            Instruction::LDD_HL_A => {
                LD_r16_r8!(self, RegisterPair::HL, Register::A);
                DEC_r16!(self, RegisterPair::HL);
            },
            Instruction::INC_SP => INC_r16!(self, RegisterPair::SP),
            Instruction::INCP_HL => {
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                let value = self.bus.read(address) + 1;
                self.bus.write(address, value);
            },
            Instruction::DECP_HL => {
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                let value = self.bus.read(address) - 1;
                self.bus.write(address, value);
            },
            Instruction::LD_HL_n8 => {
                let value = self.read_pc_u8();
                let address = self.cpu.get_register_pair(RegisterPair::HL);
                self.bus.write(address, value);
            },
            Instruction::SCF => self.cpu.set_flag(Flag::C, true),
            Instruction::JR_C_e8 => if self.cpu.get_flag(Flag::C) { JR_e8!(self) },
            Instruction::ADD_HL_SP => ADD_r16_r16!(self, RegisterPair::HL, RegisterPair::SP),
            Instruction::LDD_A_HL => {
                LD_r8_r16!(self, Register::A, RegisterPair::HL);
                DEC_r16!(self, RegisterPair::HL);
            },
            Instruction::DEC_SP => DEC_r16!(self, RegisterPair::SP),
            Instruction::INC_A => INC_r8!(self, Register::A),
            Instruction::DEC_A => DEC_r8!(self, Register::A),
            Instruction::LD_A_n8 => LD_r8_n8!(self, Register::A),
            Instruction::CCF => {
                self.cpu.toggle_flag(Flag::C);
                self.cpu.set_flag(Flag::N | Flag::H, false);
            },

            // * 0x4_
            Instruction::LD_B_B => LD_r8_r8!(self, Register::B, Register::B),
            Instruction::LD_B_C => LD_r8_r8!(self, Register::B, Register::C),
            Instruction::LD_B_D => LD_r8_r8!(self, Register::B, Register::D),
            Instruction::LD_B_E => LD_r8_r8!(self, Register::B, Register::E),
            Instruction::LD_B_H => LD_r8_r8!(self, Register::B, Register::H),
            Instruction::LD_B_L => LD_r8_r8!(self, Register::B, Register::L),
            Instruction::LD_B_HL => LD_r8_r16!(self, Register::B, RegisterPair::HL),
            Instruction::LD_B_A => LD_r8_r8!(self, Register::B, Register::A),
            Instruction::LD_C_B => LD_r8_r8!(self, Register::C, Register::B),
            Instruction::LD_C_C => LD_r8_r8!(self, Register::C, Register::C),
            Instruction::LD_C_D => LD_r8_r8!(self, Register::C, Register::D),
            Instruction::LD_C_E => LD_r8_r8!(self, Register::C, Register::E),
            Instruction::LD_C_H => LD_r8_r8!(self, Register::C, Register::H),
            Instruction::LD_C_L => LD_r8_r8!(self, Register::C, Register::L),
            Instruction::LD_C_HL => LD_r8_r16!(self, Register::C, RegisterPair::HL),
            Instruction::LD_C_A => LD_r8_r8!(self, Register::C, Register::A),

            // * 0x5_
            Instruction::LD_D_B => LD_r8_r8!(self, Register::D, Register::B),
            Instruction::LD_D_C => LD_r8_r8!(self, Register::D, Register::C),
            Instruction::LD_D_D => LD_r8_r8!(self, Register::D, Register::D),
            Instruction::LD_D_E => LD_r8_r8!(self, Register::D, Register::E),
            Instruction::LD_D_H => LD_r8_r8!(self, Register::D, Register::H),
            Instruction::LD_D_L => LD_r8_r8!(self, Register::D, Register::L),
            Instruction::LD_D_HL => LD_r8_r16!(self, Register::D, RegisterPair::HL),
            Instruction::LD_D_A => LD_r8_r8!(self, Register::D, Register::A),
            Instruction::LD_E_B => LD_r8_r8!(self, Register::E, Register::B),
            Instruction::LD_E_C => LD_r8_r8!(self, Register::E, Register::C),
            Instruction::LD_E_D => LD_r8_r8!(self, Register::E, Register::D),
            Instruction::LD_E_E => LD_r8_r8!(self, Register::E, Register::E),
            Instruction::LD_E_H => LD_r8_r8!(self, Register::E, Register::H),
            Instruction::LD_E_L => LD_r8_r8!(self, Register::E, Register::L),
            Instruction::LD_E_HL => LD_r8_r16!(self, Register::E, RegisterPair::HL),
            Instruction::LD_E_A => LD_r8_r8!(self, Register::E, Register::A),

            // * 0x6_
            Instruction::LD_H_B => LD_r8_r8!(self, Register::H, Register::B),
            Instruction::LD_H_C => LD_r8_r8!(self, Register::H, Register::C),
            Instruction::LD_H_D => LD_r8_r8!(self, Register::H, Register::D),
            Instruction::LD_H_E => LD_r8_r8!(self, Register::H, Register::E),
            Instruction::LD_H_H => LD_r8_r8!(self, Register::H, Register::H),
            Instruction::LD_H_L => LD_r8_r8!(self, Register::H, Register::L),
            Instruction::LD_H_HL => LD_r8_r16!(self, Register::H, RegisterPair::HL),
            Instruction::LD_H_A => LD_r8_r8!(self, Register::H, Register::A),
            Instruction::LD_L_B => LD_r8_r8!(self, Register::L, Register::B),
            Instruction::LD_L_C => LD_r8_r8!(self, Register::L, Register::C),
            Instruction::LD_L_D => LD_r8_r8!(self, Register::L, Register::D),
            Instruction::LD_L_E => LD_r8_r8!(self, Register::L, Register::E),
            Instruction::LD_L_H => LD_r8_r8!(self, Register::L, Register::H),
            Instruction::LD_L_L => LD_r8_r8!(self, Register::L, Register::L),
            Instruction::LD_L_HL => LD_r8_r16!(self, Register::L, RegisterPair::HL),
            Instruction::LD_L_A => LD_r8_r8!(self, Register::L, Register::A),

            // * 0x7_
            Instruction::LD_HL_B => LD_r16_r8!(self, RegisterPair::HL, Register::B),
            Instruction::LD_HL_C => LD_r16_r8!(self, RegisterPair::HL, Register::C),
            Instruction::LD_HL_D => LD_r16_r8!(self, RegisterPair::HL, Register::D),
            Instruction::LD_HL_E => LD_r16_r8!(self, RegisterPair::HL, Register::E),
            Instruction::LD_HL_H => LD_r16_r8!(self, RegisterPair::HL, Register::H),
            Instruction::LD_HL_L => LD_r16_r8!(self, RegisterPair::HL, Register::L),
            Instruction::HALT => todo!(),
            Instruction::LD_HL_A => LD_r16_r8!(self, RegisterPair::HL, Register::A),
            Instruction::LD_A_B => LD_r8_r8!(self, Register::A, Register::B),
            Instruction::LD_A_C => LD_r8_r8!(self, Register::A, Register::C),
            Instruction::LD_A_D => LD_r8_r8!(self, Register::A, Register::D),
            Instruction::LD_A_E => LD_r8_r8!(self, Register::A, Register::E),
            Instruction::LD_A_H => LD_r8_r8!(self, Register::A, Register::H),
            Instruction::LD_A_L => LD_r8_r8!(self, Register::A, Register::L),
            Instruction::LD_A_HL => LD_r8_r16!(self, Register::A, RegisterPair::HL),
            Instruction::LD_A_A => LD_r8_r8!(self, Register::A, Register::A),

            // * 0x8_
            Instruction::ADD_A_B => ADD_r8_r8!(self, Register::A, Register::B),
            Instruction::ADD_A_C => ADD_r8_r8!(self, Register::A, Register::C),
            Instruction::ADD_A_D => ADD_r8_r8!(self, Register::A, Register::D),
            Instruction::ADD_A_E => ADD_r8_r8!(self, Register::A, Register::E),
            Instruction::ADD_A_H => ADD_r8_r8!(self, Register::A, Register::H),
            Instruction::ADD_A_L => ADD_r8_r8!(self, Register::A, Register::L),
            Instruction::ADD_A_HL => ADD_r8_r16!(self, Register::A, RegisterPair::HL),
            Instruction::ADD_A_A => ADD_r8_r8!(self, Register::A, Register::A),
            Instruction::ADC_A_B => ADC_r8_r8!(self, Register::A, Register::B),
            Instruction::ADC_A_C => ADC_r8_r8!(self, Register::A, Register::C),
            Instruction::ADC_A_D => ADC_r8_r8!(self, Register::A, Register::D),
            Instruction::ADC_A_E => ADC_r8_r8!(self, Register::A, Register::E),
            Instruction::ADC_A_H => ADC_r8_r8!(self, Register::A, Register::H),
            Instruction::ADC_A_L => ADC_r8_r8!(self, Register::A, Register::L),
            Instruction::ADC_A_HL => ADC_r8_r16!(self, Register::A, RegisterPair::HL),
            Instruction::ADC_A_A => ADC_r8_r8!(self, Register::A, Register::A),

            // * 0x9_
            Instruction::SUB_A_B => SUB_r8_r8!(self, Register::A, Register::B),
            Instruction::SUB_A_C => SUB_r8_r8!(self, Register::A, Register::C),
            Instruction::SUB_A_D => SUB_r8_r8!(self, Register::A, Register::D),
            Instruction::SUB_A_E => SUB_r8_r8!(self, Register::A, Register::E),
            Instruction::SUB_A_H => SUB_r8_r8!(self, Register::A, Register::H),
            Instruction::SUB_A_L => SUB_r8_r8!(self, Register::A, Register::L),
            Instruction::SUB_A_HL => SUB_r8_r16!(self, Register::A, RegisterPair::HL),
            Instruction::SUB_A_A => SUB_r8_r8!(self, Register::A, Register::A),
            Instruction::SBC_A_B => SBC_r8_r8!(self, Register::A, Register::B),
            Instruction::SBC_A_C => SBC_r8_r8!(self, Register::A, Register::C),
            Instruction::SBC_A_D => SBC_r8_r8!(self, Register::A, Register::D),
            Instruction::SBC_A_E => SBC_r8_r8!(self, Register::A, Register::E),
            Instruction::SBC_A_H => SBC_r8_r8!(self, Register::A, Register::H),
            Instruction::SBC_A_L => SBC_r8_r8!(self, Register::A, Register::L),
            Instruction::SBC_A_HL => SBC_r8_r16!(self, Register::A, RegisterPair::HL),
            Instruction::SBC_A_A => SBC_r8_r8!(self, Register::A, Register::A),

            // * 0xA_
            Instruction::AND_A_B => {},
            Instruction::AND_A_C => {},
            Instruction::AND_A_D => {},
            Instruction::AND_A_E => {},
            Instruction::AND_A_H => {},
            Instruction::AND_A_L => {},
            Instruction::AND_A_HL => {},
            Instruction::AND_A_A => {},
            Instruction::XOR_A_B => {},
            Instruction::XOR_A_C => {},
            Instruction::XOR_A_D => {},
            Instruction::XOR_A_E => {},
            Instruction::XOR_A_H => {},
            Instruction::XOR_A_L => {},
            Instruction::XOR_A_HL => {},
            Instruction::XOR_A_A => {},

            // * 0xB_
            Instruction::OR_A_B => {},
            Instruction::OR_A_C => {},
            Instruction::OR_A_D => {},
            Instruction::OR_A_E => {},
            Instruction::OR_A_H => {},
            Instruction::OR_A_L => {},
            Instruction::OR_A_HL => {},
            Instruction::OR_A_A => {},
            Instruction::CP_A_B => {},
            Instruction::CP_A_C => {},
            Instruction::CP_A_D => {},
            Instruction::CP_A_E => {},
            Instruction::CP_A_H => {},
            Instruction::CP_A_L => {},
            Instruction::CP_A_HL => {},
            Instruction::CP_A_A => {},

            // * 0xC_
            Instruction::RET_NZ => {},
            Instruction::POP_BC => {},
            Instruction::JP_NZ_a16 => {},
            Instruction::JP_a16 => {},
            Instruction::CALL_NZ_a16 => {},
            Instruction::PUSH_BC => {},
            Instruction::ADD_A_n8 => {},
            Instruction::RST_0x00 => {},
            Instruction::RET_Z => {},
            Instruction::RET => {},
            Instruction::JP_Z_a16 => {},
            Instruction::PREFIX => {},
            Instruction::CALL_Z_a16 => {},
            Instruction::CALL_a16 => {},
            Instruction::ADC_A_n8 => {},
            Instruction::RST_0x08 => {},

            // * 0xD_
            Instruction::RET_NC => {},
            Instruction::POP_DE => {},
            Instruction::JP_NC_a16 => {},
            Instruction::CALL_NC_a16 => {},
            Instruction::PUSH_DE => {},
            Instruction::SUB_A_n8 => {},
            Instruction::RST_0x10 => {},
            Instruction::RET_C => {},
            Instruction::RETI => {},
            Instruction::JP_C_a16 => {},
            Instruction::CALL_C_a16 => {},
            Instruction::SBC_A_n8 => {},
            Instruction::RST_0x18 => {},

            // * 0xE_
            Instruction::LDH_a8_A => {},
            Instruction::POP_HL => {},
            Instruction::LDH_C_A => {},
            Instruction::PUSH_HL => {},
            Instruction::AND_A_n8 => {},
            Instruction::RST_0x20 => {},
            Instruction::ADD_SP_e8 => {},
            Instruction::JP_HL => {},
            Instruction::LD_a16_A => {},
            Instruction::XOR_A_n8 => {},
            Instruction::RST_0x28 => {},

            // * 0xF_
            Instruction::LDH_A_a8 => {},
            Instruction::POP_AF => {},
            Instruction::LDH_A_C => {},
            Instruction::DI => {},
            Instruction::PUSH_AF => {},
            Instruction::OR_A_n8 => {},
            Instruction::RST_0x30 => {},
            Instruction::LD_HL_SP_e8 => {},
            Instruction::LD_SP_HL => {},
            Instruction::LD_A_a16 => {},
            Instruction::EI => {},
            Instruction::CP_A_n8 => {},
            Instruction::RST_0x38 => {},
        }
    }
}