use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::gpu::GPU;
use crate::header::Header;
use crate::memory::Memory;

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

        println!("0x02f0 {:#04x}", self.cpu.memory.borrow().read_byte(0x02f0));
        // TO DI
        let mut step = 0;
        /*
        for _ in 0..40_155 {
            self.cpu.step(false);
            self.gpu.step();
        }
        */

        // -> 0x0272 == GOOD
        // Registers: A: 0x00 F: 0xc0 B: 0x00 C: 0x10 D: 0x00 E: 0xd8 H: 0xcf L: 0xff SP: 0xcfff PC: 0x26b Ticks: 600048

        while self.cpu.memory.borrow().pc != 0x02cd {
            self.cpu.step(false);
            self.gpu.step(1);    
            step += 1;
        }

        // 0x02c4 -> good

        // 02ca

        // Inside all calls 0x02c4 -> 0x29a6
        // 0x29e0 -> prb (b, c, f)
        // 0x02c7 call -> load tiles

        for _ in 0..100 {
            self.cpu.step(false);
            self.gpu.step();
        }

        for _ in 0..15 {
            self.cpu.step(true);
            println!("Registers: {}", self.cpu);
            self.gpu.step();
        }

        // T0 : 0x02C2 :  Good
        // PRB : 0x02cd : 0xff80 must be 0x00 not 0xff
        println!("0xff44: {:x}", self.cpu.memory.borrow().read_byte(0xff44));
        println!("0xff80: {:x}", self.cpu.memory.borrow().read_byte(0xff80));
        println!("0xff85: {:x}", self.cpu.memory.borrow().read_byte(0xff85));

        /*
        use std::io::Write;
        // Save vram in a file
        let mut file = std::fs::File::create("vram.bin").unwrap();
        file.write_all(&self.cpu.memory.borrow().vram).unwrap();
        */
    }
}
