use super::{
    cartridge::Cartridge,
    emu::GameboyEmulator,
    io::graphics::{OAM, VRAM},
};
#[cfg(not(test))]
use super::{io::io_registers::IORegisters, utils::join_u16};
use crate::byte_field;

#[cfg(not(test))]
static mut BOOT_ROM: [u8; 256] = *include_bytes!("../../roms/gb/GB Boot ROM.bin");

#[cfg(test)]
static mut TEST_RAM: [u8; 65536] = [0x00; 65536];

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
    pub fn read(
        #[allow(unused_variables)] emu: &mut GameboyEmulator,
        #[allow(unused_mut)] mut address: u16,
    ) -> u8 {
        #[cfg(test)]
        {
            return unsafe { TEST_RAM[address as usize] };
        }

        #[cfg(not(test))]
        {
            // ? DMA transfer in progress: R/W is not functional (except for HRAM = 0xFF80..=0xFFFE).
            if let Some(index) = emu.io_registers.graphics.DMA_transfer_progress {
                if index != 0x00 && !(0xFF80..=0xFFFE).contains(&address) {
                    // ? Reads during DMA transfer actually return the byte being transferred.
                    address = join_u16(index, emu.io_registers.graphics.DMA);
                }
            }
            match address {
                0x0000..=0x00FF => {
                    if Self::read(emu, 0xFF50) == 0x00 {
                        // ? Boot ROM is still mapped.
                        unsafe { BOOT_ROM[address as usize] }
                    } else {
                        emu.bus.cartridge[address as usize]
                    }
                }
                0x0100..=0x3FFF => emu.bus.cartridge[address as usize],
                0x4000..=0x7FFF => todo!("GB - Swappable ROM"),
                0x8000..=0x9FFF => emu.bus.vram[address as usize - 0x8000],
                0xA000..=0xBFFF => todo!("GB - Swappable RAM"),
                0xC000..=0xDFFF => emu.bus.wram[address as usize - 0xC000],
                0xE000..=0xFDFF => emu.bus.wram[address as usize - 0xE000],
                0xFE00..=0xFE9F => emu.bus.oam[address as usize - 0xFE00],
                0xFEA0..=0xFEFF => 0xFF, // ? unimplemented!("GB - 0xFEA0..=0xFEFF not usable!")
                0xFF00..=0xFF7F => IORegisters::read(emu, address as usize - 0xFF00),
                0xFF80..=0xFFFE => emu.bus.hram[address as usize - 0xFF80],
                0xFFFF => IORegisters::read(emu, address as usize - 0xFF00),
            }
        }
    }

    pub fn write(#[allow(unused_variables)] emu: &mut GameboyEmulator, address: u16, value: u8) {
        #[cfg(test)]
        {
            return unsafe {
                TEST_RAM[address as usize] = value;
            };
        }

        #[cfg(not(test))]
        {
            // ? DMA transfer in progress: R/W is not functional (except for HRAM = 0xFF80..=0xFFFE).
            if emu
                .io_registers
                .graphics
                .DMA_transfer_progress
                .is_some_and(|i| i != 0x00)
                && !(0xFF80..=0xFFFE).contains(&address)
            {
                return;
            }

            match address {
                0x0000..=0x00FF => {}
                0x0100..=0x3FFF => emu.bus.cartridge[address as usize - 0x0100] = value,
                0x4000..=0x7FFF => {}
                0x8000..=0x9FFF => emu.bus.vram[address as usize - 0x8000] = value,
                0xA000..=0xBFFF => todo!("GB - Swappable RAM"),
                0xC000..=0xDFFF => emu.bus.wram[address as usize - 0xC000] = value,
                0xE000..=0xFDFF => emu.bus.wram[address as usize - 0xE000] = value,
                0xFE00..=0xFE9F => emu.bus.oam[address as usize - 0xFE00] = value,
                0xFEA0..=0xFEFF => {} // ? unimplemented!("GB - 0xFEA0..=0xFEFF not usable!")
                0xFF00..=0xFF7F => IORegisters::write(emu, address as usize - 0xFF00, value),
                0xFF80..=0xFFFE => emu.bus.hram[address as usize - 0xFF80] = value,
                0xFFFF => IORegisters::write(emu, address as usize - 0xFF00, value),
            }
        }
    }

    #[cfg(test)]
    pub fn reset_test_ram() -> [u8; 65536] {
        unsafe { std::mem::replace(&mut TEST_RAM, [0x00; 65536]) }
    }
}
