use std::{cell::RefCell, rc::Rc};

use crate::memory::Memory;

const ADRESS_LCDC: u16 = 0xff40;
const ADRESS_LY: u16 = 0xff42;
const ADRESS_LX: u16 = 0xff43;
const ADRESS_LINE: u16 = 0xff44;

#[derive(Debug)]
enum Mode {
    HBlank,
    VBlank,
    OAM,
    VRAM,
}

#[derive(Debug)]
pub struct GPU {
    memory: Rc<RefCell<Memory>>,
    mode: Mode,
    tick: u64,
    pub current_line: u8,
}

impl GPU {
    pub fn new(memory: Rc<RefCell<Memory>>) -> GPU {

        let current_line = memory.borrow().read_byte(ADRESS_LINE);

        GPU {
            memory,
            mode: Mode::HBlank,
            tick: 0,
            current_line,
        }
    }

    pub fn step(&mut self) {
        
        // Ticks
        self.tick += 1;
        

        match self.mode {

            Mode::HBlank => {
                if self.tick >= 204 {
                    self.current_line += 1;
                    if self.current_line == 143 {
                        self.mode = Mode::VBlank;
                    } else {
                        self.mode = Mode::OAM;
                    }
                    self.memory.borrow_mut().write_byte(ADRESS_LINE, self.current_line);
                    self.tick -= 204;
                }
            }

            Mode::VBlank => {
                if self.tick >= 456 {
                    self.current_line += 1;
                    if self.current_line > 153 {
                        self.mode = Mode::OAM;
                        self.current_line = 0;
                    }
                    self.memory.borrow_mut().write_byte(ADRESS_LINE, self.current_line);
                    self.tick -= 456;
                }
            },

            Mode::OAM => {
                if self.tick >= 80 {
                    self.mode = Mode::VRAM;
                    self.tick -= 80;
                }
            },

            Mode::VRAM => {
                if self.tick >= 172 {
                    self.mode = Mode::HBlank;
                    self.tick -= 172;
                }
            }

        }


    }
}