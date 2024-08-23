use crate::cpu::CPU;
use crate::header::Header;
use crate::registers::Flag;

pub struct Gameboy {
    cpu: CPU,
    header: Header,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            cpu: CPU::new(),
            header: Header::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.cpu.memory.load_rom(&rom);
    }

    pub fn run(&mut self) {

        // 2F2A --> Intro
        // 6A6B --> Title screen
        // 650C
        // 2CF --> CFFB est remis (64D3)

        while self.cpu.registers.pc != 0x6a6b {
            self.cpu.step(false);
        }
        /* for _ in 0..160_000_000 {
            self.cpu.step(false);
        } */
        /* for _ in 0..1_000 {
            self.cpu.step(false);
            for _ in 0..500_000 {
                self.cpu.step(false);
            }
            //println!("Registers: {:?}", self.cpu.registers);
        } */
        for _ in 0..50_000 {
            self.cpu.step(true);
        }
        for _ in 0..15 {
            self.cpu.step_debug();
        }

        println!("0x9820: {:#04x}", self.cpu.memory.read(0x9820));
    }
}
