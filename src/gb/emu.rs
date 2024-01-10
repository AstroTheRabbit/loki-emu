use super::{bus::Bus, cpu::CPU};

#[derive(Debug)]
pub struct Emulator {
    pub cpu: CPU,
    pub bus: Bus,
}

impl Emulator {
    pub fn step(&mut self) {}
}
