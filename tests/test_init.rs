use rusty_boy::gameboy::Gameboy;

#[test]
fn test_header() {
    
    let gb = Gameboy::new();
    gb.load_rom("roms/tetris.gb");

    let title = "TETRIS";
    let mut title_bytes = [0; 16];
    title_bytes[..title.len()].copy_from_slice(title.as_bytes());

    assert_eq!(game.header.title, title_bytes );
    assert_eq!(game.header.rom_size, 0x02);
    assert_eq!(game.header.ram_size, 0x00);

}