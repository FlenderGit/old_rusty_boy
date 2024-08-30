use std::ffi::CStr;

pub struct Header {
    title: String,
    manufacturer_code: String,
    cgb_flag: u8,
    new_licensee_code: u8,
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: u16
}



impl Header {
    pub fn load_rom(header: &[u8]) -> Self {
        Header {
            title: CStr::from_bytes_until_nul(&header[0x0134..0x0143])
                .unwrap().to_str()
                .unwrap().to_string(),
            manufacturer_code: CStr::from_bytes_until_nul(&header[0x013F..0x0143])
                .unwrap().to_str()
                .unwrap().to_string(),
            cgb_flag: header[0x0143],
            new_licensee_code: header[0x0144],
            sgb_flag: header[0x0146],
            cartridge_type: header[0x0147],
            rom_size: header[0x0148],
            ram_size: header[0x0149],
            destination_code: header[0x014A],
            old_licensee_code: header[0x014B],
            mask_rom_version_number: header[0x014C],
            header_checksum: header[0x014D],
            global_checksum: (header[0x014E] as u16) << 8 | header[0x014F] as u16
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn manufacturer_code(&self) -> &str {
        &self.manufacturer_code
    }

    pub fn cgb_flag(&self) -> u8 {
        self.cgb_flag
    }


    
}


#[cfg(test)]
mod tests {
    use crate::gameboy::Gameboy;

    #[test]
    fn test_header() {
        let mut gb = Gameboy::new();
        gb.load_rom_from_filename("roms/tetris.gb");

        let header = gb.header();

        assert_eq!(header.title(), "TETRIS");
        assert_eq!(header.manufacturer_code(), "");
        assert_eq!(header.cgb_flag(), 0x00);
    }
}
