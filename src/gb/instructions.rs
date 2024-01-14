use super::{emu::GameBoyEmulator, operations::*, utils::*};

/// A 4 t-cycle long step of an instruction, either returning the next step or signalling the instruction's completion.
pub enum InstructionStep {
    Running(Box<dyn FnOnce(&mut GameBoyEmulator) -> InstructionStep>),
    Complete,
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
        F: FnOnce(&mut GameBoyEmulator) -> InstructionStep + 'static,
    {
        Self::Running(Box::new(func))
    }
}

pub struct Instruction {
    pub mnemomic: &'static str,
    /// Code of the instruction split into steps each 4 t-cycles long.
    current_step: InstructionStep,
}

impl Instruction {
    pub const fn new<F>(mnemomic: &'static str, steps: F) -> Self
    where
        F: FnOnce(&mut GameBoyEmulator) -> InstructionStep + 'static,
    {
        Self {
            mnemomic,
            current_step: InstructionStep::new(steps),
        }
    }

    /// Runs one step of the instruction, returning true if the instruction has fully completed.
    pub fn step(&mut self, emu: &mut GameBoyEmulator) -> bool {
        if let InstructionStep::Running(step) = self.current_step {
            self.current_step = step(emu);
        }
        return self.current_step.is_complete();
    }
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Instruction::new("NOP", &|_| InstructionStep::Complete),
            0x01 => LD_r16_n16(RegisterPair::BC),
            0x02 => LD_r16_r8(RegisterPair::BC, Register::A),
            0x03 => INC_r16(RegisterPair::BC),
            0x04 => INC_r8(Register::B),
            0x05 => DEC_r8(Register::B),
            0x06 => LD_r8_n8(Register::B),
            0x07 => Instruction::new(
                "RLCA",
                vec![&|_, emu| {
                    // ? Slightly different to prefixed `RLC A` (zero flag is always unset, not dependent).
                    let v = emu.cpu.get_register(Register::A);
                    let new_carry = get_bit(&v, 0b1000_0000);
                    let v = (v << 1) | new_carry as u8;

                    emu.cpu.set_register(Register::A, v);
                    emu.cpu.set_flag(Flag::C, new_carry);
                    emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                }],
            ),
            0x08 => LD_a16_r16(RegisterPair::SP),
            0x09 => ADD_r16_r16(RegisterPair::HL, RegisterPair::BC),
            0x0A => LD_r8_r16(Register::A, RegisterPair::BC),
            0x0B => DEC_r16(RegisterPair::BC),
            0x0C => INC_r8(Register::C),
            0x0D => DEC_r8(Register::C),
            0x0E => LD_r8_n8(Register::C),
            0x0F => Instruction::new(
                "RRCA",
                vec![&|_, emu| {
                    // ? Slightly different to prefixed `RRC A` (zero flag is always unset, not dependent).
                    let v = emu.cpu.get_register(Register::A);
                    let new_carry = get_bit(&v, 0b0000_0001);
                    let v = (v >> 1) | ((new_carry as u8) << 7);

                    emu.cpu.set_register(Register::A, v);
                    emu.cpu.set_flag(Flag::C, new_carry);
                    emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                }],
            ),

            0x10 => Instruction::new("STOP", vec![&|[_, _, _, _], emu| emu.is_halted = true]),
            0x11 => LD_r16_n16(RegisterPair::DE),
            0x12 => LD_r16_r8(RegisterPair::DE, Register::A),
            0x13 => INC_r16(RegisterPair::DE),
            0x14 => INC_r8(Register::D),
            0x15 => DEC_r8(Register::D),
            0x16 => LD_r8_n8(Register::D),
            0x17 => Instruction::new(
                "RLA",
                vec![&|_, emu| {
                    // ? Slightly different to prefixed `RL A` (zero flag is always unset, not dependent).
                    let v = emu.cpu.get_register(Register::A);
                    let prev_carry = emu.cpu.get_flag(Flag::C);
                    let new_carry = get_bit(&v, 0b1000_0000);
                    let v = (v << 1) | prev_carry as u8;

                    emu.cpu.set_register(Register::A, v);
                    emu.cpu.set_flag(Flag::C, new_carry);
                    emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                }],
            ),
            0x18 => LD_a16_r16(RegisterPair::SP),
            0x19 => ADD_r16_r16(RegisterPair::HL, RegisterPair::DE),
            0x1A => LD_r8_r16(Register::A, RegisterPair::DE),
            0x1B => DEC_r16(RegisterPair::DE),
            0x1C => INC_r8(Register::E),
            0x1D => DEC_r8(Register::E),
            0x1E => LD_r8_n8(Register::E),
            0x1F => Instruction::new(
                "RRA",
                vec![&|_, emu| {
                    // ? Slightly different to prefixed `RRC A` (zero flag is always unset, not dependent).
                    let v = emu.cpu.get_register(Register::A);
                    let prev_carry = emu.cpu.get_flag(Flag::C);
                    let new_carry = get_bit(&v, 0b0000_0001);
                    let v = (v >> 1) | ((prev_carry as u8) << 7);

                    emu.cpu.set_register(Register::A, v);
                    emu.cpu.set_flag(Flag::C, new_carry);
                    emu.cpu.set_flag(Flag::Z | Flag::N | Flag::H, false);
                }],
            ),
            // 0x20 =>
        }
    }
}
