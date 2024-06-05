
use std::cell::RefCell;
use std::rc::Rc;

use crate::header::Header;
use crate::memory::Memory;
use crate::cpu_new::CPU;

pub struct Game {
    pub header: Header,
    //memory: Memory,
    cpu: CPU,
}

impl Game {
    pub fn new(file: &str) -> Game {
        let rom = std::fs::read(file);

        let rom = match rom {
            Ok(rom) => rom,
            Err(error) => {
                eprintln!("Error reading file: {}", error);
                std::process::exit(1);
            }
        };

        let memory = Memory::new();
        let memory_ref = Rc::new(RefCell::new(memory));
        
        memory_ref.borrow_mut().load_rom(&rom);

        Game {
            header: Header::new(&rom),
            //memory,
            cpu: CPU::new(memory_ref),
        }
    }

    pub fn run(&mut self) {
        println!("Running game: {}", self.header.get_title());
        
        for _ in 0..100_000 {
            self.cpu.step();
        }

        println!("Registers: {:?}", self.cpu.registers);

    }
}