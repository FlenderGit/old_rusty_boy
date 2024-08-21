use crate::{gpu::GPU, keypad::Keypad};


const ROM_SIZE: usize = 0x8000;
const RAM_SIZE: usize = 0x2000;
const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7F;

pub struct Memory {
    rom: [u8; ROM_SIZE],
    gpu : GPU,
    pub keypad: Keypad,

    wram: [u8; WRAM_SIZE],
    wram_bank: u8,
    hram: [u8; HRAM_SIZE],
    ie: u8,
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
            ie: 0,
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
            0xff40..=0xFF4B => self.gpu.read(address),                      //LCD Control, Status, Position, Scrolling, and Palettes
            0xff4f => self.gpu.vram_bank,                                   // VRAM Bank
            0xff50 => 0,                                                    // Boot ROM disable
            0xff51..=0xFF55 => self.gpu.read(address),                      // VRAM DMA
            0xff68..=0xff6b => self.gpu.read(address),                      // Background/Object Palette Data
            0xff70 => self.wram_bank,                                       // WRAM Bank
            0xff80..=0xfffe => self.hram[address as usize & HRAM_SIZE],     // High RAM
            0xffff => self.ie,                                              // Interrupt Enable
            _ => { panic!("Unimplemented memory read at address: {:#06x}", address); }
        }
    }
    
}