use super::instructions::Instruction;

/// [pandocs](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
pub struct CPURegisters {
    /// Accumulator
    pub a: u8,
    /// Flags
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    /// Program counter/pointer
    pub pc: u16,
    /// Stack pointer
    pub sp: u16,
}

impl CPURegisters {
    pub fn new_init() -> Self {
        Self {
            a: 0x01,
            f: 0xb0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            pc: 0x0100,
            sp: 0xfffe,
        }
    }
}

pub struct CPUContext {
    pub registers: CPURegisters,

    // current fetch
    pub fetched_data: u16,
    pub mem_dest: u16,
    pub dest_is_mem: bool,
    pub current_opcode: u8,
    pub current_instruction: Instruction,

    pub halted: bool,
    pub stepping: bool,
    pub int_master_enabled: bool,
}
