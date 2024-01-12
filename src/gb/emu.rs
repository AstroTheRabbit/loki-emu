use super::{bus::Bus, cpu::CPU, utils::*};

#[derive(Debug)]
pub struct GameBoyEmulator {
    pub cpu: CPU,
    pub bus: Bus,
    pub ime: IME,
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
}
