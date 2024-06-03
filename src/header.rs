pub struct Header {

    pub entry_point: u16,
    pub nintendo_logo: [u8; 48],
    pub title: [u8; 16],    // Can contains the manufacturer code in new cartridges
    pub cgb_flag: u8,
    pub new_licensee_code: u8,
    pub sgb_flag: u8,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: u8,
    pub old_licensee_code: u8,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,

}

impl Header {

    pub fn new(rom: &[u8]) -> Header {

        let header = Header {
            entry_point: (rom[0x102] as u16) | ((rom[0x103] as u16) << 8),
            nintendo_logo: {
                let mut logo = [0; 48];
                logo.copy_from_slice(&rom[0x104..0x134]);
                logo
            },
            title: {
                let mut title = [0; 16];
                title.copy_from_slice(&rom[0x134..0x144]);
                title
            },
            cgb_flag: rom[0x143],
            new_licensee_code: rom[0x144],
            sgb_flag: rom[0x146],
            cartridge_type: rom[0x147],
            rom_size: Header::get_rom_size(rom[0x148]),
            ram_size: rom[0x149],
            destination_code: rom[0x14a],
            old_licensee_code: rom[0x14b],
            mask_rom_version_number: rom[0x14c],
            header_checksum: rom[0x14d],
            global_checksum: (rom[0x14e] as u16) | ((rom[0x14f] as u16) << 8),
        };

        if !header.is_valid(rom) {
            eprintln!("Invalid header checksum");
            std::process::exit(1);
        }

        let rom_size = header.rom_size as u64 * 16 * 1024;
        if rom_size as u64 != rom.len() as u64 {
            let msg = format!("ROM size in header ({}) does not match actual ROM size ({})", rom_size, rom.len());
            eprintln!("{}", msg);
            std::process::exit(1);
        }

        return header;
    }

    pub fn is_valid(&self, rom: &[u8]) -> bool {
        let mut sum = 0u8;
        for i in 0x134..0x14d {
            sum = sum.wrapping_sub(rom[i]).wrapping_sub(1);
        }
        sum == self.header_checksum
    }

    fn get_rom_size(v: u8) -> u8 {
        match v {
            0x00 => 2,
            0x01 => 4,
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            _ => 0,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Title: {}\nCartridge Type: {}\nROM Size: {}\nRAM Size: {}\n",
            String::from_utf8_lossy(&self.title).trim_end_matches(char::from(0)),
            self.cartridge_type,
            self.rom_size,
            self.ram_size,
        )
    }

}