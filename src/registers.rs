
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Flag {
    Zero = 1 << 7,
    Sub = 1 << 6,
    HalfCarry = 1 << 5,
    Carry = 1 << 4,
    None = 0,
}


#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xb0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn hli(&mut self) -> u16 {
        let value = self.hl();
        self.set_hl(value.wrapping_add(1));
        value
    }

    pub fn hld(&mut self) -> u16 {
        let value = self.hl();
        self.set_hl(value.wrapping_sub(1));
        value
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0x00ff) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xff00) >> 8) as u8;
        self.e = (value & 0x00ff) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xff00) >> 8) as u8;
        self.l = (value & 0x00ff) as u8;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xff00) >> 8) as u8;
        self.f = (value & 0x00ff) as u8;
    }

    pub fn up_flag(&mut self, flag: Flag) {
        self.f |= flag as u8;
    }

    pub fn down_flag(&mut self, flag: Flag) {
        self.f &= !(flag as u8);
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        if value {
            self.up_flag(flag);
        } else {
            self.down_flag(flag);
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.f & (flag as u8) != 0
    }

}