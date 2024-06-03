
use crate::header::Header;

pub struct Game {
    pub header: Header,
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
        }
    }
}