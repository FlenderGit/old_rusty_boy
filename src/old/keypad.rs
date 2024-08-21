#[derive(Debug)]
pub struct Keypad {
    row0: u8,
    row1: u8,
    data: u8,
    pub interrupt: u8,
}

#[derive(Copy, Clone)]
pub enum KeypadKey {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xFF,
            interrupt: 0,
        }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, value: u8) {
        self.data = (self.data & 0xCF) | (value & 0x30);
        self.update();
    }

    fn update(&mut self) {
        let old_values = self.data & 0xF;
        let mut new_values = 0xF;

        if self.data & 0x10 == 0x00 {
            new_values &= self.row0;
        }
        if self.data & 0x20 == 0x00 {
            new_values &= self.row1;
        }

        if old_values == 0xF && new_values != 0xF {
            self.interrupt |= 0x10;
        }

        self.data = (self.data & 0xF0) | new_values;
    }

    pub fn keydown(&mut self, key: KeypadKey) {
        match key {
            KeypadKey::Right =>  self.row0 &= !(1 << 0),
            KeypadKey::Left =>   self.row0 &= !(1 << 1),
            KeypadKey::Up =>     self.row0 &= !(1 << 2),
            KeypadKey::Down =>   self.row0 &= !(1 << 3),
            KeypadKey::A =>      self.row1 &= !(1 << 0),
            KeypadKey::B =>      self.row1 &= !(1 << 1),
            KeypadKey::Select => self.row1 &= !(1 << 2),
            KeypadKey::Start =>  self.row1 &= !(1 << 3),
        }
        self.update();
    }

    pub fn keyup(&mut self, key: KeypadKey) {
        match key {
            KeypadKey::Right =>  self.row0 |= 1 << 0,
            KeypadKey::Left =>   self.row0 |= 1 << 1,
            KeypadKey::Up =>     self.row0 |= 1 << 2,
            KeypadKey::Down =>   self.row0 |= 1 << 3,
            KeypadKey::A =>      self.row1 |= 1 << 0,
            KeypadKey::B =>      self.row1 |= 1 << 1,
            KeypadKey::Select => self.row1 |= 1 << 2,
            KeypadKey::Start =>  self.row1 |= 1 << 3,
        }
        self.update();
    }
}