use std::fmt;

use crate::gb::{emu::GameboyEmulator, utils::*, bus::Bus};

use super::{operations::*, prefixed_instructions::PREFIX_n8};

/// A 4 t-cycle long step of an instruction, either returning the next step or signalling the instruction's completion.
#[derive(Default)]
pub enum InstructionStep {
    Running(Box<dyn FnOnce(&mut GameboyEmulator) -> InstructionStep>),
    #[default]
    Complete,
}

impl fmt::Debug for InstructionStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running(_) => write!(f, "Running"),
            Self::Complete => write!(f, "Complete"),
        }
    }
}

impl InstructionStep {
    /// Returns `true` if this instruction has finished.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }

    #[inline]
    pub fn new<F>(func: F) -> Self
    where
        F: FnOnce(&mut GameboyEmulator) -> InstructionStep + 'static,
    {
        Self::Running(Box::new(func))
    }
}

#[derive(Debug, Default)]
pub struct Instruction {
    pub mnemomic: String,
    /// Code of the instruction split into steps each 4 t-cycles long.
    current_step: InstructionStep,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(&self.mnemomic)
    }
}

impl Instruction {
    pub fn new<F>(mnemomic: String, steps: F) -> Self
    where
        F: FnOnce(&mut GameboyEmulator) -> InstructionStep + 'static,
    {
        Self {
            mnemomic,
            current_step: InstructionStep::new(steps),
        }
    }

    /// Runs one step of the instruction.
    pub fn step(&mut self, emu: &mut GameboyEmulator) {
        let step = std::mem::take(&mut self.current_step);
        if let InstructionStep::Running(next_step) = step {
            self.current_step = next_step(emu);
        }
    }

    /// Returns true if the instruction has fully completed.
    pub fn has_completed(&self) -> bool {
        self.current_step.is_complete()
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            // * 0x0_
            0x00 => Instruction::new("NOP".to_string(), |_| InstructionStep::Complete),
            0x01 => LD_r16_n16(RegisterPair::BC),
            0x02 => LD_r16_r8(RegisterPair::BC, Register::A),
            0x03 => INC_r16(RegisterPair::BC),
            0x04 => INC_r8(Register::B),
            0x05 => DEC_r8(Register::B),
            0x06 => LD_r8_n8(Register::B),
            0x07 => Instruction::new("RLCA".to_string(), |emu| {
                // ? Slightly different to prefixed `RLC A` (zero flag is always unset, not dependent).
                let v = emu.cpu.get_register(Register::A);
                let new_carry = get_bit(v, 0b1000_0000);
                let v = (v << 1) | new_carry as u8;

                emu.cpu.set_register(Register::A, v);
                emu.cpu.set_flag(Flag::C, new_carry);
                emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                InstructionStep::Complete
            }),
            0x08 => LD_n16_r16(RegisterPair::SP),
            0x09 => ADD_r16_r16(RegisterPair::HL, RegisterPair::BC),
            0x0A => LD_r8_r16(Register::A, RegisterPair::BC),
            0x0B => DEC_r16(RegisterPair::BC),
            0x0C => INC_r8(Register::C),
            0x0D => DEC_r8(Register::C),
            0x0E => LD_r8_n8(Register::C),
            0x0F => Instruction::new("RRCA".to_string(), |emu| {
                // ? Slightly different to prefixed `RRC A` (zero flag is always unset, not dependent).
                let v = emu.cpu.get_register(Register::A);
                let new_carry = get_bit(v, 0b0000_0001);
                let v = (v >> 1) | ((new_carry as u8) << 7);

                emu.cpu.set_register(Register::A, v);
                emu.cpu.set_flag(Flag::C, new_carry);
                emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                InstructionStep::Complete
            }),
            // * 0x1_
            0x10 => Instruction::new("STOP".to_string(), |emu| {
                emu.is_halted = true;
                InstructionStep::Complete
            }),
            0x11 => LD_r16_n16(RegisterPair::DE),
            0x12 => LD_r16_r8(RegisterPair::DE, Register::A),
            0x13 => INC_r16(RegisterPair::DE),
            0x14 => INC_r8(Register::D),
            0x15 => DEC_r8(Register::D),
            0x16 => LD_r8_n8(Register::D),
            0x17 => Instruction::new("RLA".to_string(), |emu| {
                // ? Slightly different to prefixed `RL A` (zero flag is always unset, not dependent).
                let v = emu.cpu.get_register(Register::A);
                let prev_carry = emu.cpu.get_flag(Flag::C);
                let new_carry = get_bit(v, 0b1000_0000);
                let v = (v << 1) | prev_carry as u8;

                emu.cpu.set_register(Register::A, v);
                emu.cpu.set_flag(Flag::C, new_carry);
                emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                InstructionStep::Complete
            }),
            0x18 => LD_n16_r16(RegisterPair::SP),
            0x19 => ADD_r16_r16(RegisterPair::HL, RegisterPair::DE),
            0x1A => LD_r8_r16(Register::A, RegisterPair::DE),
            0x1B => DEC_r16(RegisterPair::DE),
            0x1C => INC_r8(Register::E),
            0x1D => DEC_r8(Register::E),
            0x1E => LD_r8_n8(Register::E),
            0x1F => Instruction::new("RRA".to_string(), |emu| {
                // ? Slightly different to prefixed `RRC A` (zero flag is always unset, not dependent).
                let v = emu.cpu.get_register(Register::A);
                let prev_carry = emu.cpu.get_flag(Flag::C);
                let new_carry = get_bit(v, 0b0000_0001);
                let v = (v >> 1) | ((prev_carry as u8) << 7);

                emu.cpu.set_register(Register::A, v);
                emu.cpu.set_flag(Flag::C, new_carry);
                emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                InstructionStep::Complete
            }),
            // * 0x2_
            0x20 => JR_nc_n8(Flag::Z),
            0x21 => LD_r16_n16(RegisterPair::HL),
            0x22 => Instruction::new("LD (HL+), A".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.cpu.get_register(Register::A);
                    emu.write_r16(RegisterPair::HL, value);
                    emu.cpu.inc_register_pair(RegisterPair::HL);
                    InstructionStep::Complete
                })
            }),
            0x23 => INC_r16(RegisterPair::HL),
            0x24 => INC_r8(Register::H),
            0x25 => DEC_r8(Register::H),
            0x26 => LD_r8_n8(Register::H),
            0x27 => todo!("GB - DAA instruction"),
            0x28 => JR_c_n8(Flag::Z),
            0x29 => ADD_r16_r16(RegisterPair::HL, RegisterPair::HL),
            0x2A => Instruction::new("LD A, (HL+)".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.read_r16(RegisterPair::HL);
                    emu.cpu.set_register(Register::A, value);
                    emu.cpu.inc_register_pair(RegisterPair::HL);
                    InstructionStep::Complete
                })
            }),
            0x2B => DEC_r16(RegisterPair::HL),
            0x2C => INC_r8(Register::L),
            0x2D => DEC_r8(Register::L),
            0x2E => LD_r8_n8(Register::L),
            0x2F => Instruction::new("CPL".to_string(), |emu| {
                let value = !emu.cpu.get_register(Register::A);
                emu.cpu.set_register(Register::A, value);
                emu.cpu.set_flag(Flag::N | Flag::H, true);
                InstructionStep::Complete
            }),

            // * 0x3_
            0x30 => JR_nc_n8(Flag::C),
            0x31 => LD_r16_n16(RegisterPair::SP),
            0x32 => Instruction::new("LD (HL-), A".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.cpu.get_register(Register::A);
                    emu.write_r16(RegisterPair::HL, value);
                    emu.cpu.dec_register_pair(RegisterPair::HL);
                    InstructionStep::Complete
                })
            }),
            0x33 => INC_r16(RegisterPair::SP),
            0x34 => Instruction::new("INC (HL)".to_string(), |_emu| {
                // This instruction is different to `INC HL`.
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.read_r16(RegisterPair::HL).wrapping_add(1);
                    InstructionStep::new(move |emu| {
                        emu.write_r16(RegisterPair::HL, value);
                        emu.cpu.set_flag(Flag::Z, value == 0);
                        emu.cpu.set_flag(Flag::N, false);
                        emu.cpu.set_flag(Flag::H, (value & 0xF) > 0xE);
                        InstructionStep::Complete
                    })
                })
            }),
            0x35 => Instruction::new("DEC (HL)".to_string(), |_emu| {
                // This instruction is different to `DEC HL`.
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.read_r16(RegisterPair::HL).wrapping_sub(1);
                    InstructionStep::new(move |emu| {
                        emu.write_r16(RegisterPair::HL, value);
                        emu.cpu.set_flag(Flag::Z, value == 0);
                        emu.cpu.set_flag(Flag::N, true);
                        emu.cpu.set_flag(Flag::H, (value & 0xF) > 0xE);
                        InstructionStep::Complete
                    })
                })
            }),
            0x36 => LD_r16_n8(RegisterPair::HL),
            0x37 => Instruction::new("SCF".to_string(), |emu| {
                emu.cpu.set_flag(Flag::N | Flag::H, false);
                emu.cpu.set_flag(Flag::C, true);
                InstructionStep::Complete
            }),
            0x38 => JR_c_n8(Flag::C),
            0x39 => ADD_r16_r16(RegisterPair::HL, RegisterPair::SP),
            0x3A => Instruction::new("LD A, (HL-)".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.read_r16(RegisterPair::HL);
                    emu.cpu.set_register(Register::A, value);
                    emu.cpu.dec_register_pair(RegisterPair::HL);
                    InstructionStep::Complete
                })
            }),
            0x3B => DEC_r16(RegisterPair::SP),
            0x3C => INC_r8(Register::A),
            0x3D => DEC_r8(Register::A),
            0x3E => LD_r8_n8(Register::A),
            0x3F => Instruction::new("CCF".to_string(), |emu| {
                emu.cpu.set_flag(Flag::N | Flag::H, false);
                emu.cpu.toggle_flag(Flag::C);
                InstructionStep::Complete
            }),

            // * 0x4_
            0x40 => LD_r8_r8(Register::B, Register::B),
            0x41 => LD_r8_r8(Register::B, Register::C),
            0x42 => LD_r8_r8(Register::B, Register::D),
            0x43 => LD_r8_r8(Register::B, Register::E),
            0x44 => LD_r8_r8(Register::B, Register::H),
            0x45 => LD_r8_r8(Register::B, Register::L),
            0x46 => LD_r8_r16(Register::B, RegisterPair::HL),
            0x47 => LD_r8_r8(Register::B, Register::A),
            0x48 => LD_r8_r8(Register::C, Register::B),
            0x49 => LD_r8_r8(Register::C, Register::C),
            0x4A => LD_r8_r8(Register::C, Register::D),
            0x4B => LD_r8_r8(Register::C, Register::E),
            0x4C => LD_r8_r8(Register::C, Register::H),
            0x4D => LD_r8_r8(Register::C, Register::L),
            0x4E => LD_r8_r16(Register::C, RegisterPair::HL),
            0x4F => LD_r8_r8(Register::C, Register::A),

            // * 0x5_
            0x50 => LD_r8_r8(Register::D, Register::B),
            0x51 => LD_r8_r8(Register::D, Register::C),
            0x52 => LD_r8_r8(Register::D, Register::D),
            0x53 => LD_r8_r8(Register::D, Register::E),
            0x54 => LD_r8_r8(Register::D, Register::H),
            0x55 => LD_r8_r8(Register::D, Register::L),
            0x56 => LD_r8_r16(Register::D, RegisterPair::HL),
            0x57 => LD_r8_r8(Register::D, Register::A),
            0x58 => LD_r8_r8(Register::E, Register::B),
            0x59 => LD_r8_r8(Register::E, Register::C),
            0x5A => LD_r8_r8(Register::E, Register::D),
            0x5B => LD_r8_r8(Register::E, Register::E),
            0x5C => LD_r8_r8(Register::E, Register::H),
            0x5D => LD_r8_r8(Register::E, Register::L),
            0x5E => LD_r8_r16(Register::E, RegisterPair::HL),
            0x5F => LD_r8_r8(Register::E, Register::A),

            // * 0x6_
            0x60 => LD_r8_r8(Register::H, Register::B),
            0x61 => LD_r8_r8(Register::H, Register::C),
            0x62 => LD_r8_r8(Register::H, Register::D),
            0x63 => LD_r8_r8(Register::H, Register::E),
            0x64 => LD_r8_r8(Register::H, Register::H),
            0x65 => LD_r8_r8(Register::H, Register::L),
            0x66 => LD_r8_r16(Register::H, RegisterPair::HL),
            0x67 => LD_r8_r8(Register::H, Register::A),
            0x68 => LD_r8_r8(Register::L, Register::B),
            0x69 => LD_r8_r8(Register::L, Register::C),
            0x6A => LD_r8_r8(Register::L, Register::D),
            0x6B => LD_r8_r8(Register::L, Register::E),
            0x6C => LD_r8_r8(Register::L, Register::H),
            0x6D => LD_r8_r8(Register::L, Register::L),
            0x6E => LD_r8_r16(Register::L, RegisterPair::HL),
            0x6F => LD_r8_r8(Register::L, Register::A),

            // * 0x7_
            0x70 => LD_r16_r8(RegisterPair::HL, Register::B),
            0x71 => LD_r16_r8(RegisterPair::HL, Register::C),
            0x72 => LD_r16_r8(RegisterPair::HL, Register::D),
            0x73 => LD_r16_r8(RegisterPair::HL, Register::E),
            0x74 => LD_r16_r8(RegisterPair::HL, Register::H),
            0x75 => LD_r16_r8(RegisterPair::HL, Register::L),
            0x76 => Instruction::new("HALT".to_string(), |emu| {
                emu.is_halted = true;
                InstructionStep::Complete
            }),
            0x77 => LD_r16_r8(RegisterPair::HL, Register::A),
            0x78 => LD_r8_r8(Register::A, Register::B),
            0x79 => LD_r8_r8(Register::A, Register::C),
            0x7A => LD_r8_r8(Register::A, Register::D),
            0x7B => LD_r8_r8(Register::A, Register::E),
            0x7C => LD_r8_r8(Register::A, Register::H),
            0x7D => LD_r8_r8(Register::A, Register::L),
            0x7E => LD_r8_r16(Register::A, RegisterPair::HL),
            0x7F => LD_r8_r8(Register::A, Register::A),

            // * 0x8_
            0x80 => ADD_r8_r8(Register::A, Register::B),
            0x81 => ADD_r8_r8(Register::A, Register::C),
            0x82 => ADD_r8_r8(Register::A, Register::D),
            0x83 => ADD_r8_r8(Register::A, Register::E),
            0x84 => ADD_r8_r8(Register::A, Register::H),
            0x85 => ADD_r8_r8(Register::A, Register::L),
            0x86 => ADD_r8_r16(Register::A, RegisterPair::HL),
            0x87 => ADD_r8_r8(Register::A, Register::A),
            0x88 => ADC_r8_r8(Register::A, Register::B),
            0x89 => ADC_r8_r8(Register::A, Register::C),
            0x8A => ADC_r8_r8(Register::A, Register::D),
            0x8B => ADC_r8_r8(Register::A, Register::E),
            0x8C => ADC_r8_r8(Register::A, Register::H),
            0x8D => ADC_r8_r8(Register::A, Register::L),
            0x8E => ADC_r8_r16(Register::A, RegisterPair::HL),
            0x8F => ADC_r8_r8(Register::A, Register::A),

            // * 0x9_
            0x90 => SUB_r8_r8(Register::A, Register::B),
            0x91 => SUB_r8_r8(Register::A, Register::C),
            0x92 => SUB_r8_r8(Register::A, Register::D),
            0x93 => SUB_r8_r8(Register::A, Register::E),
            0x94 => SUB_r8_r8(Register::A, Register::H),
            0x95 => SUB_r8_r8(Register::A, Register::L),
            0x96 => SUB_r8_r16(Register::A, RegisterPair::HL),
            0x97 => SUB_r8_r8(Register::A, Register::A),
            0x98 => SBC_r8_r8(Register::A, Register::B),
            0x99 => SBC_r8_r8(Register::A, Register::C),
            0x9A => SBC_r8_r8(Register::A, Register::D),
            0x9B => SBC_r8_r8(Register::A, Register::E),
            0x9C => SBC_r8_r8(Register::A, Register::H),
            0x9D => SBC_r8_r8(Register::A, Register::L),
            0x9E => SBC_r8_r16(Register::A, RegisterPair::HL),
            0x9F => SBC_r8_r8(Register::A, Register::A),

            // * 0xA_
            0xA0 => AND_r8_r8(Register::A, Register::B),
            0xA1 => AND_r8_r8(Register::A, Register::C),
            0xA2 => AND_r8_r8(Register::A, Register::D),
            0xA3 => AND_r8_r8(Register::A, Register::E),
            0xA4 => AND_r8_r8(Register::A, Register::H),
            0xA5 => AND_r8_r8(Register::A, Register::L),
            0xA6 => AND_r8_r16(Register::A, RegisterPair::HL),
            0xA7 => AND_r8_r8(Register::A, Register::A),
            0xA8 => XOR_r8_r8(Register::A, Register::B),
            0xA9 => XOR_r8_r8(Register::A, Register::C),
            0xAA => XOR_r8_r8(Register::A, Register::D),
            0xAB => XOR_r8_r8(Register::A, Register::E),
            0xAC => XOR_r8_r8(Register::A, Register::H),
            0xAD => XOR_r8_r8(Register::A, Register::L),
            0xAE => XOR_r8_r16(Register::A, RegisterPair::HL),
            0xAF => XOR_r8_r8(Register::A, Register::A),

            // * 0xB_
            0xB0 => OR_r8_r8(Register::A, Register::B),
            0xB1 => OR_r8_r8(Register::A, Register::C),
            0xB2 => OR_r8_r8(Register::A, Register::D),
            0xB3 => OR_r8_r8(Register::A, Register::E),
            0xB4 => OR_r8_r8(Register::A, Register::H),
            0xB5 => OR_r8_r8(Register::A, Register::L),
            0xB6 => OR_r8_r16(Register::A, RegisterPair::HL),
            0xB7 => OR_r8_r8(Register::A, Register::A),
            0xB8 => CP_r8_r8(Register::A, Register::B),
            0xB9 => CP_r8_r8(Register::A, Register::C),
            0xBA => CP_r8_r8(Register::A, Register::D),
            0xBB => CP_r8_r8(Register::A, Register::E),
            0xBC => CP_r8_r8(Register::A, Register::H),
            0xBD => CP_r8_r8(Register::A, Register::L),
            0xBE => CP_r8_r16(Register::A, RegisterPair::HL),
            0xBF => CP_r8_r8(Register::A, Register::A),

            // * 0xC_
            0xC0 => RET_nc(Flag::Z),
            0xC1 => POP_r16(RegisterPair::BC),
            0xC2 => JP_nc_n16(Flag::Z),
            0xC3 => JP_n16(),
            0xC4 => CALL_nc_n16(Flag::Z),
            0xC5 => PUSH_r16(RegisterPair::BC),
            0xC6 => Instruction::new("ADD A, n8".to_string(), move |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let mut value = emu.read_pc();
                    value = emu.cpu.add_register(Register::A, value);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xC7 => RST_n16(0x00),
            0xC8 => RET_c(Flag::Z),
            0xC9 => RET(),
            0xCA => JP_c_n16(Flag::Z),
            0xCB => PREFIX_n8(),
            0xCC => CALL_c_n16(Flag::Z),
            0xCD => CALL_n16(),
            0xCE => Instruction::new("ADC A, n8".to_string(), move |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let mut value = emu.read_pc() + emu.cpu.get_flag(Flag::C) as u8;
                    value = emu.cpu.add_register(Register::A, value);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xCF => RST_n16(0x08),

            // * 0xD_
            0xD0 => RET_nc(Flag::C),
            0xD1 => POP_r16(RegisterPair::DE),
            0xD2 => JP_nc_n16(Flag::C),
            0xD3 => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xD4 => CALL_nc_n16(Flag::C),
            0xD5 => POP_r16(RegisterPair::DE),
            0xD6 => Instruction::new("SUB A, n8".to_string(), move |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let mut value = emu.read_pc();
                    value = emu.cpu.sub_register(Register::A, value);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xD7 => RST_n16(0x10),
            0xD8 => RET_c(Flag::C),
            0xD9 => Instruction::new("RETI".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let lsb = emu.read_sp();
                    InstructionStep::new(move |emu| {
                        let msb = emu.read_sp();
                        InstructionStep::new(move |emu| {
                            emu.cpu
                                .set_register_pair(RegisterPair::PC, join_u16(lsb, msb));
                            emu.ime = IME::Enabled;
                            InstructionStep::Complete
                        })
                    })
                })
            }),
            0xDA => JP_c_n16(Flag::C),
            0xDB => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xDC => CALL_c_n16(Flag::C),
            0xDD => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xDE => Instruction::new("SBC A, n8".to_string(), move |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let mut value = emu.read_pc() + emu.cpu.get_flag(Flag::C) as u8;
                    value = emu.cpu.sub_register(Register::A, value);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xDF => RST_n16(0x18),

            // * 0xE_
            0xE0 => Instruction::new("LD (0xFF00 + n8), A".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let address = 0xFF00 + emu.read_pc() as u16;
                    InstructionStep::new(move |emu| {
                        let value = emu.cpu.get_register(Register::A);
                        Bus::write(emu, address, value);
                        InstructionStep::Complete
                    })
                })
            }),
            0xE1 => POP_r16(RegisterPair::HL),
            0xE2 => Instruction::new("LD (0xFF00 + C), A".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let address = 0xFF00 + emu.cpu.get_register(Register::C) as u16;
                    let value = emu.cpu.get_register(Register::A);
                    Bus::write(emu, address, value);
                    InstructionStep::Complete
                })
            }),
            0xE3 => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xE4 => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xE5 => PUSH_r16(RegisterPair::HL),
            0xE6 => Instruction::new("AND A, n8".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let mut value = emu.read_pc();
                    value = emu.cpu.and_register(Register::A, value);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xE7 => RST_n16(0x20),
            0xE8 => Instruction::new("ADD SP, i8".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.read_pc() as i8;
                    InstructionStep::new(move |emu| {
                        // ? Techincally writes upper and lower bytes seperately.
                        let value = emu
                            .cpu
                            .get_register_pair(RegisterPair::SP)
                            .wrapping_add_signed(value.into());
                        InstructionStep::new(move |emu| {
                            emu.cpu.set_register_pair(RegisterPair::SP, value);
                            InstructionStep::Complete
                        })
                    })
                })
            }),
            0xE9 => Instruction::new("JP HL".to_string(), |emu| {
                let value = emu.cpu.get_register_pair(RegisterPair::HL);
                emu.cpu.set_register_pair(RegisterPair::PC, value);
                InstructionStep::Complete
            }),
            0xEA => Instruction::new("LD (n16), A".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let lsb = emu.read_pc();
                    InstructionStep::new(move |emu| {
                        let msb = emu.read_pc();
                        let address = join_u16(lsb, msb);
                        InstructionStep::new(move |emu| {
                            let value = emu.cpu.get_register(Register::A);
                            Bus::write(emu, address, value);
                            InstructionStep::Complete
                        })
                    })
                })
            }),
            0xEB => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xEC => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xED => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xEE => Instruction::new("XOR A, n8".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let mut value = emu.read_pc();
                    value = emu.cpu.xor_register(Register::A, value);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xEF => RST_n16(0x28),

            // * 0xF_
            0xF0 => Instruction::new("LD A, (0xFF00 + n8)".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let address = 0xFF00 + emu.read_pc() as u16;
                    InstructionStep::new(move |emu| {
                        let value = Bus::read(emu, address);
                        emu.cpu.set_register(Register::A, value);
                        InstructionStep::Complete
                    })
                })
            }),
            0xF1 => POP_r16(RegisterPair::AF),
            0xF2 => Instruction::new("LD A, (0xFF00 + C)".to_string(), |emu| {
                let address = 0xFF00 + emu.cpu.get_register(Register::C) as u16;
                InstructionStep::new(move |emu| {
                    let value = Bus::read(emu, address);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xF3 => Instruction::new("DI".to_string(), |emu| {
                emu.ime = IME::Disabled;
                InstructionStep::Complete
            }),
            0xF4 => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xF5 => PUSH_r16(RegisterPair::AF),
            0xF6 => Instruction::new("OR A, u8".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let mut value = emu.read_pc();
                    value = emu.cpu.or_register(Register::A, value);
                    emu.cpu.set_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xF7 => RST_n16(0x30),
            0xF8 => Instruction::new("LD HL, SP + i8".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.read_pc() as i8;
                    InstructionStep::new(move |emu| {
                        let value = emu
                            .cpu
                            .get_register_pair(RegisterPair::SP)
                            .wrapping_add_signed(value.into());
                        emu.cpu.set_register_pair(RegisterPair::HL, value);
                        InstructionStep::Complete
                    })
                })
            }),
            0xF9 => Instruction::new("LD SP, HL".to_string(), |emu| {
                let value = emu.cpu.get_register_pair(RegisterPair::HL);
                emu.cpu.set_register_pair(RegisterPair::SP, value);
                InstructionStep::Complete
            }),
            0xFA => Instruction::new("LD A, (n16)".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let lsb = emu.read_pc();
                    InstructionStep::new(move |emu| {
                        let msb = emu.read_pc();
                        InstructionStep::new(move |emu| {
                            let address = join_u16(lsb, msb);
                            let value = Bus::read(emu, address);
                            emu.cpu.set_register(Register::A, value);
                            InstructionStep::Complete
                        })
                    })
                })
            }),
            0xFB => Instruction::new("EI".to_string(), |emu| {
                emu.ime = IME::Scheduled;
                InstructionStep::Complete
            }),
            0xFC => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xFD => unimplemented!("GB - {:#X} is and invalid opcode!", value),
            0xFE => Instruction::new("CP A, n8".to_string(), |_emu| {
                // ? One bus read or write per m-cycle.
                InstructionStep::new(move |emu| {
                    let value = emu.read_pc();
                    emu.cpu.sub_register(Register::A, value);
                    InstructionStep::Complete
                })
            }),
            0xFF => RST_n16(0x38),
        }
    }
}
