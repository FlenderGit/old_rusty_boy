use std::cmp::max;

use super::{get_number_ram_banks, get_number_rom_banks, MBC};

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,

    rom_banks_number: usize,
    ram_banks_number: usize,

    rom_bank: usize,
    ram_bank: usize,
    ram_enabled: bool,
    mode: u8,

    has_battery: bool,
}

impl MBC1 {
    pub fn new(rom: &Vec<u8>) -> Self {
        let (has_battery, ram_size) = match rom[0x0149] {
            0x01 => (false, 0),
            0x02 => (false, get_number_ram_banks(rom[0x0149])),
            0x03 => (true, get_number_ram_banks(rom[0x0149])),
            _ => panic!("Invalid MBC1 RAM size: {:02x}", rom[0x0149]),
        };

        MBC1 {
            rom: rom.to_vec(),
            ram: std::iter::repeat(0).take(ram_size * 0x2000).collect(),
            rom_bank: 1,
            ram_bank: 0,

            ram_enabled: false,
            mode: 0,

            rom_banks_number: get_number_rom_banks(rom[0x0148]),
            ram_banks_number: ram_size,

            has_battery: has_battery,
        }
    }
}

impl MBC for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        let bank = if address < 0x4000 {
            if self.mode == 0 {
                0
            } else {
                self.rom_bank & 0xE0
            }
        } else {
            self.rom_bank
        };

        let offset = address as usize & 0x3FFF;
        self.rom.get(bank * 0x4000 + offset).copied().unwrap_or(0xff)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1fff => self.ram_enabled = value & 0xf == 0xa, // Value with 0xa on the lowest but enable the RAM. Else disable.
            0x2000..=0x3fff => {
                let rom_bank = max(1, value & 0x1f) as usize; // Bank is selected using 5 lower bits. 0x00 -> 0x01
                self.rom_bank = ((self.rom_bank & 0x60) | rom_bank) % self.rom_banks_number;
            }
            0x4000..=0x5fff => {
                self.ram_bank = value as usize & 0x03;      // Redo : https://gbdev.io/pandocs/MBC1.html#40005fff--ram-bank-number--or--upper-bits-of-rom-bank-number-write-only
            }
            0x6000..=0x7fff => { self.mode = value & 0x01; }
            _ => {
                panic!("Attempted to write value on {:04x}", address);
            }
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled { return 0xff; }
        let bank = if self.mode == 0 { 0 } else { self.ram_bank };
        let offset = (bank * 0x2000 | address as usize) & 0x1FFF;
        self.ram[offset]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled { return }
        let rambank = if self.mode == 1 { self.ram_bank } else { 0 };
        let address = (rambank * 0x2000) | ((address & 0x1FFF) as usize);
        if address < self.ram.len() {
            self.ram[address] = value;
        }
    }

    fn has_battery(&self) -> bool { false }

    fn info(&self) -> String {
        format!(
            "MBC1: {:02x}, {:02x}, {}, {}",
            self.rom_bank, self.ram_bank, self.ram_enabled, self.mode
        )
    }
}
