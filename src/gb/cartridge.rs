use crate::byte_field;

byte_field! {
    /// [pandocs](https://gbdev.io/pandocs/The_Cartridge_Header.html).
    CartridgeHeader;
    entry_point:       4,
    nintendo_logo:     48,
    title:             16,
    licensee_code:     2,
    sgb_flag:          1,
    cartridge_type:    1,
    rom_size:          1,
    ram_size:          1,
    destination_code:  1,
    old_licensee_code: 1,
    version_number:    1,
    header_checksum:   1,
    global_checksum:   2,
}

impl CartridgeHeader {
    pub fn read(&self, address: u16) -> u8 {
        self[address as usize]
    }

    pub fn write(&self, address: u16, value: u8) {
        todo!();
    }

    pub fn load_from_file(file_path: &str) -> std::io::Result<Self> {
        let f = std::fs::read(file_path)?;
        let mut s = Self::from([0; 80]);
        for i in 0..80 {
            s[i] = f[i + 0x0100]
        }
        Ok(s)
    }
}
