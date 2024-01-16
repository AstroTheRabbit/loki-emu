use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

use crate::Bus;

use super::{utils::{split_u16, get_bit, set_bit}, emu::GameboyEmulator};

#[derive(Debug)]
pub struct IORegisters {
    pub joypad: Joypad,
    pub timer: Timer,
}

impl IORegisters {
    pub fn new() -> Self {
        Self {
            joypad: Joypad::new(),
            timer: Timer::new(),
        }
    }

    /// Updates timers and I/O as if 4 t-cycles have passed.
    pub fn update(emu: &mut GameboyEmulator, input: &mut WinitInputHelper) {
        emu.io_registers.joypad.update(input);
        Timer::update(emu);
    }

    pub fn read(&self, index: usize) -> u8 {
        match index {
            0x0000 => self.joypad.read(),
            0x0001 => todo!("GB - IO: Serial byte"),
            0x0002 => todo!("GB - IO: Serial control"),
            0x0003 => unimplemented!("GB - IO: Unmapped"),
            0x0004 => split_u16(self.timer.div).1,
            0x0005 => self.timer.tima,
            0x0006 => self.timer.tma,
            0x0007 => self.timer.tac,
            0x0008..=0x000E => unimplemented!("GB - IO: Unmapped"),
            0x000F => todo!("GB - IO: IF"),
            0xFF10 => todo!("GB - IO: Audio channel 1 sweep"),
            0xFF11 => todo!("GB - IO: Audio channel 1 sound length/wave duty"),
            0xFF12 => todo!("GB - IO: Audio channel 1 envelope"),
            0xFF13 => todo!("GB - IO: Audio channel 1 frequency"),
            0xFF14 => todo!("GB - IO: Audio channel 1 control"),
            0xFF15 => unimplemented!("GB - IO: Unmapped"),
            0xFF16 => todo!("GB - IO: Audio channel 2 sound length/wave duty"),
            0xFF17 => todo!("GB - IO: Audio channel 2 envelope"),
            0xFF18 => todo!("GB - IO: Audio channel 2 frequency"),
            0xFF19 => todo!("GB - IO: Audio channel 2 control"),
            0xFF1A => todo!("GB - IO: Audio channel 3 enable"),
            0xFF1B => todo!("GB - IO: Audio channel 3 sound length"),
            0xFF1C => todo!("GB - IO: Audio channel 3 volume"),
            0xFF1D => todo!("GB - IO: Audio channel 3 frequency"),
            0xFF1E => todo!("GB - IO: Audio channel 3 control"),
            0xFF1F => unimplemented!("GB - IO: Unmapped"),
            0xFF20 => todo!("GB - IO: Audio channel 4 sound length"),
            0xFF21 => todo!("GB - IO: Audio channel 4 volume"),
            0xFF22 => todo!("GB - IO: Audio channel 4 frequency"),
            0xFF23 => todo!("GB - IO: Audio channel 4 control"),
            0xFF24 => todo!("GB - IO: Audio output mapping"),
            0xFF25 => todo!("GB - IO: Audio channel mapping"),
            0xFF26 => todo!("GB - IO: Audio channel control"),
            0xFF27..=0xFF2F => unimplemented!("GB - IO: Unmapped"),
            0xFF30..=0xFF3F => todo!("GB - IO: Wave pattern"),
            0xFF40 => todo!("GB - IO: LCDC"),
            0xFF41 => todo!("GB - IO: STAT"),
            0xFF42 => todo!("GB - IO: SCY"),
            0xFF43 => todo!("GB - IO: SCX"),
            0xFF44 => todo!("GB - IO: LY"),
            0xFF45 => todo!("GB - IO: LYC"),
            0xFF46 => todo!("GB - IO: DMA"),
            0xFF47 => todo!("GB - IO: BGP"),
            0xFF48 => todo!("GB - IO: OBP0"),
            0xFF49 => todo!("GB - IO: OBP1"),
            0xFF4A => todo!("GB - IO: WY"),
            0xFF4B => todo!("GB - IO: WX"),
            0xFF50 => todo!("GB - IO: Boot ROM control"),
            0xFFFF => todo!("GB - IO: IE"),
            _ => panic!("GB - IO: Index out of range!"),
        }
    }

    pub fn write(&mut self, index: usize, value: u8) {
        match index {
            0x0000 => self.joypad.write(value),
            0x0001 => todo!("GB - IO: Serial byte"),
            0x0002 => todo!("GB - IO: Serial control"),
            0x0003 => unimplemented!("GB - IO: Unmapped"),
            0x0004 => self.timer.write_div(),
            0x0005 => self.timer.write_tima(value),
            0x0006 => self.timer.tma = value,
            0x0007 => self.timer.tac = value,
            0x0008..=0x000E => unimplemented!("GB - IO: Unmapped"),
            0x000F => todo!("GB - IO: IF"),
            0xFF10 => todo!("GB - IO: Audio channel 1 sweep"),
            0xFF11 => todo!("GB - IO: Audio channel 1 sound length/wave duty"),
            0xFF12 => todo!("GB - IO: Audio channel 1 envelope"),
            0xFF13 => todo!("GB - IO: Audio channel 1 frequency"),
            0xFF14 => todo!("GB - IO: Audio channel 1 control"),
            0xFF15 => unimplemented!("GB - IO: Unmapped"),
            0xFF16 => todo!("GB - IO: Audio channel 2 sound length/wave duty"),
            0xFF17 => todo!("GB - IO: Audio channel 2 envelope"),
            0xFF18 => todo!("GB - IO: Audio channel 2 frequency"),
            0xFF19 => todo!("GB - IO: Audio channel 2 control"),
            0xFF1A => todo!("GB - IO: Audio channel 3 enable"),
            0xFF1B => todo!("GB - IO: Audio channel 3 sound length"),
            0xFF1C => todo!("GB - IO: Audio channel 3 volume"),
            0xFF1D => todo!("GB - IO: Audio channel 3 frequency"),
            0xFF1E => todo!("GB - IO: Audio channel 3 control"),
            0xFF1F => unimplemented!("GB - IO: Unmapped"),
            0xFF20 => todo!("GB - IO: Audio channel 4 sound length"),
            0xFF21 => todo!("GB - IO: Audio channel 4 volume"),
            0xFF22 => todo!("GB - IO: Audio channel 4 frequency"),
            0xFF23 => todo!("GB - IO: Audio channel 4 control"),
            0xFF24 => todo!("GB - IO: Audio output mapping"),
            0xFF25 => todo!("GB - IO: Audio channel mapping"),
            0xFF26 => todo!("GB - IO: Audio channel control"),
            0xFF27..=0xFF2F => unimplemented!("GB - IO: Unmapped"),
            0xFF30..=0xFF3F => todo!("GB - IO: Wave pattern"),
            0xFF40 => todo!("GB - IO: LCDC"),
            0xFF41 => todo!("GB - IO: STAT"),
            0xFF42 => todo!("GB - IO: SCY"),
            0xFF43 => todo!("GB - IO: SCX"),
            0xFF44 => todo!("GB - IO: LY"),
            0xFF45 => todo!("GB - IO: LYC"),
            0xFF46 => todo!("GB - IO: DMA"),
            0xFF47 => todo!("GB - IO: BGP"),
            0xFF48 => todo!("GB - IO: OBP0"),
            0xFF49 => todo!("GB - IO: OBP1"),
            0xFF4A => todo!("GB - IO: WY"),
            0xFF4B => todo!("GB - IO: WX"),
            0xFF50 => todo!("GB - IO: Boot ROM control"),
            0xFFFF => todo!("GB - IO: IE"),
            _ => panic!("GB - IO: Index out of range!"),
        }
    }
}

/// Info from the [Open Game Boy Documentation Project](https://mgba-emu.github.io/gbdoc/#mmio-p1).
#[derive(Debug)]
pub struct Joypad {
    /// A `u8` that determines whether each of the player's inputs are currently being pressed (set to 0) or not (set to 1).
    /// * bit 0: A
    /// * bit 1: B
    /// * bit 2: Select
    /// * bit 3: Start
    /// * bit 4: Right
    /// * bit 5: Left
    /// * bit 6: Up
    /// * bit 7: Down
    input_state: u8,
    /// Determines whether the directional inputs are read.
    bit_4_low: bool,
    /// Determines whether the non-directional inputs are read.
    bit_5_low: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Self { input_state: 0xFF, bit_4_low: true, bit_5_low: true }
    }

    pub fn update(&mut self, input: &mut WinitInputHelper) {
        // TODO: Keybinds settings.
        set_bit(&mut self.input_state, 0b0000_0001, !input.key_held(KeyCode::KeyQ));
        set_bit(&mut self.input_state, 0b0000_0010, !input.key_held(KeyCode::KeyE));
        set_bit(&mut self.input_state, 0b0000_0100, !input.key_held(KeyCode::KeyZ));
        set_bit(&mut self.input_state, 0b0000_1000, !input.key_held(KeyCode::KeyX));
        set_bit(&mut self.input_state, 0b0001_0000, !input.key_held(KeyCode::KeyD));
        set_bit(&mut self.input_state, 0b0010_0000, !input.key_held(KeyCode::KeyA));
        set_bit(&mut self.input_state, 0b0100_0000, !input.key_held(KeyCode::KeyW));
        set_bit(&mut self.input_state, 0b1000_0000, !input.key_held(KeyCode::KeyS));
    }

    pub fn read(&self) -> u8 {
        match self.bit_4_low {
            true => match self.bit_5_low {
                true => 0b1100_0000 | (self.get_directional() & self.get_nondirectional()),
                false => 0b1101_0000 | self.get_directional(),
            },
            false => match self.bit_5_low {
                true => 0b1110_0000 | self.get_nondirectional(),
                false => 0b1111_1111,
            },
        }
    }

    pub fn write(&mut self, value: u8) {
        self.bit_4_low = get_bit(value, 0b0001_0000);
        self.bit_4_low = get_bit(value, 0b0001_0000);
    }

    #[inline]
    fn get_directional(&self) -> u8 {
        self.input_state >> 4
    }

    #[inline]
    fn get_nondirectional(&self) -> u8 {
        self.input_state & 0x0F
    }
}

#[derive(Debug)]
pub struct Timer {
    /// Divider register: Incremented every t-cycle.
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    pub prev_and_result: bool,
    pub cycles_since_tima_overflow: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            prev_and_result: false,
            cycles_since_tima_overflow: 0,
        }
    }

    /// Update the timer as if 4 t-cycles have passed.
    pub fn update(emu: &mut GameboyEmulator) {
        // ? https://hacktix.github.io/GBEDG/timers/#timer-operation
        for _ in 0..4 {
            emu.io_registers.timer.div += 1;

            if emu.io_registers.timer.cycles_since_tima_overflow > 0 {
                emu.io_registers.timer.cycles_since_tima_overflow -= 1;
                if emu.io_registers.timer.cycles_since_tima_overflow == 0 {
                    emu.io_registers.timer.tima = emu.io_registers.timer.tma;
                    let value = Bus::read(emu, 0xFF0F) | 0000_0100;
                    Bus::write(emu, 0xFF0F, value);
                }
            } else {
                let bitmask = match emu.io_registers.timer.tac & 0b0011 {
                    0b00 => 0b0010_0000_0000,
                    0b01 => 0b0000_0000_1000,
                    0b10 => 0b0000_0010_0000,
                    0b11 => 0b0000_1000_0000,
                    _ => panic!("GB - Timer: Invalid result?!"),
                };
                let div_bit = emu.io_registers.timer.div & bitmask != 0;
                let timer_enable = get_bit(emu.io_registers.timer.tac, 0b0100);
                let and_result = div_bit & timer_enable;

                if emu.io_registers.timer.prev_and_result == true && and_result == false {
                    let (tima, tima_overflow) = emu.io_registers.timer.tima.overflowing_add(1);
                    emu.io_registers.timer.tima = tima;
                    if tima_overflow {
                        emu.io_registers.timer.cycles_since_tima_overflow = 4;
                    }
                }
            }
        }
    }

    #[inline]
    pub fn read_div(&self) -> u8 {
        return split_u16(self.div).1;
    }

    #[inline]
    pub fn write_div(&mut self) {
        self.div = 0;
    }

    #[inline]
    pub fn write_tima(&mut self, value: u8) {
        self.tima = value;
        self.cycles_since_tima_overflow = 0;
    }

    #[inline]
    pub fn read_tac(&self) -> u8 {
        return 0b1111_1000 | self.tac;
    }
}