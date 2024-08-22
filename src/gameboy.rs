use crate::cpu::CPU;

pub struct Gameboy {
    cpu: CPU
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            cpu: CPU::new()
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.cpu.memory.load_rom(&rom);
    }

    pub fn run(&mut self) {
        while self.cpu.registers.pc != 0x2ca {
            self.cpu.step(false);
        }
        /* for _ in 0..80_000 {
            self.cpu.step();
        } */
        for _ in 0..15_000 {
            self.cpu.step(true);
        }
        for _ in 0..5 {
            self.cpu.step_debug();
        }

        println!("0x9820 = {:#04x}", self.cpu.memory.read(0x9820));
    }
}