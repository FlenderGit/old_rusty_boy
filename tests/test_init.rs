#[cfg(test)]
mod tests {
    use rusty_boy::gameboy::Gameboy;

    #[test]
    fn test_header() {
        let mut gb = Gameboy::new();
        gb.load_rom_from_filename("roms/tetris.gb");

        let header = gb.header();

        assert_eq!(header.title(), "TETRIS");
        assert_eq!(header.manufacturer_code(), "");
        assert_eq!(header.cgb_flag(), 0x00);

    }
}
