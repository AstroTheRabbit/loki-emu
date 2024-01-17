#![allow(non_snake_case)]

use std::io::Write;

use winit_input_helper::WinitInputHelper;

use crate::gb::emu::GameboyEmulator;

use super::{timer::TimerRegisters, joypad::JoypadRegisters, graphics::GraphicsRegisters};

#[derive(Debug)]
pub struct IORegisters {
    pub joypad: JoypadRegisters,
    pub serial: SerialRegisters,
    pub timer: TimerRegisters,
    pub graphics: GraphicsRegisters,
    pub interrupts: InterruptsRegisters,
    pub boot_rom_control: u8,
}

impl IORegisters {
    pub fn new() -> Self {
        Self {
            joypad: JoypadRegisters::new(),
            serial: SerialRegisters::new(),
            timer: TimerRegisters::new(),
            graphics: GraphicsRegisters::new(),
            interrupts: InterruptsRegisters::new(),
            boot_rom_control: 0x00,
        }
    }

    /// Updates timers and I/O as if 4 t-cycles have passed.
    pub fn update(emu: &mut GameboyEmulator, input: &mut WinitInputHelper) {
        emu.io_registers.joypad.update(input);
        TimerRegisters::update(emu);
    }

    pub fn read(emu: &mut GameboyEmulator, index: usize) -> u8 {
        match index {
            0x0000 => emu.io_registers.joypad.read(),
            0x0001 => todo!("GB - IO: Serial byte"),
            0x0002 => todo!("GB - IO: Serial control"),
            0x0003 => unimplemented!("GB - IO: Unmapped"),
            0x0004 => emu.io_registers.timer.read_DIV(),
            0x0005 => emu.io_registers.timer.TIMA,
            0x0006 => emu.io_registers.timer.TMA,
            0x0007 => emu.io_registers.timer.TAC | 0b1111_1000,
            0x0008..=0x000E => unimplemented!("GB - IO: Unmapped"),
            0x000F => emu.io_registers.interrupts.IF | 0b1110_0000,
            0x0010 => 0xFF, // todo!("GB - IO: Audio channel 1 sweep"),
            0x0011 => 0xFF, // todo!("GB - IO: Audio channel 1 sound length/wave duty"),
            0x0012 => 0xFF, // todo!("GB - IO: Audio channel 1 envelope"),
            0x0013 => 0xFF, // todo!("GB - IO: Audio channel 1 frequency"),
            0x0014 => 0xFF, // todo!("GB - IO: Audio channel 1 control"),
            0x0015 => unimplemented!("GB - IO: Unmapped"),
            0x0016 => 0xFF, // todo!("GB - IO: Audio channel 2 sound length/wave duty"),
            0x0017 => 0xFF, // todo!("GB - IO: Audio channel 2 envelope"),
            0x0018 => 0xFF, // todo!("GB - IO: Audio channel 2 frequency"),
            0x0019 => 0xFF, // todo!("GB - IO: Audio channel 2 control"),
            0x001A => 0xFF, // todo!("GB - IO: Audio channel 3 enable"),
            0x001B => 0xFF, // todo!("GB - IO: Audio channel 3 sound length"),
            0x001C => 0xFF, // todo!("GB - IO: Audio channel 3 volume"),
            0x001D => 0xFF, // todo!("GB - IO: Audio channel 3 frequency"),
            0x001E => 0xFF, // todo!("GB - IO: Audio channel 3 control"),
            0x001F => unimplemented!("GB - IO: Unmapped"),
            0x0020 => 0xFF, // todo!("GB - IO: Audio channel 4 sound length"),
            0x0021 => 0xFF, // todo!("GB - IO: Audio channel 4 volume"),
            0x0022 => 0xFF, // todo!("GB - IO: Audio channel 4 frequency"),
            0x0023 => 0xFF, // todo!("GB - IO: Audio channel 4 control"),
            0x0024 => 0xFF, // todo!("GB - IO: Audio output mapping"),
            0x0025 => 0xFF, // todo!("GB - IO: Audio channel mapping"),
            0x0026 => 0xFF, // todo!("GB - IO: Audio channel control"),
            0x0027..=0x002F => unimplemented!("GB - IO: Unmapped"),
            0x0030..=0x003F => todo!("GB - IO: Wave pattern"),
            0x0040 => emu.io_registers.graphics.LCDC,
            0x0041 => emu.io_registers.graphics.STAT | 0b1000_0000,
            0x0042 => emu.io_registers.graphics.SCY,
            0x0043 => emu.io_registers.graphics.SCX,
            0x0044 => emu.io_registers.graphics.LY,
            0x0045 => emu.io_registers.graphics.LYC,
            0x0046 => emu.io_registers.graphics.DMA,
            0x0047 => emu.io_registers.graphics.BGP,
            0x0048 => emu.io_registers.graphics.OBP0,
            0x0049 => emu.io_registers.graphics.OBP1,
            0x004A => emu.io_registers.graphics.WY,
            0x004B => emu.io_registers.graphics.WX,
            0x0050 => emu.io_registers.boot_rom_control,
            0x00FF => emu.io_registers.interrupts.IE | 0b1110_0000,
            _ => panic!("GB - IO: Index {:X} out of range!", index),
        }
    }

    pub fn write(emu: &mut GameboyEmulator, index: usize, value: u8) {
        match index {
            0x0000 => emu.io_registers.joypad.write(value),
            0x0001 => SerialRegisters::write_SB(emu, value),
            0x0002 => todo!("GB - IO: Serial control"),
            0x0003 => unimplemented!("GB - IO: Unmapped"),
            0x0004 => emu.io_registers.timer.write_DIV(),
            0x0005 => emu.io_registers.timer.write_TIMA(value),
            0x0006 => emu.io_registers.timer.TMA = value,
            0x0007 => emu.io_registers.timer.TAC = value,
            0x0008..=0x000E => unimplemented!("GB - IO: Unmapped"),
            0x000F => emu.io_registers.interrupts.IF = value,
            0x0010 => {} // todo!("GB - IO: Audio channel 1 sweep"),
            0x0011 => {} // todo!("GB - IO: Audio channel 1 sound length/wave duty"),
            0x0012 => {} // todo!("GB - IO: Audio channel 1 envelope"),
            0x0013 => {} // todo!("GB - IO: Audio channel 1 frequency"),
            0x0014 => {} // todo!("GB - IO: Audio channel 1 control"),
            0x0015 => unimplemented!("GB - IO: Unmapped"),
            0x0016 => {} // todo!("GB - IO: Audio channel 2 sound length/wave duty"),
            0x0017 => {} // todo!("GB - IO: Audio channel 2 envelope"),
            0x0018 => {} // todo!("GB - IO: Audio channel 2 frequency"),
            0x0019 => {} // todo!("GB - IO: Audio channel 2 control"),
            0x001A => {} // todo!("GB - IO: Audio channel 3 enable"),
            0x001B => {} // todo!("GB - IO: Audio channel 3 sound length"),
            0x001C => {} // todo!("GB - IO: Audio channel 3 volume"),
            0x001D => {} // todo!("GB - IO: Audio channel 3 frequency"),
            0x001E => {} // todo!("GB - IO: Audio channel 3 control"),
            0x001F => unimplemented!("GB - IO: Unmapped"),
            0x0020 => {} // todo!("GB - IO: Audio channel 4 sound length"),
            0x0021 => {} // todo!("GB - IO: Audio channel 4 volume"),
            0x0022 => {} // todo!("GB - IO: Audio channel 4 frequency"),
            0x0023 => {} // todo!("GB - IO: Audio channel 4 control"),
            0x0024 => {} // todo!("GB - IO: Audio output mapping"),
            0x0025 => {} // todo!("GB - IO: Audio channel mapping"),
            0x0026 => {} // todo!("GB - IO: Audio channel control"),
            0x0027..=0x002F => unimplemented!("GB - IO: Unmapped"),
            0x0030..=0x003F => todo!("GB - IO: Wave pattern"),
            0x0040 => emu.io_registers.graphics.LCDC = value,
            0x0041 => emu.io_registers.graphics.STAT = value,
            0x0042 => emu.io_registers.graphics.SCY = value,
            0x0043 => emu.io_registers.graphics.SCX = value,
            0x0044 => emu.io_registers.graphics.LY = value,
            0x0045 => emu.io_registers.graphics.LYC = value,
            0x0046 => GraphicsRegisters::write_DMA(emu, value),
            0x0047 => emu.io_registers.graphics.BGP = value,
            0x0048 => emu.io_registers.graphics.OBP0 = value,
            0x0049 => emu.io_registers.graphics.OBP1 = value,
            0x004A => emu.io_registers.graphics.WY = value,
            0x004B => emu.io_registers.graphics.WX = value,
            0x004C..=0x4F => unimplemented!("GB - IO: Unmapped"),
            0x0050 => emu.io_registers.boot_rom_control = value,
            0x0051..=0x00FE => unimplemented!("GB - IO: Unmapped"),
            0x00FF => emu.io_registers.interrupts.IE = value,
            _ => panic!("GB - IO: Index {:X} out of range!", index),
        }
    }
}

#[derive(Debug)]
pub struct InterruptsRegisters {
    /// `0xFF0F` - Interrupts asserted.
    pub IF: u8,
    /// `0xFFFF` - Interrupts enabled.
    pub IE: u8,
}

impl InterruptsRegisters {
    pub fn new() -> Self {
        Self { IF: 0x00, IE: 0x00 }
    }
}

#[derive(Debug)]
pub struct SerialRegisters {
    /// `0xFF01` - Serial byte.
    pub SB: u8,
    /// `0xFF02` - Serial control.
    pub SC: u8,
}

impl SerialRegisters {
    pub fn new() -> Self {
        Self { SB: 0x00, SC: 0x00 }
    }

    pub fn write_SB(emu: &mut GameboyEmulator, value: u8) {
        emu.io_registers.serial.SB = value;
        // ! Blargg tests output.
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).write(true).open("./blargg_output.txt") {
            f.write(&[value]).unwrap();
        }
    }
}
