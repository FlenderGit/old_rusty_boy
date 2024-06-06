
use std::cell::RefCell;
use std::rc::Rc;

use crate::gpu::GPU;
use crate::header::Header;
use crate::memory::Memory;
use crate::cpu::CPU;

pub struct Game {
    pub header: Header,
    cpu: CPU,
    gpu: GPU,
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

        //let memory = Memory::new();
        let memory = Rc::new(RefCell::new(Memory::new()));

        memory.borrow_mut().load_rom(&rom);

        Game {
            header: Header::new(&rom),
            cpu: CPU::new(Rc::clone(&memory)),
            gpu: GPU::new(Rc::clone(&memory)),
        }
    }

    pub fn run(&mut self) {
        println!("Running game: {}", self.header.get_title());
        
        /* // TO DI
        for _ in 0..12_338 {
            self.cpu.step(false);
            self.gpu.step();
        }
        */

        while self.cpu.memory.borrow().pc != 0x0180 {
            self.cpu.step(false);
            self.gpu.step();
        }

        // 0x02c4 -> good

        // 02ca

        // Inside all calls 0x02c4 -> 0x29a6
        // 0x29e0 -> prb (b, c, f)
        // 0x02c7 call -> load tiles

        // 0x02c7 -> Normaly load tiles, not working

        
        for _ in 0..9 {
            self.cpu.step(true);
            println!("Registers: {:?}", self.cpu.registers);
            self.gpu.step();
        }

        println!("VRAM (100 first bytes): {:?}", &self.cpu.memory.borrow().vram[0x26e..0x27e]);

        use std::io::Write;
        // Save vram in a file
        let mut file = std::fs::File::create("vram.bin").unwrap();
        file.write_all(&self.cpu.memory.borrow().vram).unwrap();

        


    }
}