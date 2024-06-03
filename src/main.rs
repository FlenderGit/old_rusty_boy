use gb_emulator::rom::Header;

fn main() {
    
    // Read file and instance a new Header
    let rom = std::fs::read("roms/tetris.gb");

    // Check if the file was read
    let rom = match rom {
        Ok(rom) => rom,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            std::process::exit(1);
        }
    };

    let header = Header::new(&rom);

    // Print the header
    println!("{}", header.to_string());
}
