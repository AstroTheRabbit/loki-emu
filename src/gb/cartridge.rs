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
            s[i] = f[i + 0x0104]
        }
        Ok(s)
    }

    pub fn get_num_rom_banks(&self) -> u16 {
        match self.rom_size[0] {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,

            // ? "Only listed in unofficial docs. No cartridges or ROM files using these sizes are known. As the other ROM sizes are all powers of 2, these are likely inaccurate. The source of these values is unknown."
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => panic!("Cartridge: ROM size not recognised!"),
        }
    }
}
