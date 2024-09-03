use mbc1::MBC1;
use no_mbc::NoMBC;



mod no_mbc;
mod mbc1;

pub trait MBC {
    fn read_rom(&self , address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self , address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn has_battery(&self) -> bool;
    fn info(&self) -> String;
}

pub fn from_rom(rom: &Vec<u8>) -> Box<dyn MBC> {
    if rom.len() <= 0x0150 {
        panic!("ROM is too small to have a MBC");
    }
    
    match rom[0x147] {
        0x00 => Box::new(NoMBC::new(rom)),
        0x01 ..= 0x03 => Box::new(MBC1::new(rom)),
        _ => panic!("Unsupported MBC: {:02x}", rom[0x147]),
    }
}

fn get_number_rom_banks(value: u8) -> usize {
    match value {
        0x00 => 2,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => panic!("Invalid MBC1 bank size: {:02x}", value),
    }
}

fn get_number_ram_banks(value: u8) -> usize {
    match value {
        0x00 => 0,
        0x01 => 1,
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => panic!("Invalid MBC1 RAM bank size: {:02x}", value),
    }
}