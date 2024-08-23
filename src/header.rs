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
    pub fn new() -> Header {
        Header {
            title: String::new(),
            manufacturer_code: String::new(),
            cgb_flag: 0,
            new_licensee_code: 0,
            sgb_flag: 0,
            cartridge_type: 0,
            rom_size: 0,
            ram_size: 0,
            destination_code: 0,
            old_licensee_code: 0,
            mask_rom_version_number: 0,
            header_checksum: 0,
            global_checksum: 0
        }
    }


    
}