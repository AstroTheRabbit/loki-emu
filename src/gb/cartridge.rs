use crate::byte_field;
use std::fs;

byte_field! {
    /// [pandocs](https://gbdev.io/pandocs/The_Cartridge_Header.html).
    #[derive(Debug)]
    pub CartridgeHeader;
    pub entry_point:       4,
    pub nintendo_logo:     48,
    pub title:             16,
    pub licensee_code:     2,
    pub sgb_flag:          1,
    pub cartridge_type:    1,
    pub rom_size:          1,
    pub ram_size:          1,
    pub destination_code:  1,
    pub old_licensee_code: 1,
    pub version_number:    1,
    pub header_checksum:   1,
    pub global_checksum:   2,
}

impl CartridgeHeader {
    pub fn read(&self, address: u16) -> u8 {
        return self[(address - 0x0100) as usize];
    }

    pub fn write(&self, address: u16, value: u8) {
        unimplemented!();
    }

    pub fn load_from_file(file_path: &str) -> std::io::Result<Self> {
        let f = fs::read(file_path)?;

        let mut s = Self::from([0; 80]);
        for i in 0..80 {
            s[i] = f[i + 0x0100]
        }

        return Ok(s);
    }

    /// Returns the cartridge's ROM size in bytes.
    pub fn get_rom_size(&self) -> usize {
        match self.rom_size[0] {
            0x00 => 32000,
            0x01 => 64000,
            0x02 => 128000,
            0x03 => 256000,
            0x04 => 512000,
            0x05 => 1024000,
            0x06 => 2048000,
            0x07 => 4096000,
            _ => panic!("Cartridge: ROM size not recognised!"),
        }
    }

    /// Returns the cartridge's RAM size in bytes.
    pub fn get_ram_size(&self) -> usize {
        match self.ram_size[0] {
            0x00 => 0,
            0x01 => 2000,
            0x02 => 8000,
            0x03 => 32000,
            0x04 => 128000,
            0x05 => 64000,
            _ => panic!("Cartridge: RAM size not recognised!"),
        }
    }
}
