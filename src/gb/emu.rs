use std::rc::Rc;
use std::time::Instant;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use super::instructions::operations::INTERRUPT;
use super::io::graphics::PPU;
use super::{bus::Bus, cpu::CPU, instructions::instructions::Instruction, utils::*};
use crate::gb::io::io_registers::IORegisters;
use crate::RenderBuffer;

#[derive(Debug)]
pub struct GameboyEmulator {
    pub prev_update: Instant,
    pub cpu: CPU,
    pub ppu: PPU,
    pub ime: IME,
    pub bus: Bus,
    pub is_halted: bool,
    pub io_registers: IORegisters,
    pub current_instruction: Instruction,
}

impl GameboyEmulator {
    pub fn update(
        &mut self,
        input: &mut WinitInputHelper,
        _window: Rc<Window>,
        _buffer: &mut RenderBuffer,
    ) {
        // ? Get, wait and update the time between m-cycles.
        // let prev_update = std::mem::replace(&mut self.prev_update, Instant::now());
        // let delta_time = self.prev_update.duration_since(prev_update);
        // println!("{:?}", delta_time.as_nanos() as f64 / 1000000.);
        // let wait = prev_update + Duration::from_secs_f64(2.38418579102e-7);
        // while Instant::now() < wait {
        //     continue;
        // }

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
            // ? Check for any enabled interrupts first.
            let pc = self.cpu.get_register_pair(RegisterPair::PC);
            let mut interrupt_occurred  = false;
            if self.ime == IME::Enabled {
                let enabled_interrupts = Bus::read(self, 0xFF0F) & Bus::read(self, 0xFFFF);
                if let Some(current_interrupt) = InterruptMask::get_interrupt_from_register(enabled_interrupts) {
                    self.ime = IME::Disabled;
                    self.current_instruction = INTERRUPT(current_interrupt);
                    self.set_interrupt_flag(current_interrupt, false);
                    interrupt_occurred = true;
                } else {
                    self.current_instruction = self.read_pc().into();
                }
            }
            if !interrupt_occurred {
                self.current_instruction = self.read_pc().into();
            }
            println!("0x{:0>4X} - {}", pc, &self.current_instruction);
        }

        // ? Run the current instruction.
        let mut instruction = std::mem::take(&mut self.current_instruction);
        instruction.step(self);
        self.current_instruction = instruction;

        // TODO: Interrupts, graphics, audio, serial I/O
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

    #[inline]
    pub fn set_interrupt_flag(&mut self, interrupt: InterruptMask, state: bool) {
        let mut interrupts = Bus::read(self, 0xFF0F);
        set_bit(&mut interrupts, interrupt, state);
        Bus::write(self, 0xFF0F, interrupts);
    }
}
