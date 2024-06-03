use gb_emulator::game::Game;

#[test]
fn test_header() {
    
    let game = Game::new("roms/tetris.gb");

    let title = "ZELDA";
    let mut title_bytes = [0; 16];
    title_bytes[..title.len()].copy_from_slice(title.as_bytes());

    assert_eq!(game.header.title, title_bytes );
    assert_eq!(game.header.rom_size, 0x20);
    assert_eq!(game.header.ram_size, 0x02);

}