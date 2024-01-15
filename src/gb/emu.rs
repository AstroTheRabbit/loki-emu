
use std::rc::Rc;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::RenderBuffer;

use super::{bus::Bus, cpu::CPU, utils::*, instructions::Instruction};

#[derive(Debug)]
pub struct GameBoyEmulator {
    pub cpu: CPU,
    pub ime: IME,
    pub is_halted: bool,
    pub current_instruction: Instruction,
    pub bus: Bus,
}

impl GameBoyEmulator {
    pub fn update(
        &mut self,
        _window: Rc<Window>,
        _input: &mut WinitInputHelper,
        buffer: &mut RenderBuffer,
    ) {
        if self.is_halted {
            return;
        }

        // ? Get the next instruction if the previous instruction has completed.
        if self.current_instruction.has_completed() {
            self.current_instruction = self.read_pc().into();
            println!("{}", self.current_instruction);
        }

        // ? Run the current instruction.
        let mut instruction = std::mem::take(&mut self.current_instruction);
        instruction.step(self);
        self.current_instruction = instruction;

        // TODO: Rendering, input, audio, interupts

        self.render(buffer);
    }

    /// Read and return a byte from the address of the `PC`, then increment `PC`.
    #[inline]
    pub fn read_pc(&mut self) -> u8 {
        let address = self.cpu.get_register_pair(RegisterPair::PC);
        self.cpu.inc_register_pair(RegisterPair::PC);
        self.bus.read(address)
    }

    /// Read and return a byte from the address of the `SP`, then increment `SP`.
    #[inline]
    pub fn read_sp(&mut self) -> u8 {
        let address = self.cpu.get_register_pair(RegisterPair::SP);
        self.cpu.inc_register_pair(RegisterPair::SP);
        self.bus.read(address)
    }

    /// Decrement the `SP`, then write a byte to its address.
    #[inline]
    pub fn write_sp(&mut self, value: u8) {
        self.cpu.dec_register_pair(RegisterPair::SP);
        let address = self.cpu.get_register_pair(RegisterPair::SP);
        self.bus.write(address, value)
    }

    /// Read a byte from the address `r16`.
    #[inline]
    pub fn read_r16(&mut self, r16: RegisterPair) -> u8 {
        let address = self.cpu.get_register_pair(r16);
        self.bus.read(address)
    }

    /// Write a byte to the address `r16`.
    #[inline]
    pub fn write_r16(&mut self, r16: RegisterPair, value: u8) {
        let address = self.cpu.get_register_pair(r16);
        self.bus.write(address, value)
    }

    pub fn render(&mut self, _buffer: &mut RenderBuffer) {}
}
