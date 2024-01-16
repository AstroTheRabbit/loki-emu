use crate::byte_field;
use super::cartridge::Cartridge;
use super::emu::GameboyEmulator;
use super::graphics::{OAM, VRAM};

static mut BOOT_ROM: [u8; 256] = *include_bytes!("../../roms/GB Boot ROM.bin");

#[derive(Debug)]
pub struct Bus {
    pub cartridge: Cartridge,
    pub vram: VRAM,
    pub wram: WRAM,
    pub oam: OAM,
    pub hram: HRAM,
}

byte_field! {
    /// General purpose work RAM.
    #[derive(Debug)]
    pub WRAM;
    pub ram: 8192,
}

byte_field! {
    /// High RAM, mostly the same as WRAM.
    #[derive(Debug)]
    pub HRAM;
    pub ram: 127,
}

impl Bus {
    pub fn read(emu: &mut GameboyEmulator, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF => {
                if Self::read(emu, 0xFF50) == 0x00 {
                    // ? Boot ROM is still mapped.
                    unsafe { BOOT_ROM[address as usize] }
                } else {
                    emu.bus.cartridge[address as usize]
                }
            }
            0x0100..=0x3FFF => emu.bus.cartridge[address as usize - 0x0100],
            0x4000..=0x7FFF => todo!("GB - Swappable ROM"),
            0x8000..=0x9FFF => emu.bus.vram[address as usize - 0x8000],
            0xA000..=0xBFFF => todo!("GB - Swappable RAM"),
            0xC000..=0xDFFF => emu.bus.wram[address as usize - 0xC000],
            0xE000..=0xFDFF => emu.bus.wram[address as usize - 0xE000],
            0xFE00..=0xFE9F => emu.bus.oam[address as usize - 0xFE00],
            0xFEA0..=0xFEFF => 0xFF, // ? unimplemented!("GB - 0xFEA0..=0xFEFF not usable!")
            0xFF00..=0xFF7F => emu.io_registers.read(address as usize - 0xFF00),
            0xFF80..=0xFFFE => emu.bus.hram[address as usize - 0xFF80],
            0xFFFF => emu.io_registers.read(address as usize - 0xFF00),
        }
    }

    pub fn write(emu: &mut GameboyEmulator, address: u16, value: u8) {
        match address {
            0x0000..=0x00FF => {},
            0x0100..=0x3FFF => emu.bus.cartridge[address as usize - 0x0100] = value,
            0x4000..=0x7FFF => {},
            0x8000..=0x9FFF => emu.bus.vram[address as usize - 0x8000] = value,
            0xA000..=0xBFFF => todo!("GB - Swappable RAM"),
            0xC000..=0xDFFF => emu.bus.wram[address as usize - 0xC000] = value,
            0xE000..=0xFDFF => emu.bus.wram[address as usize - 0xE000] = value,
            0xFE00..=0xFE9F => emu.bus.oam[address as usize - 0xFE00] = value,
            0xFEA0..=0xFEFF => {}, // ? unimplemented!("GB - 0xFEA0..=0xFEFF not usable!")
            0xFF00..=0xFF7F => emu.io_registers.write(address as usize - 0xFF00, value),
            0xFF80..=0xFFFE => emu.bus.hram[address as usize - 0xFF80] = value,
            0xFFFF => emu.io_registers.write(address as usize - 0xFF00, value),
        }
    }
}