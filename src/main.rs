use gb_emulator::game::Game;

fn main() {

    let game = Game::new("roms/tetris.gb");
    println!("{}", game.header.to_string());
    
}
