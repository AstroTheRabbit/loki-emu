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

        let next_instruction = self.read_u8(RegisterPair::PC);
        self.run_instruction(next_instruction);

        // TODO: Rendering, input, audio, interupts

        self.render(buffer);
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

    pub fn render(&mut self, buffer: &mut RenderBuffer) {}
}
