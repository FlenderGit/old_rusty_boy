use crate::{memory::Memory, registers::{Flag, Registers}};

pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
    ime: bool,
}

impl CPU {
    
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
            ime: false,
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.memory.read(address)
    }

    fn fetch_byte(&mut self) -> u8 {
        let value = self.memory.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        value
    }

    fn read_word(&mut self) -> u16 {
        self.memory.read_word(self.registers.pc)
    }

    fn fetch_word(&mut self) -> u16 {
        let value = self.read_word();
        self.registers.pc = self.registers.pc.wrapping_add(2);
        value
    }

    pub fn step(&mut self) -> u8 {

        if self.ime {
            self.handle_interrupts();
        }

        let ticks = self.call();
        self.memory.step(ticks);
        //self.memory.gpu.step(ticks, draw);
        ticks
    }

    fn handle_interrupts(&mut self) {
        let interrupt_flags = self.memory.interrupt_flags;
        let interrupt_enable = self.memory.interrupt_enable;
        let interrupt = interrupt_flags & interrupt_enable;

        if interrupt == 0 {
            return;
        }

        let interrupt_vector = match interrupt {
            0x01 => 0x40,
            0x02 => 0x48,
            0x04 => 0x50,
            0x08 => 0x58,
            0x10 => 0x60,
            _ => panic!("Invalid interrupt: {:#04x}, PC: {:#06x}", interrupt, self.registers.pc),
        };

        self.call_interrupt(interrupt_vector);
        self.memory.write(0xff0f, interrupt_flags & !interrupt);
    }

    fn call_interrupt(&mut self, address: u16) {
        let pc = self.registers.pc;
        self.push_stack(pc);
        self.registers.pc = address;
    }

    fn call(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => 4, // NOP
            0x01 => { let v = self.fetch_word(); self.registers.set_bc(v); 12 }, // LD BC, d16
            0x02 => { self.memory.write(self.registers.bc(), self.registers.a) ; 8 }, // LD (BC), A
            0x03 => { self.registers.set_bc(self.registers.bc().wrapping_add(1)); 8 }, // INC BC
            0x04 => { self.registers.b = self.reg_inc(self.registers.b); 4 }, // INC B
            0x05 => { self.registers.b = self.reg_dec(self.registers.b); 4 }, // DEC B
            0x06 => { self.registers.b = self.fetch_byte(); 8 }, // LD B, d8
            0x07 => { self.registers.a = self.rlc(self.registers.a); self.registers.set_flag(Flag::Zero, false); 4 }, // RLCA
            0x08 => { let addr = self.fetch_word(); self.memory.write_word(addr, self.registers.sp); 20 }, // LD (a16), SP
            0x09 => { self.add_hl(self.registers.bc()); 8 }, // ADD HL, BC
            0x0A => { self.registers.a = self.memory.read(self.registers.bc()); 8 }, // LD A, (BC)
            0x0B => { self.registers.set_bc(self.registers.bc().wrapping_sub(1)); 8 }, // DEC BC
            0x0C => { self.registers.c = self.reg_inc(self.registers.c); 4 }, // INC C
            0x0D => { self.registers.c = self.reg_dec(self.registers.c); 4 }, // DEC C
            0x0E => { self.registers.c = self.fetch_byte(); 8 }, // LD C, d8
            

            0x11 => { let v = self.fetch_word(); self.registers.set_de(v); 12 }, // LD DE, d16
            0x12 => { self.memory.write(self.registers.de(), self.registers.a); 8 }, // LD (DE), A
            0x13 => { self.registers.set_de(self.registers.de().wrapping_add(1)); 8 }, // INC DE
            0x14 => { self.registers.d = self.reg_inc(self.registers.d); 4 }, // INC D
            0x15 => { self.registers.d = self.reg_dec(self.registers.d); 4 }, // DEC D
            0x16 => { self.registers.d = self.fetch_byte(); 8 }, // LD D, d8

            0x18 => { self.jr(); 12 }, // JR r8
            0x19 => { self.add_hl(self.registers.de()); 8 }, // ADD HL, DE
            0x1A => { self.registers.a = self.memory.read(self.registers.de()); 8 }, // LD A, (DE)
            0x1B => { self.registers.set_de(self.registers.de().wrapping_sub(1)); 8 }, // DEC DE
            0x1C => { self.registers.e = self.reg_inc(self.registers.e); 4 }, // INC E
            0x1D => { self.registers.e = self.reg_dec(self.registers.e); 4 }, // DEC E
            0x1E => { self.registers.e = self.fetch_byte(); 8 }, // LD E, d8
            
            0x20 => { if !self.registers.get_flag(Flag::Zero) { self.jr(); 12 } else { self.registers.pc += 1; 8 } }, // JR NZ, r8
            0x21 => { let v = self.fetch_word(); self.registers.set_hl(v); 12 }, // LD HL, d16
            0x22 => { self.memory.write(self.registers.hli(), self.registers.a); 8 }, // LD (HL+), A
            0x23 => { self.registers.set_hl(self.registers.hl().wrapping_add(1)); 8 }, // INC HL
            0x24 => { self.registers.h = self.reg_inc(self.registers.h); 4 }, // INC H
            0x25 => { self.registers.h = self.reg_dec(self.registers.h); 4 }, // DEC H
            0x26 => { self.registers.h = self.fetch_byte(); 8 }, // LD H, d8
            0x27 => { self.daa(); 4 }, // DAA
            0x28 => { if self.registers.get_flag(Flag::Zero) { self.jr(); 12 } else { self.registers.pc += 1; 8 } }, // JR Z, r8
            0x29 => { self.add_hl(self.registers.hl()); 8 }, // ADD HL, HL
            0x2A => { self.registers.a = self.memory.read(self.registers.hli()); 8 }, // LD A, (HL+)
            0x2B => { self.registers.set_hl(self.registers.hl().wrapping_sub(1)); 8 }, // DEC HL
            0x2C => { self.registers.l = self.reg_inc(self.registers.l); 4 }, // INC L
            0x2D => { self.registers.l = self.reg_dec(self.registers.l); 4 }, // DEC L
            0x2E => { self.registers.l = self.fetch_byte(); 8 }, // LD L, d8
            0x2f => { self.registers.a = !self.registers.a; self.registers.set_flag(Flag::Sub, true); self.registers.set_flag(Flag::HalfCarry, true); 4 }, // CPL
            0x30 => { if !self.registers.get_flag(Flag::Carry) { self.jr(); 12 } else { self.registers.pc += 1; 8 } }, // JR NC, r8
            0x31 => { self.registers.sp = self.fetch_word(); 12 }, // LD SP, d16
            0x32 => { self.memory.write(self.registers.hld(), self.registers.a); 8 }, // LD (HL-), A
            0x33 => { self.registers.sp = self.registers.sp.wrapping_add(1); 8 }, // INC SP
            0x34 => { let v = self.memory.read(self.registers.hl()); let v2 = self.reg_inc(v); self.memory.write(self.registers.hl(), v2); 12 }, // INC (HL)
            0x35 => { let v = self.memory.read(self.registers.hl()); let v2 = self.reg_dec(v); self.memory.write(self.registers.hl(), v2); 12 }, // DEC (HL)
            0x36 => { let v = self.fetch_byte(); self.memory.write(self.registers.hl(), v); 12 }, // LD (HL), d8
            0x38 => { if self.registers.get_flag(Flag::Carry) { self.jr(); 12 } else { self.registers.pc += 1; 8 } }, // JR C, r8
            0x3A => { self.registers.a = self.memory.read(self.registers.hld()); 8 }, // LD A, (HL-)
            0x3B => { self.registers.sp = self.registers.sp.wrapping_sub(1); 8 }, // DEC SP
            0x3C => { self.registers.a = self.reg_inc(self.registers.a); 4 }, // INC A
            0x3D => { self.registers.a = self.reg_dec(self.registers.a); 4 }, // DEC A
            0x3E => { self.registers.a = self.fetch_byte(); 8 }, // LD A, d8

            0x40 => { self.registers.b = self.registers.b; 4 }, // LD B, B
            0x41 => { self.registers.b = self.registers.c; 4 }, // LD B, C
            0x42 => { self.registers.b = self.registers.d; 4 }, // LD B, D
            0x43 => { self.registers.b = self.registers.e; 4 }, // LD B, E
            0x44 => { self.registers.b = self.registers.h; 4 }, // LD B, H
            0x45 => { self.registers.b = self.registers.l; 4 }, // LD B, L
            0x46 => { self.registers.b = self.memory.read(self.registers.hl()); 8 }, // LD B, (HL)
            0x47 => { self.registers.b = self.registers.a; 4 }, // LD B, A
            0x48 => { self.registers.c = self.registers.b; 4 }, // LD C, B
            0x49 => { self.registers.c = self.registers.c; 4 }, // LD C, C
            0x4A => { self.registers.c = self.registers.d; 4 }, // LD C, D
            0x4B => { self.registers.c = self.registers.e; 4 }, // LD C, E
            0x4C => { self.registers.c = self.registers.h; 4 }, // LD C, H
            0x4D => { self.registers.c = self.registers.l; 4 }, // LD C, L
            0x4E => { self.registers.c = self.memory.read(self.registers.hl()); 8 }, // LD C, (HL)
            0x4F => { self.registers.c = self.registers.a; 4 }, // LD C, A
            0x50 => { self.registers.d = self.registers.b; 4 }, // LD D, B
            0x51 => { self.registers.d = self.registers.c; 4 }, // LD D, C
            0x52 => { self.registers.d = self.registers.d; 4 }, // LD D, D
            0x53 => { self.registers.d = self.registers.e; 4 }, // LD D, E
            0x54 => { self.registers.d = self.registers.h; 4 }, // LD D, H
            0x55 => { self.registers.d = self.registers.l; 4 }, // LD D, L
            0x56 => { self.registers.d = self.memory.read(self.registers.hl()); 8 }, // LD D, (HL)
            0x57 => { self.registers.d = self.registers.a; 4 }, // LD D, A
            0x58 => { self.registers.e = self.registers.b; 4 }, // LD E, B
            0x59 => { self.registers.e = self.registers.c; 4 }, // LD E, C
            0x5A => { self.registers.e = self.registers.d; 4 }, // LD E, D
            0x5B => { self.registers.e = self.registers.e; 4 }, // LD E, E
            0x5C => { self.registers.e = self.registers.h; 4 }, // LD E, H
            0x5D => { self.registers.e = self.registers.l; 4 }, // LD E, L
            0x5E => { self.registers.e = self.memory.read(self.registers.hl()); 8 }, // LD E, (HL)
            0x5F => { self.registers.e = self.registers.a; 4 }, // LD E, A
            0x60 => { self.registers.h = self.registers.b; 4 }, // LD H, B
            0x61 => { self.registers.h = self.registers.c; 4 }, // LD H, C
            0x62 => { self.registers.h = self.registers.d; 4 }, // LD H, D
            0x63 => { self.registers.h = self.registers.e; 4 }, // LD H, E
            0x64 => { self.registers.h = self.registers.h; 4 }, // LD H, H
            0x65 => { self.registers.h = self.registers.l; 4 }, // LD H, L
            0x66 => { self.registers.h = self.memory.read(self.registers.hl()); 8 }, // LD H, (HL)
            0x67 => { self.registers.h = self.registers.a; 4 }, // LD H, A
            0x68 => { self.registers.l = self.registers.b; 4 }, // LD L, B
            0x69 => { self.registers.l = self.registers.c; 4 }, // LD L, C
            0x6A => { self.registers.l = self.registers.d; 4 }, // LD L, D
            0x6B => { self.registers.l = self.registers.e; 4 }, // LD L, E
            0x6C => { self.registers.l = self.registers.h; 4 }, // LD L, H
            0x6D => { self.registers.l = self.registers.l; 4 }, // LD L, L
            0x6E => { self.registers.l = self.memory.read(self.registers.hl()); 8 }, // LD L, (HL)
            0x6F => { self.registers.l = self.registers.a; 4 }, // LD L, A
            0x70 => { self.memory.write(self.registers.hl(), self.registers.b); 8 }, // LD (HL), B
            0x71 => { self.memory.write(self.registers.hl(), self.registers.c); 8 }, // LD (HL), C
            0x72 => { self.memory.write(self.registers.hl(), self.registers.d); 8 }, // LD (HL), D
            0x73 => { self.memory.write(self.registers.hl(), self.registers.e); 8 }, // LD (HL), E
            0x74 => { self.memory.write(self.registers.hl(), self.registers.h); 8 }, // LD (HL), H
            0x75 => { self.memory.write(self.registers.hl(), self.registers.l); 8 }, // LD (HL), L
            
            0x77 => { self.memory.write(self.registers.hl(), self.registers.a); 8 }, // LD (HL), A
            0x78 => { self.registers.a = self.registers.b; 4 }, // LD A, B
            0x79 => { self.registers.a = self.registers.c; 4 }, // LD A, C
            0x7A => { self.registers.a = self.registers.d; 4 }, // LD A, D
            0x7B => { self.registers.a = self.registers.e; 4 }, // LD A, E
            0x7C => { self.registers.a = self.registers.h; 4 }, // LD A, H
            0x7D => { self.registers.a = self.registers.l; 4 }, // LD A, L
            0x7E => { self.registers.a = self.memory.read(self.registers.hl()); 8 }, // LD A, (HL)
            0x7F => { self.registers.a = self.registers.a; 4 }, // LD A, A
            0x80 => { self.add(self.registers.b, false); 4 }, // ADD A, B
            0x81 => { self.add(self.registers.c, false); 4 }, // ADD A, C
            0x82 => { self.add(self.registers.d, false); 4 }, // ADD A, D
            0x83 => { self.add(self.registers.e, false); 4 }, // ADD A, E
            0x84 => { self.add(self.registers.h, false); 4 }, // ADD A, H
            0x85 => { self.add(self.registers.l, false); 4 }, // ADD A, L
            0x86 => { self.add(self.memory.read(self.registers.hl()), false); 8 }, // ADD A, (HL)
            0x87 => { self.add(self.registers.a, false); 4 }, // ADD A, A
            0x88 => { self.add(self.registers.b, true); 4 }, // ADC A, B
            0x89 => { self.add(self.registers.c, true); 4 }, // ADC A, C
            0x8A => { self.add(self.registers.d, true); 4 }, // ADC A, D
            0x8B => { self.add(self.registers.e, true); 4 }, // ADC A, E
            0x8C => { self.add(self.registers.h, true); 4 }, // ADC A, H
            0x8D => { self.add(self.registers.l, true); 4 }, // ADC A, L
            0x8E => { self.add(self.memory.read(self.registers.hl()), true); 8 }, // ADC A, (HL)
            0x8F => { self.add(self.registers.a, true); 4 }, // ADC A, A
            0x90 => { self.sub(self.registers.b, false); 4 }, // SUB B
            0x91 => { self.sub(self.registers.c, false); 4 }, // SUB C
            0x92 => { self.sub(self.registers.d, false); 4 }, // SUB D
            0x93 => { self.sub(self.registers.e, false); 4 }, // SUB E
            0x94 => { self.sub(self.registers.h, false); 4 }, // SUB H
            0x95 => { self.sub(self.registers.l, false); 4 }, // SUB L
            0x96 => { self.sub(self.memory.read(self.registers.hl()), false); 8 }, // SUB (HL)
            0x97 => { self.sub(self.registers.a, false); 4 }, // SUB A
            0x98 => { self.sub(self.registers.b, true); 4 }, // SBC A, B
            0x99 => { self.sub(self.registers.c, true); 4 }, // SBC A, C
            0x9A => { self.sub(self.registers.d, true); 4 }, // SBC A, D
            0x9B => { self.sub(self.registers.e, true); 4 }, // SBC A, E
            0x9C => { self.sub(self.registers.h, true); 4 }, // SBC A, H
            0x9D => { self.sub(self.registers.l, true); 4 }, // SBC A, L
            0x9E => { self.sub(self.memory.read(self.registers.hl()), true); 8 }, // SBC A, (HL)
            0x9F => { self.sub(self.registers.a, true); 4 }, // SBC A, A
            0xA0 => { self.and(self.registers.b); 4 }, // AND B
            0xA1 => { self.and(self.registers.c); 4 }, // AND C
            0xA2 => { self.and(self.registers.d); 4 }, // AND D
            0xA3 => { self.and(self.registers.e); 4 }, // AND E
            0xA4 => { self.and(self.registers.h); 4 }, // AND H
            0xA5 => { self.and(self.registers.l); 4 }, // AND L
            0xA6 => { self.and(self.memory.read(self.registers.hl())); 8 }, // AND (HL)
            0xA7 => { self.and(self.registers.a); 4 }, // AND A
            0xA8 => { self.xor(self.registers.b); 4 }, // XOR B
            0xA9 => { self.xor(self.registers.c); 4 }, // XOR C
            0xAA => { self.xor(self.registers.d); 4 }, // XOR D
            0xAB => { self.xor(self.registers.e); 4 }, // XOR E
            0xAC => { self.xor(self.registers.h); 4 }, // XOR H
            0xAD => { self.xor(self.registers.l); 4 }, // XOR L
            0xAE => { self.xor(self.memory.read(self.registers.hl())); 8 }, // XOR (HL)
            0xAF => { self.xor(self.registers.a); 4 }, // XOR A
            0xB0 => { self.or(self.registers.b); 4 }, // OR B
            0xB1 => { self.or(self.registers.c); 4 }, // OR C
            0xB2 => { self.or(self.registers.d); 4 }, // OR D
            0xB3 => { self.or(self.registers.e); 4 }, // OR E
            0xB4 => { self.or(self.registers.h); 4 }, // OR H
            0xB5 => { self.or(self.registers.l); 4 }, // OR L
            0xB6 => { self.or(self.memory.read(self.registers.hl())); 8 }, // OR (HL)
            0xB7 => { self.or(self.registers.a); 4 }, // OR A
            0xB8 => { self.cp(self.registers.b); 4 }, // CP B
            0xB9 => { self.cp(self.registers.c); 4 }, // CP C
            0xBA => { self.cp(self.registers.d); 4 }, // CP D
            0xBB => { self.cp(self.registers.e); 4 }, // CP E
            0xBC => { self.cp(self.registers.h); 4 }, // CP H
            0xBD => { self.cp(self.registers.l); 4 }, // CP L
            0xBE => { self.cp(self.memory.read(self.registers.hl())); 8 }, // CP (HL)
            0xBF => { self.cp(self.registers.a); 4 }, // CP A
            0xC0 => { if !self.registers.get_flag(Flag::Zero) { self.registers.pc = self.pop_stack(); 20 } else { 8 } }, // RET NZ
            0xC1 => { let v = self.pop_stack(); self.registers.set_bc(v); 12 }, // POP BC
            0xC2 => { if !self.registers.get_flag(Flag::Zero) { self.registers.pc = self.fetch_word(); 16 } else { 12 } }, // JP NZ, a16
            0xC3 => { self.registers.pc = self.fetch_word(); 16 }, // JP a16
            0xC5 => { self.push_stack(self.registers.bc()); 16 }, // PUSH BC
            0xC6 => { let v = self.fetch_byte(); self.add(v, false); 8 }, // ADD A, d8
            0xC8 => { if self.registers.get_flag(Flag::Zero) { self.registers.pc = self.pop_stack(); 20 } else { 8 } }, // RET Z
            0xC9 => { self.registers.pc = self.pop_stack(); 16 }, // RET
            0xCA => { if self.registers.get_flag(Flag::Zero) { self.registers.pc = self.fetch_word(); 16 } else { self.registers.pc += 2; 12 } }, // JP Z, a16
            0xcb => { self.cb_call() + 4 }, // CB
            0xcd => { self.push_stack(self.registers.pc + 2); self.registers.pc = self.fetch_word(); 24 }, // CALL a16
            0xCE => { let v = self.fetch_byte(); self.add(v, true); 8 }, // ADC A, d8
            0xD0 => { if !self.registers.get_flag(Flag::Carry) { self.registers.pc = self.pop_stack(); 20 } else { self.registers.pc += 1; 8 } }, // RET NC
            0xD1 => { let v = self.pop_stack(); self.registers.set_de(v); 12 }, // POP DE
            0xD5 => { self.push_stack(self.registers.de()); 16 }, // PUSH DE
            0xD6 => { let v = self.fetch_byte(); self.sub(v, false); 8 }, // SUB d8
            0xD8 => { if self.registers.get_flag(Flag::Carry) { self.registers.pc = self.pop_stack(); 20 } else { 8 } }, // RET C
            0xD9 => { self.registers.pc = self.pop_stack(); self.ime = true; 16 }, // RETI
            0xE0 => { let v = 0xFF00 | self.fetch_byte() as u16; self.memory.write(v, self.registers.a); 12 }, // LDH (a8), A
            0xe1 => { let v = self.pop_stack(); self.registers.set_hl(v); 12 }, // POP HL
            0xe2 => { let v = 0xFF00 | self.registers.c as u16; self.memory.write(v, self.registers.a); 8 }, // LD (C), A
            0xe5 => { self.push_stack(self.registers.hl()); 16 }, // PUSH HL
            0xe6 => { let v = self.fetch_byte(); self.and(v); 8 }, // AND d8
            0xe9 => { self.registers.pc = self.registers.hl(); 4 }, // JP (HL)
            0xea => { let v = self.fetch_word(); self.memory.write(v, self.registers.a); 16 }, // LD (a16), A
            0xee => { let v = self.fetch_byte(); self.xor(v); 8 }, // XOR d8
            0xef => { self.push_stack(self.registers.pc); self.registers.pc = 0x28; 16 }, // RST 28H
            0xF0 => { let v = 0xFF00 | self.fetch_byte() as u16; self.registers.a = self.memory.read(v); 12 }, // LDH A, (a8)
            0xf1 => { let v = self.pop_stack(); self.registers.set_af(v); 12 }, // POP AF
            0xF3 => { self.ime = false; 4 }, // DI
            0xf5 => { self.push_stack(self.registers.af()); 16 }, // PUSH AF
            0xf6 => { let v = self.fetch_byte(); self.or(v); 8 }, // OR d8
            0xfa => { let v = self.fetch_word(); self.registers.a = self.memory.read(v); 16 }, // LD A, (a16)
            0xfb => { self.ime = true; 4 }
            0xFE => { let v = self.fetch_byte(); self.cp(v); 8 }, // CP d8
            0xff => { self.push_stack(self.registers.pc); self.registers.pc = 0x38; 16 }, // RST 38H
            _ => { panic!("Unimplemented opcode: {:#04x}", opcode); }
        }
    }

    fn cb_call(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        match opcode {
            0x00 => { self.registers.b = self.rlc(self.registers.b); 8 }, // RLC B
            0x01 => { self.registers.c = self.rlc(self.registers.c); 8 }, // RLC C
            0x02 => { self.registers.d = self.rlc(self.registers.d); 8 }, // RLC D
            0x03 => { self.registers.e = self.rlc(self.registers.e); 8 }, // RLC E
            0x04 => { self.registers.h = self.rlc(self.registers.h); 8 }, // RLC H
            0x05 => { self.registers.l = self.rlc(self.registers.l); 8 }, // RLC L
            0x06 => { let v = self.memory.read(self.registers.hl()); let v2 = self.rlc(v); self.memory.write(self.registers.hl(), v2); 16 }, // RLC (HL)
            0x07 => { self.registers.a = self.rlc(self.registers.a); 8 }, // RLC A
            0x08 => { self.registers.b = self.rrc(self.registers.b); 8 }, // RRC B
            0x09 => { self.registers.c = self.rrc(self.registers.c); 8 }, // RRC C
            0x0A => { self.registers.d = self.rrc(self.registers.d); 8 }, // RRC D
            0x0B => { self.registers.e = self.rrc(self.registers.e); 8 }, // RRC E
            0x0C => { self.registers.h = self.rrc(self.registers.h); 8 }, // RRC H
            0x0D => { self.registers.l = self.rrc(self.registers.l); 8 }, // RRC L
            0x0E => { let v = self.memory.read(self.registers.hl()); let v2 = self.rrc(v); self.memory.write(self.registers.hl(), v2); 16 }, // RRC (HL)
            0x0F => { self.registers.a = self.rrc(self.registers.a); 8 }, // RRC A
            0x10 => { self.registers.b = self.rl(self.registers.b); 8 }, // RL B
            0x11 => { self.registers.c = self.rl(self.registers.c); 8 }, // RL C
            0x12 => { self.registers.d = self.rl(self.registers.d); 8 }, // RL D
            0x13 => { self.registers.e = self.rl(self.registers.e); 8 }, // RL E
            0x14 => { self.registers.h = self.rl(self.registers.h); 8 }, // RL H
            0x15 => { self.registers.l = self.rl(self.registers.l); 8 }, // RL L
            0x16 => { let v = self.memory.read(self.registers.hl()); let v2 = self.rl(v); self.memory.write(self.registers.hl(), v2); 16 }, // RL (HL)
            0x17 => { self.registers.a = self.rl(self.registers.a); 8 }, // RL A
            0x18 => { self.registers.b = self.rr(self.registers.b); 8 }, // RR B
            0x19 => { self.registers.c = self.rr(self.registers.c); 8 }, // RR C
            0x1A => { self.registers.d = self.rr(self.registers.d); 8 }, // RR D
            0x1B => { self.registers.e = self.rr(self.registers.e); 8 }, // RR E
            0x1C => { self.registers.h = self.rr(self.registers.h); 8 }, // RR H
            0x1D => { self.registers.l = self.rr(self.registers.l); 8 }, // RR L
            0x1E => { let v = self.memory.read(self.registers.hl()); let v2 = self.rr(v); self.memory.write(self.registers.hl(), v2); 16 }, // RR (HL)
            0x1F => { self.registers.a = self.rr(self.registers.a); 8 }, // RR A
            0x20 => { self.registers.b = self.sla(self.registers.b); 8 }, // SLA B
            0x21 => { self.registers.c = self.sla(self.registers.c); 8 }, // SLA C
            0x22 => { self.registers.d = self.sla(self.registers.d); 8 }, // SLA D
            0x23 => { self.registers.e = self.sla(self.registers.e); 8 }, // SLA E
            0x24 => { self.registers.h = self.sla(self.registers.h); 8 }, // SLA H
            0x25 => { self.registers.l = self.sla(self.registers.l); 8 }, // SLA L
            0x26 => { let v = self.memory.read(self.registers.hl()); let v2 = self.sla(v); self.memory.write(self.registers.hl(), v2); 16 }, // SLA (HL)
            0x27 => { self.registers.a = self.sla(self.registers.a); 8 }, // SLA A
            0x28 => { self.registers.b = self.sra(self.registers.b); 8 }, // SRA B
            0x29 => { self.registers.c = self.sra(self.registers.c); 8 }, // SRA C
            0x2A => { self.registers.d = self.sra(self.registers.d); 8 }, // SRA D
            0x2B => { self.registers.e = self.sra(self.registers.e); 8 }, // SRA E
            0x2C => { self.registers.h = self.sra(self.registers.h); 8 }, // SRA H
            0x2D => { self.registers.l = self.sra(self.registers.l); 8 }, // SRA L
            0x2E => { let v = self.memory.read(self.registers.hl()); let v2 = self.sra(v); self.memory.write(self.registers.hl(), v2); 16 }, // SRA (HL)
            0x2F => { self.registers.a = self.sra(self.registers.a); 8 }, // SRA A
            0x30 => { self.registers.b = self.swap(self.registers.b); 8 }, // SWAP B
            0x31 => { self.registers.c = self.swap(self.registers.c); 8 }, // SWAP C
            0x32 => { self.registers.d = self.swap(self.registers.d); 8 }, // SWAP D
            0x33 => { self.registers.e = self.swap(self.registers.e); 8 }, // SWAP E
            0x34 => { self.registers.h = self.swap(self.registers.h); 8 }, // SWAP H
            0x35 => { self.registers.l = self.swap(self.registers.l); 8 }, // SWAP L
            0x36 => { let v = self.memory.read(self.registers.hl()); let v2 = self.swap(v); self.memory.write(self.registers.hl(), v2); 16 }, // SWAP (HL)
            0x37 => { self.registers.a = self.swap(self.registers.a); 8 }, // SWAP A
            0x38 => { self.registers.b = self.srl(self.registers.b); 8 }, // SRL B
            0x39 => { self.registers.c = self.srl(self.registers.c); 8 }, // SRL C
            0x3A => { self.registers.d = self.srl(self.registers.d); 8 }, // SRL D
            0x3B => { self.registers.e = self.srl(self.registers.e); 8 }, // SRL E
            0x3C => { self.registers.h = self.srl(self.registers.h); 8 }, // SRL H
            0x3D => { self.registers.l = self.srl(self.registers.l); 8 }, // SRL L
            0x3E => { let v = self.memory.read(self.registers.hl()); let v2 = self.srl(v); self.memory.write(self.registers.hl(), v2); 16 }, // SRL (HL)
            0x3F => { self.registers.a = self.srl(self.registers.a); 8 }, // SRL A
            0x40 => { self.bit(self.registers.b, 0); 8 }, // BIT 0, B
            0x41 => { self.bit(self.registers.c, 0); 8 }, // BIT 0, C
            0x42 => { self.bit(self.registers.d, 0); 8 }, // BIT 0, D
            0x43 => { self.bit(self.registers.e, 0); 8 }, // BIT 0, E
            0x44 => { self.bit(self.registers.h, 0); 8 }, // BIT 0, H
            0x45 => { self.bit(self.registers.l, 0); 8 }, // BIT 0, L
            0x46 => { let v = self.memory.read(self.registers.hl()); self.bit(v, 0); 16 }, // BIT 0, (HL)
            0x47 => { self.bit(self.registers.a, 0); 8 }, // BIT 0, A
            0x48 => { self.bit(self.registers.b, 1); 8 }, // BIT 1, B
            0x49 => { self.bit(self.registers.c, 1); 8 }, // BIT 1, C
            0x4A => { self.bit(self.registers.d, 1); 8 }, // BIT 1, D
            0x4B => { self.bit(self.registers.e, 1); 8 }, // BIT 1, E
            0x4C => { self.bit(self.registers.h, 1); 8 }, // BIT 1, H
            0x4D => { self.bit(self.registers.l, 1); 8 }, // BIT 1, L
            0x4E => { let v = self.memory.read(self.registers.hl()); self.bit(v, 1); 16 }, // BIT 1, (HL)
            0x4F => { self.bit(self.registers.a, 1); 8 }, // BIT 1, A
            0x50 => { self.bit(self.registers.b, 2); 8 }, // BIT 2, B
            0x51 => { self.bit(self.registers.c, 2); 8 }, // BIT 2, C
            0x52 => { self.bit(self.registers.d, 2); 8 }, // BIT 2, D
            0x53 => { self.bit(self.registers.e, 2); 8 }, // BIT 2, E
            0x54 => { self.bit(self.registers.h, 2); 8 }, // BIT 2, H
            0x55 => { self.bit(self.registers.l, 2); 8 }, // BIT 2, L
            0x56 => { let v = self.memory.read(self.registers.hl()); self.bit(v, 2); 16 }, // BIT 2, (HL)
            0x57 => { self.bit(self.registers.a, 2); 8 }, // BIT 2, A
            0x58 => { self.bit(self.registers.b, 3); 8 }, // BIT 3, B
            0x59 => { self.bit(self.registers.c, 3); 8 }, // BIT 3, C
            0x5A => { self.bit(self.registers.d, 3); 8 }, // BIT 3, D
            0x5B => { self.bit(self.registers.e, 3); 8 }, // BIT 3, E
            0x5C => { self.bit(self.registers.h, 3); 8 }, // BIT 3, H
            0x5D => { self.bit(self.registers.l, 3); 8 }, // BIT 3, L
            0x5E => { let v = self.memory.read(self.registers.hl()); self.bit(v, 3); 16 }, // BIT 3, (HL)
            0x5F => { self.bit(self.registers.a, 3); 8 }, // BIT 3, A
            0x60 => { self.bit(self.registers.b, 4); 8 }, // BIT 4, B
            0x61 => { self.bit(self.registers.c, 4); 8 }, // BIT 4, C
            0x62 => { self.bit(self.registers.d, 4); 8 }, // BIT 4, D
            0x63 => { self.bit(self.registers.e, 4); 8 }, // BIT 4, E
            0x64 => { self.bit(self.registers.h, 4); 8 }, // BIT 4, H
            0x65 => { self.bit(self.registers.l, 4); 8 }, // BIT 4, L
            0x66 => { let v = self.memory.read(self.registers.hl()); self.bit(v, 4); 16 }, // BIT 4, (HL)
            0x67 => { self.bit(self.registers.a, 4); 8 }, // BIT 4, A
            0x68 => { self.bit(self.registers.b, 5); 8 }, // BIT 5, B
            0x69 => { self.bit(self.registers.c, 5); 8 }, // BIT 5, C
            0x6A => { self.bit(self.registers.d, 5); 8 }, // BIT 5, D
            0x6B => { self.bit(self.registers.e, 5); 8 }, // BIT 5, E
            0x6C => { self.bit(self.registers.h, 5); 8 }, // BIT 5, H
            0x6D => { self.bit(self.registers.l, 5); 8 }, // BIT 5, L
            0x6E => { let v = self.memory.read(self.registers.hl()); self.bit(v, 5); 16 }, // BIT 5, (HL)
            0x6F => { self.bit(self.registers.a, 5); 8 }, // BIT 5, A
            0x70 => { self.bit(self.registers.b, 6); 8 }, // BIT 6, B
            0x71 => { self.bit(self.registers.c, 6); 8 }, // BIT 6, C
            0x72 => { self.bit(self.registers.d, 6); 8 }, // BIT 6, D
            0x73 => { self.bit(self.registers.e, 6); 8 }, // BIT 6, E
            0x74 => { self.bit(self.registers.h, 6); 8 }, // BIT 6, H
            0x75 => { self.bit(self.registers.l, 6); 8 }, // BIT 6, L
            0x76 => { let v = self.memory.read(self.registers.hl()); self.bit(v, 6); 16 }, // BIT 6, (HL)
            0x77 => { self.bit(self.registers.a, 6); 8 }, // BIT 6, A
            0x78 => { self.bit(self.registers.b, 7); 8 }, // BIT 7, B
            0x79 => { self.bit(self.registers.c, 7); 8 }, // BIT 7, C
            0x7A => { self.bit(self.registers.d, 7); 8 }, // BIT 7, D
            0x7B => { self.bit(self.registers.e, 7); 8 }, // BIT 7, E
            0x7C => { self.bit(self.registers.h, 7); 8 }, // BIT 7, H
            0x7D => { self.bit(self.registers.l, 7); 8 }, // BIT 7, L
            0x7E => { let v = self.memory.read(self.registers.hl()); self.bit(v, 7); 16 }, // BIT 7, (HL)
            0x7F => { self.bit(self.registers.a, 7); 8 }, // BIT 7, A
            0x80 => { self.registers.b = self.res(self.registers.b, 0); 8 }, // RES 0, B
            0x81 => { self.registers.c = self.res(self.registers.c, 0); 8 }, // RES 0, C
            0x82 => { self.registers.d = self.res(self.registers.d, 0); 8 }, // RES 0, D
            0x83 => { self.registers.e = self.res(self.registers.e, 0); 8 }, // RES 0, E
            0x84 => { self.registers.h = self.res(self.registers.h, 0); 8 }, // RES 0, H
            0x85 => { self.registers.l = self.res(self.registers.l, 0); 8 }, // RES 0, L
            0x86 => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 0); self.memory.write(self.registers.hl(), v2); 16 }, // RES 0, (HL)
            0x87 => { self.registers.a = self.res(self.registers.a, 0); 8 }, // RES 0, A
            0x88 => { self.registers.b = self.res(self.registers.b, 1); 8 }, // RES 1, B
            0x89 => { self.registers.c = self.res(self.registers.c, 1); 8 }, // RES 1, C
            0x8A => { self.registers.d = self.res(self.registers.d, 1); 8 }, // RES 1, D
            0x8B => { self.registers.e = self.res(self.registers.e, 1); 8 }, // RES 1, E
            0x8C => { self.registers.h = self.res(self.registers.h, 1); 8 }, // RES 1, H
            0x8D => { self.registers.l = self.res(self.registers.l, 1); 8 }, // RES 1, L
            0x8E => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 1); self.memory.write(self.registers.hl(), v2); 16 }, // RES 1, (HL)
            0x8F => { self.registers.a = self.res(self.registers.a, 1); 8 }, // RES 1, A
            0x90 => { self.registers.b = self.res(self.registers.b, 2); 8 }, // RES 2, B
            0x91 => { self.registers.c = self.res(self.registers.c, 2); 8 }, // RES 2, C
            0x92 => { self.registers.d = self.res(self.registers.d, 2); 8 }, // RES 2, D
            0x93 => { self.registers.e = self.res(self.registers.e, 2); 8 }, // RES 2, E
            0x94 => { self.registers.h = self.res(self.registers.h, 2); 8 }, // RES 2, H
            0x95 => { self.registers.l = self.res(self.registers.l, 2); 8 }, // RES 2, L
            0x96 => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 2); self.memory.write(self.registers.hl(), v2); 16 }, // RES 2, (HL)
            0x97 => { self.registers.a = self.res(self.registers.a, 2); 8 }, // RES 2, A
            0x98 => { self.registers.b = self.res(self.registers.b, 3); 8 }, // RES 3, B
            0x99 => { self.registers.c = self.res(self.registers.c, 3); 8 }, // RES 3, C
            0x9A => { self.registers.d = self.res(self.registers.d, 3); 8 }, // RES 3, D
            0x9B => { self.registers.e = self.res(self.registers.e, 3); 8 }, // RES 3, E
            0x9C => { self.registers.h = self.res(self.registers.h, 3); 8 }, // RES 3, H
            0x9D => { self.registers.l = self.res(self.registers.l, 3); 8 }, // RES 3, L
            0x9E => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 3); self.memory.write(self.registers.hl(), v2); 16 }, // RES 3, (HL)
            0x9F => { self.registers.a = self.res(self.registers.a, 3); 8 }, // RES 3, A
            0xA0 => { self.registers.b = self.res(self.registers.b, 4); 8 }, // RES 4, B
            0xA1 => { self.registers.c = self.res(self.registers.c, 4); 8 }, // RES 4, C
            0xA2 => { self.registers.d = self.res(self.registers.d, 4); 8 }, // RES 4, D
            0xA3 => { self.registers.e = self.res(self.registers.e, 4); 8 }, // RES 4, E
            0xA4 => { self.registers.h = self.res(self.registers.h, 4); 8 }, // RES 4, H
            0xA5 => { self.registers.l = self.res(self.registers.l, 4); 8 }, // RES 4, L
            0xA6 => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 4); self.memory.write(self.registers.hl(), v2); 16 }, // RES 4, (HL)
            0xA7 => { self.registers.a = self.res(self.registers.a, 4); 8 }, // RES 4, A
            0xA8 => { self.registers.b = self.res(self.registers.b, 5); 8 }, // RES 5, B
            0xA9 => { self.registers.c = self.res(self.registers.c, 5); 8 }, // RES 5, C
            0xAA => { self.registers.d = self.res(self.registers.d, 5); 8 }, // RES 5, D
            0xAB => { self.registers.e = self.res(self.registers.e, 5); 8 }, // RES 5, E
            0xAC => { self.registers.h = self.res(self.registers.h, 5); 8 }, // RES 5, H
            0xAD => { self.registers.l = self.res(self.registers.l, 5); 8 }, // RES 5, L
            0xAE => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 5); self.memory.write(self.registers.hl(), v2); 16 }, // RES 5, (HL)
            0xAF => { self.registers.a = self.res(self.registers.a, 5); 8 }, // RES 5, A
            0xB0 => { self.registers.b = self.res(self.registers.b, 6); 8 }, // RES 6, B
            0xB1 => { self.registers.c = self.res(self.registers.c, 6); 8 }, // RES 6, C
            0xB2 => { self.registers.d = self.res(self.registers.d, 6); 8 }, // RES 6, D
            0xB3 => { self.registers.e = self.res(self.registers.e, 6); 8 }, // RES 6, E
            0xB4 => { self.registers.h = self.res(self.registers.h, 6); 8 }, // RES 6, H
            0xB5 => { self.registers.l = self.res(self.registers.l, 6); 8 }, // RES 6, L
            0xB6 => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 6); self.memory.write(self.registers.hl(), v2); 16 }, // RES 6, (HL)
            0xB7 => { self.registers.a = self.res(self.registers.a, 6); 8 }, // RES 6, A
            0xB8 => { self.registers.b = self.res(self.registers.b, 7); 8 }, // RES 7, B
            0xB9 => { self.registers.c = self.res(self.registers.c, 7); 8 }, // RES 7, C
            0xBA => { self.registers.d = self.res(self.registers.d, 7); 8 }, // RES 7, D
            0xBB => { self.registers.e = self.res(self.registers.e, 7); 8 }, // RES 7, E
            0xBC => { self.registers.h = self.res(self.registers.h, 7); 8 }, // RES 7, H
            0xBD => { self.registers.l = self.res(self.registers.l, 7); 8 }, // RES 7, L
            0xBE => { let v = self.memory.read(self.registers.hl()); let v2 = self.res(v, 7); self.memory.write(self.registers.hl(), v2); 16 }, // RES 7, (HL)
            0xBF => { self.registers.a = self.res(self.registers.a, 7); 8 }, // RES 7, A
            0xC0 => { self.registers.b = self.set(self.registers.b, 0); 8 }, // SET 0, B
            0xC1 => { self.registers.c = self.set(self.registers.c, 0); 8 }, // SET 0, C
            0xC2 => { self.registers.d = self.set(self.registers.d, 0); 8 }, // SET 0, D
            0xC3 => { self.registers.e = self.set(self.registers.e, 0); 8 }, // SET 0, E
            0xC4 => { self.registers.h = self.set(self.registers.h, 0); 8 }, // SET 0, H
            0xC5 => { self.registers.l = self.set(self.registers.l, 0); 8 }, // SET 0, L
            0xC6 => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 0); self.memory.write(self.registers.hl(), v2); 16 }, // SET 0, (HL)
            0xC7 => { self.registers.a = self.set(self.registers.a, 0); 8 }, // SET 0, A
            0xC8 => { self.registers.b = self.set(self.registers.b, 1); 8 }, // SET 1, B
            0xC9 => { self.registers.c = self.set(self.registers.c, 1); 8 }, // SET 1, C
            0xCA => { self.registers.d = self.set(self.registers.d, 1); 8 }, // SET 1, D
            0xCB => { self.registers.e = self.set(self.registers.e, 1); 8 }, // SET 1, E
            0xCC => { self.registers.h = self.set(self.registers.h, 1); 8 }, // SET 1, H
            0xCD => { self.registers.l = self.set(self.registers.l, 1); 8 }, // SET 1, L
            0xCE => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 1); self.memory.write(self.registers.hl(), v2); 16 }, // SET 1, (HL)
            0xCF => { self.registers.a = self.set(self.registers.a, 1); 8 }, // SET 1, A
            0xD0 => { self.registers.b = self.set(self.registers.b, 2); 8 }, // SET 2, B
            0xD1 => { self.registers.c = self.set(self.registers.c, 2); 8 }, // SET 2, C
            0xD2 => { self.registers.d = self.set(self.registers.d, 2); 8 }, // SET 2, D
            0xD3 => { self.registers.e = self.set(self.registers.e, 2); 8 }, // SET 2, E
            0xD4 => { self.registers.h = self.set(self.registers.h, 2); 8 }, // SET 2, H
            0xD5 => { self.registers.l = self.set(self.registers.l, 2); 8 }, // SET 2, L
            0xD6 => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 2); self.memory.write(self.registers.hl(), v2); 16 }, // SET 2, (HL)
            0xD7 => { self.registers.a = self.set(self.registers.a, 2); 8 }, // SET 2, A
            0xD8 => { self.registers.b = self.set(self.registers.b, 3); 8 }, // SET 3, B
            0xD9 => { self.registers.c = self.set(self.registers.c, 3); 8 }, // SET 3, C
            0xDA => { self.registers.d = self.set(self.registers.d, 3); 8 }, // SET 3, D
            0xDB => { self.registers.e = self.set(self.registers.e, 3); 8 }, // SET 3, E
            0xDC => { self.registers.h = self.set(self.registers.h, 3); 8 }, // SET 3, H
            0xDD => { self.registers.l = self.set(self.registers.l, 3); 8 }, // SET 3, L
            0xDE => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 3); self.memory.write(self.registers.hl(), v2); 16 }, // SET 3, (HL)
            0xDF => { self.registers.a = self.set(self.registers.a, 3); 8 }, // SET 3, A
            0xE0 => { self.registers.b = self.set(self.registers.b, 4); 8 }, // SET 4, B
            0xE1 => { self.registers.c = self.set(self.registers.c, 4); 8 }, // SET 4, C
            0xE2 => { self.registers.d = self.set(self.registers.d, 4); 8 }, // SET 4, D
            0xE3 => { self.registers.e = self.set(self.registers.e, 4); 8 }, // SET 4, E
            0xE4 => { self.registers.h = self.set(self.registers.h, 4); 8 }, // SET 4, H
            0xE5 => { self.registers.l = self.set(self.registers.l, 4); 8 }, // SET 4, L
            0xE6 => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 4); self.memory.write(self.registers.hl(), v2); 16 }, // SET 4, (HL)
            0xE7 => { self.registers.a = self.set(self.registers.a, 4); 8 }, // SET 4, A
            0xE8 => { self.registers.b = self.set(self.registers.b, 5); 8 }, // SET 5, B
            0xE9 => { self.registers.c = self.set(self.registers.c, 5); 8 }, // SET 5, C
            0xEA => { self.registers.d = self.set(self.registers.d, 5); 8 }, // SET 5, D
            0xEB => { self.registers.e = self.set(self.registers.e, 5); 8 }, // SET 5, E
            0xEC => { self.registers.h = self.set(self.registers.h, 5); 8 }, // SET 5, H
            0xED => { self.registers.l = self.set(self.registers.l, 5); 8 }, // SET 5, L
            0xEE => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 5); self.memory.write(self.registers.hl(), v2); 16 }, // SET 5, (HL)
            0xEF => { self.registers.a = self.set(self.registers.a, 5); 8 }, // SET 5, A
            0xF0 => { self.registers.b = self.set(self.registers.b, 6); 8 }, // SET 6, B
            0xF1 => { self.registers.c = self.set(self.registers.c, 6); 8 }, // SET 6, C
            0xF2 => { self.registers.d = self.set(self.registers.d, 6); 8 }, // SET 6, D
            0xF3 => { self.registers.e = self.set(self.registers.e, 6); 8 }, // SET 6, E
            0xF4 => { self.registers.h = self.set(self.registers.h, 6); 8 }, // SET 6, H
            0xF5 => { self.registers.l = self.set(self.registers.l, 6); 8 }, // SET 6, L
            0xF6 => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 6); self.memory.write(self.registers.hl(), v2); 16 }, // SET 6, (HL)
            0xF7 => { self.registers.a = self.set(self.registers.a, 6); 8 }, // SET 6, A
            0xF8 => { self.registers.b = self.set(self.registers.b, 7); 8 }, // SET 7, B
            0xF9 => { self.registers.c = self.set(self.registers.c, 7); 8 }, // SET 7, C
            0xFA => { self.registers.d = self.set(self.registers.d, 7); 8 }, // SET 7, D
            0xFB => { self.registers.e = self.set(self.registers.e, 7); 8 }, // SET 7, E
            0xFC => { self.registers.h = self.set(self.registers.h, 7); 8 }, // SET 7, H
            0xFD => { self.registers.l = self.set(self.registers.l, 7); 8 }, // SET 7, L
            0xFE => { let v = self.memory.read(self.registers.hl()); let v2 = self.set(v, 7); self.memory.write(self.registers.hl(), v2); 16 }, // SET 7, (HL)
            0xFF => { self.registers.a = self.set(self.registers.a, 7); 8 }, // SET 7, A
        }
    }

    #[inline(always)]
    fn reg_inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (value & 0x0F) + 1 > 0x0F);
        result
    }

    #[inline(always)]
    fn reg_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, true);
        self.registers.set_flag(Flag::HalfCarry, (value & 0x0F) == 0);
        result
    }

    #[inline(always)]
    fn add(&mut self, value: u8, need_carry: bool) {
        let carry = (need_carry && self.registers.get_flag(Flag::Carry)) as u8;
        let a = self.registers.a;
        let result = a.wrapping_add(value).wrapping_add(carry);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (a & 0x0F) + (value & 0x0F) > 0x0F);
        self.registers.set_flag(Flag::Carry, a > 0xFF - value);
        self.registers.a = result;
    }

    #[inline(always)]
    fn sub(&mut self, value: u8, need_carry: bool) {
        let carry = (need_carry && self.registers.get_flag(Flag::Carry)) as u8;
        let a = self.registers.a;
        let result = a.wrapping_sub(value).wrapping_sub(carry);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, true);
        self.registers.set_flag(Flag::HalfCarry, (a & 0x0F) < (value & 0x0F) + carry);
        self.registers.set_flag(Flag::Carry, a < value + carry);
        self.registers.a = result;
    }

    #[inline(always)]
    fn and(&mut self, value: u8) {
        let result = self.registers.a & value;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, true);
        self.registers.set_flag(Flag::Carry, false);
        self.registers.a = result;
    }

    #[inline(always)]
    fn xor(&mut self, value: u8) {
        let result = self.registers.a ^ value;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, false);
        self.registers.a = result;
    }

    #[inline(always)]
    fn or(&mut self, value: u8) {
        let result = self.registers.a | value;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, false);
        self.registers.a = result;
    }

    #[inline(always)]
    fn cp(&mut self, value: u8) {
        let a = self.registers.a;
        let result = a.wrapping_sub(value);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, true);
        self.registers.set_flag(Flag::HalfCarry, (a & 0x0F) < (value & 0x0F));
        self.registers.set_flag(Flag::Carry, a < value);
    }

    #[inline(always)]
    fn rlc(&mut self, value: u8) -> u8 {
        let carry = value >> 7;
        let result = (value << 1) | carry;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, carry == 1);
        result
    }

    #[inline(always)]
    fn rrc(&mut self, value: u8) -> u8 {
        let carry = value & 1;
        let result = (value >> 1) | (carry << 7);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, carry == 1);
        result
    }
    
    #[inline(always)]
    fn rl(&mut self, value: u8) -> u8 {
        let carry = self.registers.get_flag(Flag::Carry) as u8;
        let result = (value << 1) | carry;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, value >> 7 == 1);
        result
    }

    #[inline(always)]
    fn rr(&mut self, value: u8) -> u8 {
        let carry = self.registers.get_flag(Flag::Carry) as u8;
        let result = (value >> 1) | (carry << 7);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, value & 1 == 1);
        result
    }

    #[inline(always)]
    fn sla(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, value >> 7 == 1);
        result
    }

    #[inline(always)]
    fn sra(&mut self, value: u8) -> u8 {
        let result = (value >> 1) | (value & 0x80);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, value & 1 == 1);
        result
    }

    #[inline(always)]
    fn swap(&mut self, value: u8) -> u8 {
        let result = (value >> 4) | (value << 4);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, false);
        result
    }

    #[inline(always)]
    fn srl(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.set_flag(Flag::Carry, value & 1 == 1);
        result
    }

    #[inline(always)]
    fn bit(&mut self, value: u8, bit: u8) {
        self.registers.set_flag(Flag::Zero, value & (1 << bit) == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, true);
    }

    #[inline(always)]
    fn daa(&mut self) {
        let mut a = self.registers.a;
        let mut adjust = 0;
        if self.registers.get_flag(Flag::HalfCarry) || (!self.registers.get_flag(Flag::Sub) && (a & 0x0F) > 9) {
            adjust |= 0x06;
        }
        if self.registers.get_flag(Flag::Carry) || (!self.registers.get_flag(Flag::Sub) && a > 0x99) {
            adjust |= 0x60;
            self.registers.set_flag(Flag::Carry, true);
        }
        if self.registers.get_flag(Flag::Sub) {
            a = a.wrapping_sub(adjust);
        } else {
            a = a.wrapping_add(adjust);
        }
        self.registers.set_flag(Flag::Zero, a == 0);
        self.registers.set_flag(Flag::HalfCarry, false);
        self.registers.a = a;
    }

    #[inline(always)]
    fn res(&mut self, value: u8, bit: u8) -> u8 {
        value & !(1 << bit)
    }

    #[inline(always)]
    fn set(&mut self, value: u8, bit: u8) -> u8 {
        value | (1 << bit)
    }

    #[inline(always)]
    fn push_stack(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.memory.write_word(self.registers.sp, value);
    }

    #[inline(always)]
    fn pop_stack(&mut self) -> u16 {
        let value = self.memory.read_word(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        value
    }

    #[inline(always)]
    fn add_hl(&mut self, value: u16) {
        let hl = self.registers.hl();
        let result = hl.wrapping_add(value);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (((hl & 0xFFF) + (value & 0xFFF)) & 0x1000) == 0x1000);
        self.registers.set_flag(Flag::Carry, hl > 0xFFFF - value);
        self.registers.set_hl(result);
    }

    #[inline(always)]
    fn jr(&mut self) {
        let offset = self.fetch_byte() as i8 as u16;
        self.registers.pc = self.registers.pc.wrapping_add(offset);
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_reg_inc() {
        let mut cpu = CPU::new();
        let result = cpu.reg_inc(0x00);
        assert_eq!(result, 0x01);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);

        let result = cpu.reg_inc(0xFF);
        assert_eq!(result, 0x00);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
    }

    #[test]
    fn test_reg_dec() {
        let mut cpu = CPU::new();
        let result = cpu.reg_dec(0x01);
        assert_eq!(result, 0x00);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), true);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);

        let result = cpu.reg_dec(0x00);
        assert_eq!(result, 0xFF);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), true);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.add(0x02, false);
        assert_eq!(cpu.registers.a, 0x03);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        cpu.registers.a = 0xFF;
        cpu.add(0x01, false);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x03;
        cpu.sub(0x02, false);
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), true);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        cpu.registers.a = 0x00;
        cpu.sub(0x01, false);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), true);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b1010_1010;
        cpu.and(0b1100_1100);
        assert_eq!(cpu.registers.a, 0b1000_1000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }

    #[test]
    fn test_xor() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b1010_1010;
        cpu.xor(0b1100_1100);
        assert_eq!(cpu.registers.a, 0b0110_0110);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        cpu.registers.a = 0b0000_0000;
        cpu.xor(0b0000_0000);
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }

    #[test]
    fn test_or() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b1010_1010;
        cpu.or(0b1100_1100);
        assert_eq!(cpu.registers.a, 0b1110_1110);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        cpu.registers.a = 0b0000_0000;
        cpu.or(0b0000_0000);
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }   // Completed

    #[test]
    fn test_cp() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x03;
        cpu.cp(0x02);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), true);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        cpu.registers.a = 0x02;
        cpu.cp(0x02);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), true);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        cpu.registers.a = 0x02;
        cpu.cp(0x03);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), true);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);
    }

    #[test]
    fn test_rlc() {
        let mut cpu = CPU::new();
        let result = cpu.rlc(0b1000_0000);
        assert_eq!(result, 0b0000_0001);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);

        let result = cpu.rlc(0b0000_0001);
        assert_eq!(result, 0b0000_0010);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }

    #[test]
    fn test_rrc() {
        let mut cpu = CPU::new();
        let result = cpu.rrc(0b0000_0001);
        assert_eq!(result, 0b1000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);

        let result = cpu.rrc(0b1000_0000);
        assert_eq!(result, 0b0100_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }

    #[test]
    fn test_rl() {
        let mut cpu = CPU::new();
        cpu.registers.set_flag(Flag::Carry, true);
        let result = cpu.rl(0b1000_0000);
        assert_eq!(result, 0b0000_0001);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);

        let result = cpu.rl(0b0000_0001);
        assert_eq!(result, 0b0000_0011);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }

    #[test]
    fn test_rr() {
        let mut cpu = CPU::new();
        cpu.registers.set_flag(Flag::Carry, true);
        let result = cpu.rr(0b0000_0001);
        assert_eq!(result, 0b1000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);

        let result = cpu.rr(0b1000_0000);
        assert_eq!(result, 0b1100_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }

    #[test]
    fn test_sla() {
        let mut cpu = CPU::new();
        let result = cpu.sla(0b1000_0000);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);

        let result = cpu.sla(0b0000_0001);
        assert_eq!(result, 0b0000_0010);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }

    #[test]
    fn test_sra() {
        let mut cpu = CPU::new();
        let result = cpu.sra(0b1000_0000);
        assert_eq!(result, 0b1100_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        let result = cpu.sra(0b0000_0001);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);
    }

    #[test]
    fn test_swap() {
        let mut cpu = CPU::new();
        let result = cpu.swap(0b1001_0110);
        assert_eq!(result, 0b0110_1001);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        let result = cpu.swap(0b0000_0000);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);
    }   // Completed

    #[test]
    fn test_srl() {
        let mut cpu = CPU::new();
        let result = cpu.srl(0b1000_0000);
        assert_eq!(result, 0b0100_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        let result = cpu.srl(0b0000_0001);
        assert_eq!(result, 0b0000_0000);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);
    }   // Completed

    #[test]
    fn test_bit() {
        let mut cpu = CPU::new();
        cpu.bit(0b1010_1011, 3);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), false);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);

        cpu.bit(0b1010_1010, 2);
        assert_eq!(cpu.registers.get_flag(Flag::Zero), true);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
    }   // Completed

    #[test]
    fn test_res() {
        let mut cpu = CPU::new();
        let result = cpu.res(0b1010_1010, 3);
        assert_eq!(result, 0b1010_0010);

        let result = cpu.res(0b1010_1010, 2);
        assert_eq!(result, 0b1010_1010);
    }   // Completed

    #[test]
    fn test_set() {
        let mut cpu = CPU::new();
        let result = cpu.set(0b1010_1010, 4);
        assert_eq!(result, 0b1011_1010);

        let result = cpu.set(0b1010_1010, 3);
        assert_eq!(result, 0b1010_1010);
    }   // Completed

    #[test]
    fn test_push_stack() {
        let mut cpu = CPU::new();
        cpu.registers.sp = 0xFFFE;
        cpu.push_stack(0x1234);
        assert_eq!(cpu.memory.read_word(0xFFFC), 0x1234);
        assert_eq!(cpu.registers.sp, 0xFFFC);
    }

    #[test]
    fn test_pop_stack() {
        let mut cpu = CPU::new();
        cpu.memory.write_word(0xFFFC, 0x1234);
        cpu.registers.sp = 0xFFFC;
        let result = cpu.pop_stack();
        assert_eq!(result, 0x1234);
        assert_eq!(cpu.registers.sp, 0xFFFE);
    }

    #[test]
    fn test_add_hl() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0x1234);
        cpu.add_hl(0x5678);
        assert_eq!(cpu.registers.hl(), 0x68AC);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), false);

        cpu.registers.set_hl(0xFFFF);
        cpu.add_hl(0x0001);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_eq!(cpu.registers.get_flag(Flag::Sub), false);
        assert_eq!(cpu.registers.get_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.registers.get_flag(Flag::Carry), true);
    }

    #[test]
    fn test_jr() {
        let mut cpu = CPU::new();
        cpu.registers.pc = 0x8fbc;
        cpu.memory.write(0x8fbc, 0x03);
        cpu.jr();
        assert_eq!(cpu.registers.pc, 0x8fc0);
    }

}