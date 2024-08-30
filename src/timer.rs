pub struct Timer {
    div: u8,               // Divider register
    tima: u8,               // Counter
    tma: u8,                // Modulo
    // tac: u8,                // Control
    active: bool,           // TAC (2)
    timer_clock: u8,
    pub interrupt: u8,
    internal_clock: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            
            active: false,
            timer_clock: 64,
            interrupt: 0,

            internal_clock: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xff04 => self.div,
            0xff05 => self.tima,
            0xff06 => self.tma,
            0xff07 => {
                0xF8 |
                (self.active as u8) << 2 |
                match self.timer_clock {
                    64 => 0b00,
                    1 => 0b01,
                    4 => 0b10,
                    16 => 0b11,
                    _ => panic!("Invalid timer clock: {}", self.timer_clock),
                }
            }
            _ => panic!("Invalid timer read address: {:04x}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xff04 => self.div = 0,
            0xff05 => self.tima = value,
            0xff06 => self.tma = value,
            0xff07 => {
                self.active = value & 0b100 != 0;
                self.timer_clock = match value & 0b11 {
                    0b00 => 64,
                    0b01 => 1,
                    0b10 => 4,
                    0b11 => 16,
                    _ => panic!("Invalid timer clock: {:02x}", value),
                };
            }
            _ => panic!("Invalid timer write address: {:04x}", address),
        }
    }

    pub fn step(&mut self, cycles: u8) {
        
        self.internal_clock += cycles;

        while self.internal_clock >= self.timer_clock {
            self.div = self.div.wrapping_add(1);
            self.internal_clock -= self.timer_clock;
        }

        if !self.active { return; }

        let old_tima = self.tima;
        let new_tima = self.tima.wrapping_add(cycles);
        if new_tima < old_tima {
            self.tima = self.tma;
            self.interrupt = 0x04;
        }
    }
}