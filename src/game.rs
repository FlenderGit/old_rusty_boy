
use crate::header::Header;
use crate::cpu::CPU;

pub struct Game {
    pub header: Header,
    pub cpu: CPU,
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

        Game {
            header: Header::new(&rom),
            cpu: CPU::new(rom),
        }
    }

    pub fn run(&mut self) {
        // println!("Running game: {}", self.header.title.to_string());

        for _ in 0..10 {
            self.cpu.step();
        }


    }
}