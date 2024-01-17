use std::rc::Rc;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use super::{bus::Bus, cpu::CPU, instructions::instructions::Instruction, utils::*};
use crate::gb::io::io_registers::IORegisters;
use crate::RenderBuffer;

#[derive(Debug)]
pub struct GameboyEmulator {
    pub cpu: CPU,
    pub ime: IME,
    pub bus: Bus,
    pub is_halted: bool,
    pub io_registers: IORegisters,
    pub current_instruction: Instruction,
}

impl GameboyEmulator {
    pub fn update(
        &mut self,
        _window: Rc<Window>,
        input: &mut WinitInputHelper,
        buffer: &mut RenderBuffer,
    ) {
        if self.is_halted {
            return;
        }

        IORegisters::update(self, input);

        // ? Update IME state if `EI` was called.
        if self.ime == IME::Scheduled {
            self.ime = IME::Enabled;
        }

        // ? Get the next instruction if the previous instruction has completed.
        if self.current_instruction.has_completed() {
            let pc = self.cpu.get_register_pair(RegisterPair::PC);
            self.current_instruction = self.read_pc().into();
            println!("{:>19}   at PC = {:#06X}", self.current_instruction, pc);
        }

        // ? Run the current instruction.
        let mut instruction = std::mem::take(&mut self.current_instruction);
        instruction.step(self);
        self.current_instruction = instruction;

        // TODO: Graphics, timer, interupts, input, audio

        self.render(buffer);
    }

    /// Read and return a byte from the address of the `PC`, then increment `PC`.
    #[inline]
    pub fn read_pc(&mut self) -> u8 {
        let address = self.cpu.get_register_pair(RegisterPair::PC);
        self.cpu.inc_register_pair(RegisterPair::PC);
        Bus::read(self, address)
    }

    /// Read and return a byte from the address of the `SP`, then increment `SP`.
    #[inline]
    pub fn read_sp(&mut self) -> u8 {
        let address = self.cpu.get_register_pair(RegisterPair::SP);
        self.cpu.inc_register_pair(RegisterPair::SP);
        Bus::read(self, address)
    }

    /// Decrement the `SP`, then write a byte to its address.
    #[inline]
    pub fn write_sp(&mut self, value: u8) {
        self.cpu.dec_register_pair(RegisterPair::SP);
        let address = self.cpu.get_register_pair(RegisterPair::SP);
        Bus::write(self, address, value)
    }

    /// Read a byte from the address `r16`.
    #[inline]
    pub fn read_r16(&mut self, r16: RegisterPair) -> u8 {
        let address = self.cpu.get_register_pair(r16);
        Bus::read(self, address)
    }

    /// Write a byte to the address `r16`.
    #[inline]
    pub fn write_r16(&mut self, r16: RegisterPair, value: u8) {
        let address = self.cpu.get_register_pair(r16);
        Bus::write(self, address, value)
    }

    pub fn render(&mut self, _buffer: &mut RenderBuffer) {}
}
