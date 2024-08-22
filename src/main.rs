use gb_emulator::gameboy::Gameboy;
use log::warn;

fn main() {

    env_logger::init();

    let mut game = Gameboy::new();
    game.load_rom(std::fs::read("roms/tetris.gb").unwrap());
    game.run();
    
}
