use crate::keypad::Keypad;


#[derive(Debug)]
pub struct Memory {
    
    rom: [u8; 0x8000],

    pub vram: [u8; 0x2000],
    sram: [u8; 0x2000],
    wram: [u8; 0x2000],
    oam: [u8; 0x00a0],

    keypad: Keypad,
    io: [u8; 0x007f],
    hram: [u8; 0x007f],
    pub interrupt_enable: u8,

    pub pc: u16,
    pub sp: u16,

}


impl Memory {

    pub fn load_rom(&mut self, data: &Vec<u8>) {        
        // Clone 0x0000 -> 0x7fff
        for i in 0..0x8000 {
            self.rom[i] = data[i];
        }

        
    }

    pub fn new() -> Memory {
        Memory {
            
            rom: [0; 0x8000],
            vram: [0; 0x2000],
            sram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0x00a0],
            keypad: Keypad::new(),
            io: [0; 0x007f],
            hram: [0; 0x007f],
            interrupt_enable: 0,

            pc: 0x0100,
            sp: 0xfffe,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7fff => self.rom[address as usize],
            0x8000..=0x9fff => self.vram[(address - 0x8000) as usize],
            0xa000..=0xbfff => self.sram[(address - 0xa000) as usize],
            0xc000..=0xdfff => self.wram[(address - 0xc000) as usize],
            0xe000..=0xfdff => self.wram[(address - 0xe000) as usize],
            0xfe00..=0xfe9f => self.oam[(address - 0xfe00) as usize],
            0xfea0..=0xfeff => { panic!("Invalid memory read at address: {:x}", address) },
            0xff00 => self.keypad.read(),
            0xff01..=0xff7f => self.io[(address - 0xff01) as usize],
            0xff80..=0xfffe => self.hram[(address - 0xff80) as usize],
            0xffff => self.interrupt_enable,
            //_ => panic!("Invalid memory read at address: {:x}", address),
        }
    }

    pub fn read_short(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    pub fn fetch_short(&mut self) -> u16 {
        let short = self.read_short(self.pc);
        self.pc += 2;
        short
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7fff => self.rom[address as usize] = value,
            0x8000..=0x9fff => {
                self.vram[(address - 0x8000) as usize] = value;
            }
            0xa000..=0xbfff => self.sram[(address - 0xa000) as usize] = value,
            0xc000..=0xdfff => self.wram[(address - 0xc000) as usize] = value,
            0xe000..=0xfdff => self.wram[(address - 0xe000) as usize] = value,
            0xfe00..=0xfe9f => self.oam[(address - 0xfe00) as usize] = value,
            0xfea0..=0xfeff => {
                //println!("Invalid memory write at address: {:x}", address)
            },
            0xff00 => self.keypad.write(value),
            0xff01..=0xff7f => self.io[(address - 0xff01) as usize] = value,
            0xff80..=0xfffe => {
                self.hram[(address - 0xff80) as usize] = value;
                println!("Write to hram: {:x} -> {:x}", address, value);
            },
            0xffff => self.interrupt_enable = value,
            //_ => panic!("Invalid memory write at address: {:x}", address),
        }
    }

    pub fn write_short(&mut self, address: u16, value: u16) {
        let low = value as u8;
        let high = (value >> 8) as u8;
        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }

    pub fn write_block(&mut self, address: u16, data: &[u8]) {
        for i in 0..data.len() {
            self.write_byte(address + i as u16, data[i]);
        }
    }

    pub fn read_block(&self, address: u16, data: &mut [u8]) {
        for i in 0..data.len() {
            data[i] = self.read_byte(address + i as u16);
        }
    }

    pub fn reset(&mut self) {
        self.rom = [0; 0x8000];
        self.vram = [0; 0x2000];
        self.sram = [0; 0x2000];
        self.wram = [0; 0x2000];
        self.oam = [0; 0x00a0];
        self.io = [0; 0x007f];
        self.hram = [0; 0x007f];
        self.interrupt_enable = 0;

        self.pc = 0x0100;
        self.sp = 0xfffe;

        self.init();
    }

    fn init (&mut self) {
        self.reset();
        self.write_byte(0xFF10, 0x80);
        self.write_byte(0xFF11, 0xbf);
        self.write_byte(0xFF12, 0xf3);
        self.write_byte(0xFF14, 0xbf);
        self.write_byte(0xFF16, 0x3f);

        self.write_byte(0xFF19, 0xbf);
        self.write_byte(0xFF1A, 0x7f);
        self.write_byte(0xFF1B, 0xff);
        self.write_byte(0xFF1C, 0x9f);
        self.write_byte(0xFF1E, 0xff);
        self.write_byte(0xFF20, 0xff);
    }

    pub fn read_short_from_stack(&mut self) -> u16 {
        let value = self.read_short(self.sp);
        self.sp = self.sp.wrapping_add(2);
        value
    }

    pub fn write_short_to_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.write_short(self.sp, value);
    }

    pub fn pop_stack(&mut self) -> u16 {
        let value = self.read_short_from_stack();
        value
    }

}
