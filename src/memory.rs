use log::{info, warn};

use crate::{gpu::GPU, keypad::Keypad};


const ROM_SIZE: usize = 0x8000;
const RAM_SIZE: usize = 0x2000;
const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7F;

pub struct Memory {
    pub rom: [u8; ROM_SIZE],
    pub gpu : GPU,
    pub keypad: Keypad,

    pub interrupt_flags: u8,
    pub interrupt_enable: u8,

    wram: [u8; WRAM_SIZE],
    wram_bank: u8,
    hram: [u8; HRAM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom: [0; ROM_SIZE],
            gpu: GPU::new(),
            keypad: Keypad::new(),

            wram: [0; WRAM_SIZE],
            wram_bank: 0,
            hram: [0; HRAM_SIZE],
            
            interrupt_flags: 0,
            interrupt_enable: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],                  // Rom -- TD Handle rom bank switching
            0x8000..=0x9FFF => self.gpu.read_vram(address - 0x8000), // VRAM
            0xA000..=0xBFFF => 0,                                           // External RAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000],        // Work RAM (WRAM) -- TD Handle WRAM bank switching
            0xE000..=0xFDFF => self.read(address - 0x2000),         // Echo RAM
            0xFE00..=0xFE9F => self.gpu.read_oam(address - 0xFE00), // OAM
            0xFF00 => self.keypad.read(),                                   // Keypad
            0xFF01..= 0xFF02 => { warn!("Read Serial I/0. NI"); 0 },       // Serial I/O
            0xff04..=0xff07 => { warn!("Read Timer I/0. NI"); 0 },          // Timer I/O
            0xff0f => self.interrupt_flags,                                 // Interrupt Flags
            0xff10..=0xff3f => { warn!("Read Sound I/0. NI"); 0 },                  // Sound I/O
            0xff40..=0xFF4B => self.gpu.read(address),                      //LCD Control, Status, Position, Scrolling, and Palettes
            0xff4f => self.gpu.vram_bank,                                   // VRAM Bank
            0xff50 => 0,                                                    // Boot ROM disable
            0xff51..=0xFF55 => self.gpu.read(address),                      // VRAM DMA
            0xff68..=0xff6b => self.gpu.read(address),                      // Background/Object Palette Data
            0xff70 => self.wram_bank,                                       // WRAM Bank
            0xff80..=0xfffe => self.hram[address as usize & HRAM_SIZE],     // High RAM
            0xffff => self.interrupt_enable,                                // Interrupt Enable
            _ => { panic!("Unimplemented memory read at address: {:#06x}", address); }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => { warn!("Attempted to write to ROM at address: {:#06x}", address); }, // Rom -- TD Handle rom bank switching
            0x8000..=0x9FFF => self.gpu.write_vram(address - 0x8000, value), // VRAM
            0xA000..=0xBFFF => (),                                           // External RAM
            0xC000..=0xDFFF => self.wram[address as usize - 0xC000] = value, // Work RAM (WRAM) -- TD Handle WRAM bank switching
            0xE000..=0xFDFF => self.write(address - 0x2000, value),         // Echo RAM
            0xFE00..=0xFE9F => {
                // println!("Attempted to write to OAM at address: {:#06x} value: {:#04x}", address, value);
                self.gpu.write_oam(address - 0xFE00, value); // OAM
            },
            0xfea0..=0xfeff =>  (), // Unusable
            0xFF00 => self.keypad.write(value),                             // Keypad
            0xFF01..= 0xFF02 => { warn!("Write in serial I/0. NI") },       // Serial I/O
            0xff04..=0xff07 => { warn!("Write in Timer I/0. NI") },          // Timer I/O
            0xff0f => self.interrupt_flags = value,                         // Interrupt Flags
            0xff10..=0xff3f => { warn!("Write Sound I/0. NI") },                  // Sound I/O
            0xff46 => { 
                self.dma_transfer(value);
            }                         // OAM DMA
            0xff40..=0xFF4B => self.gpu.write(address, value),               //LCD Control, Status, Position, Scrolling, and Palettes
            0xff4f => self.gpu.vram_bank = value,                           // VRAM Bank
            0xff50 => (),                                                   // Boot ROM disable
            0xff51..=0xFF55 => self.gpu.write(address, value),               // VRAM DMA
            0xff68..=0xff6b => self.gpu.write(address, value),               // Background/Object Palette Data
            0xff70 => self.wram_bank = value,                               // WRAM Bank
            0xff80..=0xfffe => self.hram[address as usize & HRAM_SIZE] = value, // High RAM
            0xffff => self.interrupt_enable = value,                        // Interrupt Enable
            _ => { info!("Unimplemented memory write at address: {:#06x}", address); }
        }
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }


    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..ROM_SIZE {
            self.rom[i] = data[i];
        }
    }

    pub fn dma_transfer(&mut self, address: u8) {
        let start = address as u16 * 0x100;
        for i in 0..0xA0 {
            self.write(0xFE00 + i, self.read(start + i));
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.read(address) as u16) | ((self.read(address + 1) as u16) << 8)
    }

    pub fn step(&mut self, cycles: u8, draw: bool) {

        self.interrupt_flags |= self.keypad.interrupt;
        self.keypad.interrupt = 0;

        self.gpu.step(cycles, draw);
        self.interrupt_flags |= self.gpu.interrupt;
        self.gpu.interrupt = 0;
    }
    
}