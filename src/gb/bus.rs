use crate::byte_field;

use super::cartridge::CartridgeHeader;

#[derive(Debug)]
pub struct Bus {
    pub boot_rom: BootROM,
    pub cartridge_header: CartridgeHeader,
}

byte_field! {
    /// ROM run when the Game Boy is powered on.
    BootROM;
    rom: 256,
}

impl Bus {
    pub fn read(&self, address: u16) -> u8 {
        if address < 0x100 {
            // Boot ROM
            return self.boot_rom[address as usize];
        } else if address < 0x8000 {
            // Cartride ROM
            return self.cartridge_header.read(address);
        }

        todo!()
    }

    pub fn write(&mut self, address: u16, value: u8) {
        // Cartride ROM
        if address < 0x8000 {
            return self.cartridge_header.write(address, value);
        }

        todo!()
    }
}