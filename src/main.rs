use gb_emulator::game::Game;

fn main() {

    let mut game = Game::new("roms/tetris.gb");
    println!("{}", game.header.to_string());
    game.run();
    
}
