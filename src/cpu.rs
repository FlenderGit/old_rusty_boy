
enum Flag {
    Zero = 1 << 7,
    Sub = 1 << 6,
    HalfCarry = 1 << 5,
    Carry = 1 << 4,
}

struct CPU {
    registers: Registers,
    bus: [u8; 0xffff],
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            bus: [0; 0xffff],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.bus[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.bus[address as usize] = value;
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low = (value & 0x00ff) as u8;
        let high = ((value & 0xff00) >> 8) as u8;
        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    pub fn fetch_word(&mut self) -> u16 {
        let word = self.read_word(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_byte();
        match opcode {

            // First bar
            // NOP
            0x00 => {},
            // LD BC, d16
            0x01 => { self.registers.set_bc(self.fetch_word()); },
            // LD (BC), A
            0x02 => { self.write_byte(self.registers.bc(), self.registers.a); },
            // INC BC
            0x03 => { self.registers.set_bc(self.registers.bc().wrapping_add(1)); },
            // INC B
            0x04 => { self.registers.b = self.inc(self.registers.b); },
            // DEC B
            0x05 => { self.registers.b = self.dec(self.registers.b); },
            // LD B, d8
            0x06 => { self.registers.b = self.fetch_byte(); },
            // RLCA
            0x07 => {
                let carry = self.registers.a & 0x80;
                self.registers.a = (self.registers.a << 1) | carry;
                self.registers.set_flag(Flag::Carry, carry);
                self.set_flag(Flag::Zero, 0);
                self.set_flag(Flag::HalfCarry, 0);
                self.set_flag(Flag::Sub, 0);
            },
            // LD (a16), SP
            0x08 => { self.write_word(self.fetch_word(), self.registers.sp); },
            // ADD HL, BC
            0x09 => { self.add2(self.registers.hl, self.registers.bc); },
            // LD A, (BC)
            0x0a => { self.registers.a = self.read_byte(self.registers.bc()); },
            // DEC BC
            0x0b => { self.registers.set_bc(self.registers.bc().wrapping_sub(1)); },
            // INC C
            0x0c => { self.registers.c = self.inc(self.registers.c); }
            // DEC C
            0x0d => { self.registers.c = self.dec(self.registers.c); }
            // LD C, d8
            0x0e => { self.registers.c = self.fetch_byte(); }
            // RRCA
            0x0f => {
                let carry = self.registers.a & 0x01;
                self.registers.a = (self.registers.a >> 1) | (carry << 7);
                self.registers.set_flag(Flag::Carry, carry);
                self.set_flag(Flag::Zero, 0);
                self.set_flag(Flag::HalfCarry, 0);
                self.set_flag(Flag::Sub, 0);
            },
            _ => panic!("Unimplemented opcode: {:#04x}", opcode),
        }
    }

    fn add2(&mut self, &mut destination: u16, value: u16) {
        let result = destination.wrapping_add(value);
        destination = result;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (destination & 0x0fff) + (value & 0x0fff) > 0x0fff);
        self.registers.set_flag(Flag::Carry, (destination as u32) + (value as u32) > 0xffff);
    }

    fn inc(v: u8) -> u8 {
        let result = v.wrapping_add(1);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (v & 0x0f) + 1 > 0x0f);
        result
    }

    fn dec(v: u8) -> u8 {
        let result = v.wrapping_sub(1);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, true);
        self.registers.set_flag(Flag::HalfCarry, (v & 0x0f) == 0);
        result
    }

}