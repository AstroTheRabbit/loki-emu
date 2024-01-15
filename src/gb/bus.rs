use crate::byte_field;

use super::cartridge::CartridgeHeader;
use super::graphics::{OAM, VRAM};

static mut BOOT_ROM: [u8; 256] = *include_bytes!("../../roms/GB Boot ROM.bin");

#[derive(Debug)]
pub struct Bus {
    pub cartridge_header: CartridgeHeader,
    pub vram: VRAM,
    pub wram: WRAM,
    pub oam: OAM,
    pub io_registers: IORegisters,
    pub hram: HRAM,
    pub ie_register: u8,
}

byte_field! {
    /// General purpose work RAM.
    #[derive(Debug)]
    pub WRAM;
    pub ram: 8192,
}

byte_field! {
    /// Input / output registers for player controls.
    #[derive(Debug)]
    pub IORegisters;
    pub registers: 128,
}

byte_field! {
    /// High RAM, mostly the same as WRAM.
    #[derive(Debug)]
    pub HRAM;
    pub ram: 127,
}

impl Bus {
    fn get_mut(&mut self, address: u16) -> Option<&mut u8> {
        match address {
            0x0000..=0x00FF => if self.read(0xFF50) == 0x00 {
                // ? Boot ROM is still mapped.
                unsafe { Some(&mut BOOT_ROM[address as usize]) }
            } else {
                Some(&mut self.cartridge_header[address as usize])
            },
            0x0100..=0x014F => Some(&mut self.cartridge_header[address as usize]),
            0x0150..=0x3FFF => todo!("GB - Cartridge ROM"),
            0x4000..=0x7FFF => todo!("GB - Swappable ROM"),
            0x8000..=0x9FFF => Some(&mut self.vram[address as usize - 0x8000]),
            0xA000..=0xBFFF => todo!("GB - Swappable RAM"),
            0xC000..=0xDFFF => Some(&mut self.wram[address as usize - 0xC000]),
            0xE000..=0xFDFF => Some(&mut self.wram[address as usize - 0xE000]),
            0xFE00..=0xFE9F => Some(&mut self.oam[address as usize - 0xFE00]),
            0xFEA0..=0xFEFF => None, // ? unimplemented!("GB - 0xFEA0..=0xFEFF not usable!")
            0xFF00..=0xFF7F => Some(&mut self.io_registers[address as usize - 0xFF00]),
            0xFF80..=0xFFFE => Some(&mut self.hram[address as usize - 0xFF80]),
            0xFFFF => Some(&mut self.ie_register),
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match self.get_mut(address) {
            Some(res) => *res,
            None => 0x00,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if let Some(res) = self.get_mut(address) {
            *res = value;
        }
    }
}
