use rusty_boy::gameboy::Gameboy;

fn main() {

    env_logger::init();

    let mut game = Gameboy::new();
    game.load_rom(std::fs::read("roms/tetris.gb").unwrap());
    game.set_render_callback(|screen_data| {
        for y in 0..144 {
            for x in 0..160 {
                print!(
                    "{}",
                    match screen_data[y * 160 * 3 + x * 3] {
                        3 => "  ",
                        2 => "░░",
                        1 => "▒▒",
                        0 => "▓▓",
                        _ => "  ",
                    }
                );
            }
            println!();
        }
    });
    game.run_debug();
    
}
