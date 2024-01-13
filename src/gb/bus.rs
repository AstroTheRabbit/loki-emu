use crate::byte_field;

use super::cartridge_header::CartridgeHeader;
use super::graphics::{VRAM, OAM};

static mut BOOT_ROM: [u8; 256] = *include_bytes!("../../roms/GB Boot ROM.bin");

#[derive(Debug)]
pub struct Bus {
    pub cartridge_header: CartridgeHeader,
    pub vram: VRAM,
    pub wram: WRAM,
    pub oam: OAM,
}

byte_field! {
    /// General purpose work RAM.
    #[derive(Debug)]
    pub WRAM;
    pub ram: 8192,
}

impl Bus {

    fn get_mut(&mut self, address: u16) -> &mut u8 {
        match address {
            0x0000..=0x00FF => match self.read(0xFF50) {
                // ? Boot ROM is still mapped.
                0x00 => unsafe { &mut BOOT_ROM[address as usize] }
                // ? Boot ROM is has been unmapped.
                _ => &mut self.cartridge_header[address as usize]
            },
            0x0100..=0x014F => &mut self.cartridge_header[address as usize],
            0x0150..=0x3FFF => todo!("GB - Cartridge ROM"),
            0x4000..=0x7FFF => todo!("GB - Swappable ROM"),
            0x8000..=0x9FFF => &mut self.vram[address as usize - 0x8000],
            0xA000..=0xBFFF => todo!("GB - Swappable RAM"),
            0xC000..=0xDFFF => &mut self.wram[address as usize - 0xC000],
            0xE000..=0xFDFF => todo!("GB - ECHO RAM"),
            0xFE00..=0xFE9F => &mut self.oam[address as usize - 0xFE00],
            0xFEA0..=0xFEFF => unimplemented!("GB - 0xFEA0..=0xFEFF not usable!"),
            0xFF00..=0xFF7F => todo!("GB - IO Registers"),
            0xFF80..=0xFFFE => todo!("GB - HRAM"),
            0xFFFF => todo!("GB - IE Register"),
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        return *self.get_mut(address);
    }

    pub fn write(&mut self, address: u16, value: u8) {
        *self.get_mut(address) = value;
    }
}