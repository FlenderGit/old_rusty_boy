use std::{cell::RefCell, rc::Rc};

use crate::memory::Memory;

const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

const ADRESS_LCDC: u16 = 0xff40;
const ADRESS_LY: u16 = 0xff42;
const ADRESS_LX: u16 = 0xff43;
const ADRESS_LINE: u16 = 0xff44;
const ADRESS_WY: u16 = 0xff4a;
const ADRESS_WX: u16 = 0xff4b;

const TILE_DATA_BIT: u8 = 0x10;
const ADRESS_TILE_DATA_ON: u16 = 0x8000;
const ADRESS_TILE_DATA_OFF: u16 = 0x8800;
const TILE_MAP_BIT: u8 = 0x08;
const ADRESS_TILE_MAP_ON: u16 = 0x9800;
const ADRESS_TILE_MAP_OFF: u16 = 0x9c00;

#[derive(Debug)]
enum Mode {
    HBlank, // 204 cycles : termine le rendu d'une ligne horizontale et attend la prochaine ligne à dessiner
    VBlank, //4560 cycles (10 lignes * 456 cycles/ligne) : La PPU a fini de dessiner toutes les lignes, et il est temps d'envoyer l'image au framebuffer (ce qui provoque une interruption VBlank)
    OAM,    // 80 cycles : La PPU lit les sprites de la mémoire OAM
    DRAWING, // 172 cycles : dessine les pixels de la ligne actuelle
}

#[derive(Debug)]
pub struct GPU {
    memory: Rc<RefCell<Memory>>,
    mode: Mode,
    clock: u64,
    pub current_line: u8,
}

impl GPU {
    pub fn new(memory: Rc<RefCell<Memory>>) -> GPU {
        let current_line = memory.borrow().read_byte(ADRESS_LINE);

        GPU {
            memory,
            mode: Mode::HBlank,
            clock: 0,
            current_line,
        }
    }

    pub fn set_current_line(&mut self, line: u8) {
        self.current_line = line;
        self.memory.borrow_mut().write_byte(ADRESS_LINE, line);
    }

    pub fn step(&mut self, cycles: u64) {
        self.clock += cycles;
        match self.mode {
            Mode::HBlank => {
                if self.clock >= 204 {
                    self.set_current_line(self.current_line + 1);
                    self.clock -= 204;
                    if self.current_line == 144 {
                        self.mode = Mode::VBlank;
                    } else {
                        self.mode = Mode::OAM;
                    }
                }
            }
            Mode::VBlank => {
                if self.clock >= 456 {
                    self.clock -= 456;
                    if self.current_line > 153 {
                        self.mode = Mode::OAM;
                        self.set_current_line(0);
                        self.render_screen();
                    } else {
                        self.set_current_line(self.current_line + 1);
                    }
                }
            }
            Mode::OAM => {
                if self.clock >= 80 {
                    self.clock -= 80;
                    self.mode = Mode::DRAWING;
                }
            }
            Mode::DRAWING => {
                if self.clock >= 172 {
                    self.mode = Mode::HBlank;
                    self.clock -= 172;
                    self.render_scanline();
                }
            }
        }
    }

    pub fn render_scanline(&self) {
        let lcdc = self.memory.borrow().read_byte(ADRESS_LCDC);
        if lcdc & 0x80 == 0 {
            return;
        }

        let current_line = self.memory.borrow().read_byte(ADRESS_LY);
        let scroll_y = self.memory.borrow().read_byte(ADRESS_LY);
        let scroll_x = self.memory.borrow().read_byte(ADRESS_LX);
        let window_y = self.memory.borrow().read_byte(ADRESS_WY);
        let window_x = self.memory.borrow().read_byte(ADRESS_WX);

        let mut line = [0u8; 160];
        let mut line_offset = 0;

        // https://fms.komkon.org/GameBoy/Tech/Software.html --> FF40
        let bg_tile_data = if lcdc & TILE_DATA_BIT == 0 {
            ADRESS_TILE_DATA_ON
        } else {
            ADRESS_TILE_DATA_OFF
        };
        let bg_tile_map = if lcdc & TILE_MAP_BIT == 0 {
            ADRESS_TILE_MAP_ON
        } else {
            ADRESS_TILE_MAP_OFF
        };

        let mut tile_data = [0u8; 16];
        let mut tile_map = [0u8; 16];

        for x in 0..SCREEN_WIDTH {
            let tile_x = (x + scroll_x as usize) % 256;
            let tile_y = (current_line as usize + scroll_y as usize) % 256;

            let tile_map_x = tile_x / 8;
            let tile_map_y = tile_y / 8;

            let tile_map_offset = tile_map_y * 32 + tile_map_x;
            let tile_number = self
                .memory
                .borrow()
                .read_byte(bg_tile_map + tile_map_offset as u16);

            let tile_offset = if bg_tile_data == 0x8000 {
                tile_number as u16 * 16
            } else {
                ((tile_number as i8 as i16 + 128) * 16) as u16
            };

            self.memory
                .borrow()
                .read_block(bg_tile_data + tile_offset, &mut tile_data);

            let line_offset = tile_y % 8;
            let mut byte1 = tile_data[line_offset * 2];
            let mut byte2 = tile_data[line_offset * 2 + 1];

            for bit in 0..8 {
                let mask = 1 << (7 - bit);
                let mut color = if (byte2 & mask) != 0 { 1 } else { 0 } << 1;
                color |= if (byte1 & mask) != 0 { 1 } else { 0 };

                line[line_offset * 8 + bit] = color;
            }
        }

        // Draw the line
        for x in 0..SCREEN_WIDTH {
            let color = match line[x] {
                0 => [255, 255, 255],
                1 => [192, 192, 192],
                2 => [96, 96, 96],
                3 => [0, 0, 0],
                _ => panic!("Invalid color"),
            };

            // Write into vram
            let offset = (current_line as usize * SCREEN_WIDTH + x) * 3;
            self.memory.borrow_mut().write_byte(0x8000 + offset as u16, color[0]);

            let offset = (current_line as usize * SCREEN_WIDTH + x) * 3 + 1;
            self.memory.borrow_mut().write_byte(0x8000 + offset as u16, color[1]);

            let offset = (current_line as usize * SCREEN_WIDTH + x) * 3 + 2;
            self.memory.borrow_mut().write_byte(0x8000 + offset as u16, color[2]);
        }
    }
}
