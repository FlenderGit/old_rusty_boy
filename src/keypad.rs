const ROW0_FLAG: u8 = 0x10;
const ROW1_FLAG: u8 = 0x20;

enum Row {
    Row0,
    Row1,
}

pub enum Key {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

pub struct Keypad {
    data: u8,
    row0: u8,
    row1: u8,
    pub interrupt: u8,
}

impl Keypad {
    
    pub fn new() -> Keypad {
        Keypad {
            data: 0xFF,
            row0: 0x0F,
            row1: 0x0F,
            interrupt: 0x00,
        }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, value: u8) {
        let mask = ROW0_FLAG | ROW1_FLAG;
        self.data = (self.data & !mask) | (value & mask);
        self.update();
    }

    fn update(&mut self) {
        let old = self.data & 0x0F;
        let mut new = 0x0F;

        new &= if self.row0 & ROW0_FLAG == 0 { self.row0 } else { 0x0F };
        new &= if self.row0 & ROW1_FLAG == 0 { self.row1 } else { 0x0F };
        self.interrupt = if old & new != 0 { 0x10 } else { 0x00 };

        self.data = (self.data & 0xF0) | new;
    }


    pub fn press(&mut self, key: Key) {
        match key {
            Key::A => self.row0 &= 0b1110,
            Key::B => self.row0 &= 0b1101,
            Key::Select => self.row0 &= 0b1011,
            Key::Start => self.row0 &= 0b0111,
            Key::Right => self.row1 &= 0b1110,
            Key::Left => self.row1 &= 0b1101,
            Key::Up => self.row1 &= 0b1011,
            Key::Down => self.row1 &= 0b0111,
        }
    }

    pub fn release(&mut self, key: Key) {
        match key {
            Key::A => self.row0 |= 0b0001,
            Key::B => self.row0 |= 0b0010,
            Key::Select => self.row0 |= 0b0100,
            Key::Start => self.row0 |= 0b1000,
            Key::Right => self.row1 |= 0b0001,
            Key::Left => self.row1 |= 0b0010,
            Key::Up => self.row1 |= 0b0100,
            Key::Down => self.row1 |= 0b1000,
        }
    }
}

