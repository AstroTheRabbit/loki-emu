use super::{bus::Bus, cpu::CPU, instructions::Instruction, utils::{RegisterPair, join_u16, Register, get_bit, Flag, split_u16}};

#[derive(Debug)]
pub struct GameBoyEmulator {
    pub cpu: CPU,
    pub bus: Bus,
}

impl GameBoyEmulator {
    pub fn step(&mut self) {

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
            Instruction::NOP => {

            },
            Instruction::LD_BC_n16 => {
                let v = self.read_pc_u16();
                self.cpu.set_register_pair(RegisterPair::BC, v);
            },
            Instruction::LD_BC_A => {
                let value = self.cpu.get_register(Register::A);
                let address = self.cpu.get_register_pair(RegisterPair::BC);
                self.bus.write(address, value);
            },
            Instruction::INC_BC => {
                self.cpu.increment_register_pair(RegisterPair::BC);
            },
            Instruction::INC_B => {
                self.cpu.increment_register(Register::B);
            },
            Instruction::DEC_B => {
                self.cpu.decrement_register(Register::B);
            },
            Instruction::LD_B_n8 => {
                let v = self.read_pc_u8();
                self.cpu.set_register(Register::B, v);
            },
            Instruction::RLCA => {
                let v = self.cpu.get_register(Register::A);
                let carry = get_bit(&v, 0b1000_0000);
                self.cpu.set_flag(Flag::C, carry);
                self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                self.cpu.set_register(Register::A, (v << 1) | carry as u8);
            },
            Instruction::LD_a16_SP => {
                let address = self.read_pc_u16();
                let (lsb, msb) = split_u16(self.cpu.get_register_pair(RegisterPair::SP));
                self.bus.write(address, lsb);
                self.bus.write(address + 1, msb);
            },
            Instruction::ADD_HL_BC => {
                let v = self.cpu.add_register_pairs(RegisterPair::HL, RegisterPair::BC);
                self.cpu.set_register_pair(RegisterPair::HL, v);
            },
            Instruction::LD_A_BC => {
                let address = self.cpu.get_register_pair(RegisterPair::BC);
                let v = self.bus.read(address);
                self.cpu.set_register(Register::A, v);
            },
            Instruction::DEC_BC => {
                self.cpu.decrement_register_pair(RegisterPair::BC);
            },
            Instruction::INC_C => {
                self.cpu.increment_register(Register::C);
            },
            Instruction::DEC_C => {
                self.cpu.decrement_register(Register::C);
            },
            Instruction::LD_C_n8 => {
                let v = self.read_pc_u8();
                self.cpu.set_register(Register::C, v);
            },
            Instruction::RRCA => {
                let v = self.cpu.get_register(Register::A);
                let carry = get_bit(&v, 0b0000_0001);
                self.cpu.set_flag(Flag::C, carry);
                self.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                self.cpu.set_register(Register::A, (v >> 1) | carry as u8);
            },

            // * 0x1_
            Instruction::STOP => {
                todo!();
            },
            Instruction::LD_DE_n16 => {
                let v = self.read_pc_u16();
                self.cpu.set_register_pair(RegisterPair::DE, v);
            },
            Instruction::LD_DE_A => {
                let value = self.cpu.get_register(Register::A);
                let address = self.cpu.get_register_pair(RegisterPair::DE);
                self.bus.write(address, value);
            },
            Instruction::INC_DE => {
                self.cpu.increment_register_pair(RegisterPair::DE);
            },
            Instruction::INC_D => {
                self.cpu.increment_register(Register::D);
            },
            Instruction::DEC_D => {
                self.cpu.decrement_register(Register::D);

            },
            Instruction::LD_D_n8 => {
                let v = self.read_pc_u8();
                self.cpu.set_register(Register::D, v);
            },
            Instruction::RLA => {

            },
            Instruction::JR_e8 => {

            },
            Instruction::ADD_HL_DE => {

            },
            Instruction::LD_A_DE => {

            },
            Instruction::DEC_DE => {

            },
            Instruction::INC_E => {

            },
            Instruction::DEC_E => {

            },
            Instruction::LD_E_n8 => {

            },
            Instruction::RRA => {

            },
            Instruction::JR_NZ_e8 => {

            },
            Instruction::LD_HL_n16 => {

            },
            Instruction::LDI_HL_A => {

            },
            Instruction::INC_HL => {

            },
            Instruction::INC_H => {

            },
            Instruction::DEC_H => {

            },
            Instruction::LD_H_n8 => {

            },
            Instruction::DAA => {

            },
            Instruction::JR_Z_e8 => {

            },
            Instruction::ADD_HL_HL => {

            },
            Instruction::LDI_A_HL => {

            },
            Instruction::DEC_HL => {

            },
            Instruction::INC_L => {

            },
            Instruction::DEC_L => {

            },
            Instruction::LD_L_n8 => {

            },
            Instruction::CPL => {

            },
            Instruction::JR_NC_e8 => {

            },
            Instruction::LD_SP_n16 => {

            },
            Instruction::LDD_HL_A => {

            },
            Instruction::INC_SP => {

            },
            Instruction::INCP_HL => {

            },
            Instruction::DECP_HL => {

            },
            Instruction::LD_HL_n8 => {

            },
            Instruction::SCF => {

            },
            Instruction::JR_C_e8 => {

            },
            Instruction::ADD_HL_SP => {

            },
            Instruction::LDD_A_HL => {

            },
            Instruction::DEC_SP => {

            },
            Instruction::INC_A => {

            },
            Instruction::DEC_A => {

            },
            Instruction::LD_A_n8 => {

            },
            Instruction::CCF => {

            },
            Instruction::LD_B_B => {

            },
            Instruction::LD_B_C => {

            },
            Instruction::LD_B_D => {

            },
            Instruction::LD_B_E => {

            },
            Instruction::LD_B_H => {

            },
            Instruction::LD_B_L => {

            },
            Instruction::LD_B_HL => {

            },
            Instruction::LD_B_A => {

            },
            Instruction::LD_C_B => {

            },
            Instruction::LD_C_C => {

            },
            Instruction::LD_C_D => {

            },
            Instruction::LD_C_E => {

            },
            Instruction::LD_C_H => {

            },
            Instruction::LD_C_L => {

            },
            Instruction::LD_C_HL => {

            },
            Instruction::LD_C_A => {

            },
            Instruction::LD_D_B => {

            },
            Instruction::LD_D_C => {

            },
            Instruction::LD_D_D => {

            },
            Instruction::LD_D_E => {

            },
            Instruction::LD_D_H => {

            },
            Instruction::LD_D_L => {

            },
            Instruction::LD_D_HL => {

            },
            Instruction::LD_D_A => {

            },
            Instruction::LD_E_B => {

            },
            Instruction::LD_E_C => {

            },
            Instruction::LD_E_D => {

            },
            Instruction::LD_E_E => {

            },
            Instruction::LD_E_H => {

            },
            Instruction::LD_E_L => {

            },
            Instruction::LD_E_HL => {

            },
            Instruction::LD_E_A => {

            },
            Instruction::LD_H_B => {

            },
            Instruction::LD_H_C => {

            },
            Instruction::LD_H_D => {

            },
            Instruction::LD_H_E => {

            },
            Instruction::LD_H_H => {

            },
            Instruction::LD_H_L => {

            },
            Instruction::LD_H_HL => {

            },
            Instruction::LD_H_A => {

            },
            Instruction::LD_L_B => {

            },
            Instruction::LD_L_C => {

            },
            Instruction::LD_L_D => {

            },
            Instruction::LD_L_E => {

            },
            Instruction::LD_L_H => {

            },
            Instruction::LD_L_L => {

            },
            Instruction::LD_L_HL => {

            },
            Instruction::LD_L_A => {

            },
            Instruction::LD_HL_B => {

            },
            Instruction::LD_HL_C => {

            },
            Instruction::LD_HL_D => {

            },
            Instruction::LD_HL_E => {

            },
            Instruction::LD_HL_H => {

            },
            Instruction::LD_HL_L => {

            },
            Instruction::HALT => {

            },
            Instruction::LD_HL_A => {

            },
            Instruction::LD_A_B => {

            },
            Instruction::LD_A_C => {

            },
            Instruction::LD_A_D => {

            },
            Instruction::LD_A_E => {

            },
            Instruction::LD_A_H => {

            },
            Instruction::LD_A_L => {

            },
            Instruction::LD_A_HL => {

            },
            Instruction::LD_A_A => {

            },
            Instruction::ADD_A_B => {

            },
            Instruction::ADD_A_C => {

            },
            Instruction::ADD_A_D => {

            },
            Instruction::ADD_A_E => {

            },
            Instruction::ADD_A_H => {

            },
            Instruction::ADD_A_L => {

            },
            Instruction::ADD_A_HL => {

            },
            Instruction::ADD_A_A => {

            },
            Instruction::ADC_A_B => {

            },
            Instruction::ADC_A_C => {

            },
            Instruction::ADC_A_D => {

            },
            Instruction::ADC_A_E => {

            },
            Instruction::ADC_A_H => {

            },
            Instruction::ADC_A_L => {

            },
            Instruction::ADC_A_HL => {

            },
            Instruction::ADC_A_A => {

            },
            Instruction::SUB_A_B => {

            },
            Instruction::SUB_A_C => {

            },
            Instruction::SUB_A_D => {

            },
            Instruction::SUB_A_E => {

            },
            Instruction::SUB_A_H => {

            },
            Instruction::SUB_A_L => {

            },
            Instruction::SUB_A_HL => {

            },
            Instruction::SUB_A_A => {

            },
            Instruction::SBC_A_B => {

            },
            Instruction::SBC_A_C => {

            },
            Instruction::SBC_A_D => {

            },
            Instruction::SBC_A_E => {

            },
            Instruction::SBC_A_H => {

            },
            Instruction::SBC_A_L => {

            },
            Instruction::SBC_A_HL => {

            },
            Instruction::SBC_A_A => {

            },
            Instruction::AND_A_B => {

            },
            Instruction::AND_A_C => {

            },
            Instruction::AND_A_D => {

            },
            Instruction::AND_A_E => {

            },
            Instruction::AND_A_H => {

            },
            Instruction::AND_A_L => {

            },
            Instruction::AND_A_HL => {

            },
            Instruction::AND_A_A => {

            },
            Instruction::XOR_A_B => {

            },
            Instruction::XOR_A_C => {

            },
            Instruction::XOR_A_D => {

            },
            Instruction::XOR_A_E => {

            },
            Instruction::XOR_A_H => {

            },
            Instruction::XOR_A_L => {

            },
            Instruction::XOR_A_HL => {

            },
            Instruction::XOR_A_A => {

            },
            Instruction::OR_A_B => {

            },
            Instruction::OR_A_C => {

            },
            Instruction::OR_A_D => {

            },
            Instruction::OR_A_E => {

            },
            Instruction::OR_A_H => {

            },
            Instruction::OR_A_L => {

            },
            Instruction::OR_A_HL => {

            },
            Instruction::OR_A_A => {

            },
            Instruction::CP_A_B => {

            },
            Instruction::CP_A_C => {

            },
            Instruction::CP_A_D => {

            },
            Instruction::CP_A_E => {

            },
            Instruction::CP_A_H => {

            },
            Instruction::CP_A_L => {

            },
            Instruction::CP_A_HL => {

            },
            Instruction::CP_A_A => {

            },
            Instruction::RET_NZ => {

            },
            Instruction::POP_BC => {

            },
            Instruction::JP_NZ_a16 => {

            },
            Instruction::JP_a16 => {

            },
            Instruction::CALL_NZ_a16 => {

            },
            Instruction::PUSH_BC => {

            },
            Instruction::ADD_A_n8 => {

            },
            Instruction::RST_0x00 => {

            },
            Instruction::RET_Z => {

            },
            Instruction::RET => {

            },
            Instruction::JP_Z_a16 => {

            },
            Instruction::PREFIX => {

            },
            Instruction::CALL_Z_a16 => {

            },
            Instruction::CALL_a16 => {

            },
            Instruction::ADC_A_n8 => {

            },
            Instruction::RST_0x08 => {

            },
            Instruction::RET_NC => {

            },
            Instruction::POP_DE => {

            },
            Instruction::JP_NC_a16 => {

            },
            Instruction::CALL_NC_a16 => {

            },
            Instruction::PUSH_DE => {

            },
            Instruction::SUB_A_n8 => {

            },
            Instruction::RST_0x10 => {

            },
            Instruction::RET_C => {

            },
            Instruction::RETI => {

            },
            Instruction::JP_C_a16 => {

            },
            Instruction::CALL_C_a16 => {

            },
            Instruction::SBC_A_n8 => {

            },
            Instruction::RST_0x18 => {

            },
            Instruction::LDH_a8_A => {

            },
            Instruction::POP_HL => {

            },
            Instruction::LDH_C_A => {

            },
            Instruction::PUSH_HL => {

            },
            Instruction::AND_A_n8 => {

            },
            Instruction::RST_0x20 => {

            },
            Instruction::ADD_SP_e8 => {

            },
            Instruction::JP_HL => {

            },
            Instruction::LD_a16_A => {

            },
            Instruction::XOR_A_n8 => {

            },
            Instruction::RST_0x28 => {

            },
            Instruction::LDH_A_a8 => {

            },
            Instruction::POP_AF => {

            },
            Instruction::LDH_A_C => {

            },
            Instruction::DI => {

            },
            Instruction::PUSH_AF => {

            },
            Instruction::OR_A_n8 => {

            },
            Instruction::RST_0x30 => {

            },
            Instruction::LD_HL_SP_e8 => {

            },
            Instruction::LD_SP_HL => {

            },
            Instruction::LD_A_a16 => {

            },
            Instruction::EI => {

            },
            Instruction::CP_A_n8 => {

            },
            Instruction::RST_0x38 => {

            },
        }
    }
}
