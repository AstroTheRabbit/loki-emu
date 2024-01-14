use softbuffer::Buffer;
use std::rc::Rc;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::RenderBuffer;

use super::{bus::Bus, cpu::CPU, utils::*};

#[derive(Debug)]
pub struct GameBoyEmulator {
    pub cpu: CPU,
    pub ime: IME,
    pub is_halted: bool,
    pub current_cycles: usize,
    pub bus: Bus,
}

impl GameBoyEmulator {
    pub fn update(
        &mut self,
        window: Rc<Window>,
        input: &mut WinitInputHelper,
        buffer: &mut RenderBuffer,
    ) {
        if self.is_halted {
            return;
        }

        let next_instruction = self.read_pc();
        // ! exec instruction !

        // TODO: Rendering, input, audio, interupts

        self.render(buffer);
    }

    /// Read and return a byte from the address of the `PC`, then increment `PC`.
    #[inline]
    pub fn read_pc(&mut self) -> u8 {
        let address = self.cpu.get_register_pair(RegisterPair::PC);
        self.cpu.inc_register_pair(RegisterPair::PC);
        return self.bus.read(address);
    }

    /// Read a byte from the address `r16`.
    #[inline]
    pub fn read_r16(&mut self, r16: RegisterPair) -> u8 {
        let address = self.cpu.get_register_pair(r16);
        return self.bus.read(address);
    }

    /// Write a byte to the address `r16`.
    #[inline]
    pub fn write_r16(&mut self, r16: RegisterPair, value: u8) {
        let address = self.cpu.get_register_pair(r16);
        return self.bus.write(address, value);
    }

    pub fn render(&mut self, buffer: &mut RenderBuffer) {}
}
