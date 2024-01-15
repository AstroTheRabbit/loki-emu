use crate::byte_field;
use std::string::FromUtf8Error;

byte_field! {
    /// [pandocs](https://gbdev.io/pandocs/The_Cartridge_Header.html).
    #[derive(Debug)]
    pub CartridgeHeader;
    pub restart_vectors:   256,
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
        self[(address - 0x0100) as usize]
    }

    #[allow(unused_variables)]
    pub fn write(&self, address: u16, value: u8) {
        panic!("GB - Cannot write to the cartridge header ROM!");
    }

    pub fn load_from_file(file_path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(&file_path)?;
        let mut buffer = [0; 336];
        std::io::Read::read_exact(&mut file, &mut buffer)?;
        Ok(Self::from(buffer))
    }

    /// Returns the game's title.
    pub fn get_title(&self) -> Result<String, FromUtf8Error> {
        let array = self.title.iter().filter(|&c| *c != 0x00).copied().collect();
        String::from_utf8(array)
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
            _ => panic!("GB - Cartridge ROM size not recognised!"),
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
            _ => panic!("GB - Cartridge RAM size not recognised!"),
        }
    }
}
