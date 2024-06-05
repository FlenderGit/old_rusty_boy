use crate::registers::{Registers, Flag};
use crate::instruction::{Instruction, InstructionType, JumpCondition, RegisterTarget, RegisterTarget16, LhdAction};

pub struct CPU {
    pub registers: Registers,
    bus: Vec<u8>,
    tick: u64,
    debug: bool,
}

impl CPU {

    pub fn new(rom : Vec<u8>) -> CPU {
        CPU {
            registers: Registers::new(),
            bus: rom,
            tick: 0,
            debug: true,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.bus[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.bus[address as usize] = value;
    }

    fn log(&self, message: &str) {
        if self.debug {
            println!("{}", message);
        }
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

    pub fn step(&mut self) {

        let opcode = self.fetch_byte();

        let instruction = Instruction::from(opcode);
        self.log(&format!("I: {:#06x} {:#04x} -> {}", self.registers.pc-1, opcode, instruction.name ));

        self.execute_instruction(instruction);
        

        self.tick += instruction.ticks as u64;

        //self.log(&format!("PC: {:#06x} OP: {:#04x} {:#04x}", self.registers.pc, opcode, operand_value));
        //self.execute(opcode);
        //self.tick += 1; // TD

    }

    pub fn get_value(&mut self, target: RegisterTarget) -> u8 {
        match target {
            RegisterTarget::A => self.registers.a,
            RegisterTarget::B => self.registers.b,
            RegisterTarget::C => self.registers.c,
            RegisterTarget::D => self.registers.d,
            RegisterTarget::E => self.registers.e,
            RegisterTarget::H => self.registers.h,
            RegisterTarget::L => self.registers.l,
            RegisterTarget::INSTANT => self.fetch_byte(),
            RegisterTarget::_HL => self.read_byte(self.registers.hl()),
        }
    }

    pub fn get_value_16(&mut self, target: RegisterTarget16) -> u16 {
        match target {
            RegisterTarget16::BC => self.registers.bc(),
            RegisterTarget16::DE => self.registers.de(),
            RegisterTarget16::HL => self.registers.hl(),
            RegisterTarget16::SP => self.registers.sp,
            RegisterTarget16::INSTANT2 => self.fetch_word(),
        }
    }



    pub fn set_value(&mut self, target: RegisterTarget, value: u8) {
        match target {
            RegisterTarget::A => self.registers.a = value,
            RegisterTarget::B => self.registers.b = value,
            RegisterTarget::C => self.registers.c = value,
            RegisterTarget::D => self.registers.d = value,
            RegisterTarget::E => self.registers.e = value,
            RegisterTarget::H => self.registers.h = value,
            RegisterTarget::L => self.registers.l = value,
            _ => panic!("Invalid target for set_value"),
        }
    }

    pub fn set_value_16(&mut self, target: RegisterTarget16, value: u16) {
        match target {
            RegisterTarget16::BC => self.registers.set_bc(value),
            RegisterTarget16::DE => self.registers.set_de(value),
            RegisterTarget16::HL => self.registers.set_hl(value),
            RegisterTarget16::SP => self.registers.sp = value,
            RegisterTarget16::INSTANT2 => { self.fetch_word(); },
            _ => panic!("Invalid target for set_value_16"),
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.itype {
            InstructionType::NOP => {},
            InstructionType::INC(target) => {
                let value = self.get_value(target);
                let result = self.inc(value);
                self.set_value(target, result);
            },
            InstructionType::DEC(target) => {
                let value = self.get_value(target);
                let result = self.dec(value);
                self.set_value(target, result);
            },
            
            // Load
            InstructionType::LOAD11(targer, value) => {
                let value = self.get_value(value);
                self.set_value(targer, value);
            },
            InstructionType::LOAD12(target, value) => {
                let value = self.get_value_16(value);
                self.set_value(target, value as u8);
            },
            InstructionType::LOAD21(target, value) => {
                let value = self.get_value(value);
                self.set_value_16(target, value as u16);
            },
            InstructionType::LOAD22(target, value) => {
                let value = self.get_value_16(value);
                self.set_value_16(target, value);
            },

            // ADD
            InstructionType::ADD(target, value) => {
                let value = self.get_value(value);
                let register = self.get_value(target);
                let result = self.add(register, value);
                self.set_value(target, result);
            },

            // SUB
            InstructionType::SUB(target) => {
                let value = self.get_value(target);
                let register = self.registers.a;
                let result = register.wrapping_sub(value);
                self.registers.a = result;
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, true);
                self.registers.set_flag(Flag::HalfCarry, (register & 0x0f) < (value & 0x0f));
                self.registers.set_flag(Flag::Carry, register < value);
            },

            // OR
            InstructionType::OR(target, value) => {
                let value = self.get_value(value);
                let register = self.get_value(target);
                let result = register | value;
                self.set_value(target, result);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, false);
                self.registers.set_flag(Flag::HalfCarry, false);
                self.registers.set_flag(Flag::Carry, false);
            },

            // XOR
            InstructionType::XOR(target, value) => {
                let value = self.get_value(value);
                let register = self.get_value(target);
                let result = register ^ value;
                self.set_value(target, result);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, false);
                self.registers.set_flag(Flag::HalfCarry, false);
                self.registers.set_flag(Flag::Carry, false);
            },

            // AND
            InstructionType::AND(target, value) => {
                let value = self.get_value(value);
                let register = self.get_value(target);
                let result = register & value;
                self.set_value(target, result);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, false);
                self.registers.set_flag(Flag::HalfCarry, true);
                self.registers.set_flag(Flag::Carry, false);
            },

            // INC2
            InstructionType::INC2(target) => {
                let value = self.get_value_16(target);
                let result = value.wrapping_add(1);
                self.set_value_16(target, result);
            },

            // DEC2
            InstructionType::DEC2(target) => {
                let value = self.get_value_16(target);
                let result = value.wrapping_sub(1);
                self.set_value_16(target, result);
            },

            // RET
            InstructionType::RET(flag) => {
                if flag == Flag::None || self.registers.get_flag(flag) {
                    self.registers.pc = self.fetch_word();
                    self.tick += 20;
                } else {
                    self.tick += 8;
                }
            },

            // POP
            InstructionType::POP(target) => {
                let value = self.read_word(self.registers.sp);
                self.set_value_16(target, value);
            },

            // JUMP
            InstructionType::JUMP(condition) => {
                let should_jump = match condition {
                    JumpCondition::NONE => true,
                    JumpCondition::NZ => !self.registers.get_flag(Flag::Zero),
                    JumpCondition::Z => self.registers.get_flag(Flag::Zero),
                    //Flag::Carry => self.registers.get_flag(Flag::Carry),
                    //Flag::HalfCarry => !self.registers.get_flag(Flag::HalfCarry),
                    //Flag::Sub => !self.registers.get_flag(Flag::Sub),
                    _ => panic!("Invalid condition for JUMP"),
                };
                self.jump_2(should_jump, false);
            },

            InstructionType::RST(value) => {
                self.write_short_to_stack(self.registers.pc);
                self.registers.pc = value;
            },

            InstructionType::ADC(target, value) => {
                let value = self.get_value(value);
                let register = self.get_value(target);
                let carry = self.registers.get_flag(Flag::Carry);
                let result = self.add(register, value + carry as u8);
                self.set_value(target, result);
            },

            // LDH
            InstructionType::LDH(instruction) => {
                let value = self.fetch_byte();
                let address = 0xff00 + value as u16;
                match instruction {
                    LhdAction::LOAD => {
                        self.registers.a = self.read_byte(address);
                    },
                    LhdAction::SAVE => {
                        self.write_byte(address, self.registers.a);
                    },
                };
                //self.log(&format!("LDH {:?}: {:#06x} {:#06x}", instruction, address, self.read_byte(address)));
            },

            // CALL
            InstructionType::CALL(condition) => {
                match condition {
                    JumpCondition::NONE => {
                        let address = self.fetch_word();
                        self.write_short_to_stack(self.registers.pc);
                        self.registers.pc = address;
                    },
                    _ => panic!("Invalid condition for CALL"),
                }
            },

            // CP
            InstructionType::CP(target) => {
                let value = self.get_value(target);
                let register = self.registers.a;
                let result = register.wrapping_sub(value);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, true);
                self.registers.set_flag(Flag::HalfCarry, (register & 0x0f) < (value & 0x0f));
                self.registers.set_flag(Flag::Carry, register < value);
            },

            // JR
            InstructionType::JR(condition) => {
                let should_jump = match condition {
                    JumpCondition::NONE => true,
                    JumpCondition::NZ => !self.registers.get_flag(Flag::Zero),
                    JumpCondition::Z => self.registers.get_flag(Flag::Zero),
                    JumpCondition::NC => !self.registers.get_flag(Flag::Carry),
                    JumpCondition::C => self.registers.get_flag(Flag::Carry),
                    _ => panic!("Invalid condition for JR"),
                };
                self.jump(should_jump, true);
            },

            InstructionType::PREFIX_CB => {
                let opcode = self.fetch_byte();
                let instruction = Instruction::from_cb(opcode);
                self.log(&format!("PC: {:#06x} {}:\t{:#04x}", self.registers.pc-1, instruction.name , opcode));
                self.execute_instruction(instruction);
            },

            InstructionType::RES(value, target) => {
                let register = self.get_value(target);
                let result = register & !(1 << value);
                self.set_value(target, result);
            }

            _ => panic!("Unimplemented instruction: {:?}", instruction),
        }
    }

    fn add(&mut self, a: u8, b: u8) -> u8 {
        let result = a.wrapping_add(b);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (a & 0x0f) + (b & 0x0f) > 0x0f);
        self.registers.set_flag(Flag::Carry, (a as u16 + b as u16) > 0xff);
        result
    }

    fn add2(&mut self, destination: &mut u16, value: u16) {
        let result = destination.wrapping_add(value);
        
        // Must be dynamic, destination is a reference
        *destination = result;

        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (*destination & 0x0fff) + (value & 0x0fff) > 0x0fff);
        self.registers.set_flag(Flag::Carry, (*destination as u32 + value as u32) > 0xffff);
    }

    
    
    
    
    
    
    
    
    
    pub fn execute(&mut self, opcode: u8) {
        match opcode {

            // First bar
            // NOP
            0x00 => {},
            // LD BC, d16
            0x01 => { 
                let value = self.fetch_word();
                self.registers.set_bc(value);
            },
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
                self.registers.set_flag(Flag::Carry, carry != 0);
                self.registers.set_flag(Flag::Zero, false);
                self.registers.set_flag(Flag::HalfCarry, false);
                self.registers.set_flag(Flag::Sub, false);
            },
            // LD (a16), SP
            0x08 => {
                let address = self.fetch_word();
                self.write_word(address,
                self.registers.sp);
            },
            // ADD HL, BC
            0x09 => { self.add2(&mut self.registers.hl(), self.registers.bc()); },
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
                self.registers.set_flag(Flag::Carry, carry != 0);
                self.registers.set_flag(Flag::Zero, false);
                self.registers.set_flag(Flag::HalfCarry, false);
                self.registers.set_flag(Flag::Sub, false);
            },

            // Second bar



            // Twelve bar
            0xc3 => {
                let address = self.fetch_word();
                self.registers.pc = address;
            },

            0xcd => {
                let address = self.fetch_word();
                self.write_short_to_stack(self.registers.pc);
                self.registers.pc = address;
            },



            _ => panic!("Unimplemented opcode: {:#04x}", opcode),
        }
    }

    

    fn inc(&mut self, v: u8) -> u8 {
        let result = v.wrapping_add(1);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (v & 0x0f) + 1 > 0x0f);
        result
    }

    fn dec(&mut self, v: u8) -> u8 {
        let result = v.wrapping_sub(1);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, true);
        self.registers.set_flag(Flag::HalfCarry, (v & 0x0f) == 0);
        result
    }

    fn jump(&mut self, should_jump: bool, relative: bool) {
        let offset = self.fetch_byte();
        if should_jump {
            if relative {
                //self.log(&format!("Offset {}", (offset as i8).abs() as u16));
                self.registers.pc -= (offset as i8).abs() as u16 ;
            } else {
                self.registers.pc = offset as u16;
            }
            self.log(&format!("Jumping to {:#04x}", self.registers.pc));
        }
    }

    fn jump_2(&mut self, should_jump: bool, relative: bool) {
        let offset = self.fetch_word();
        if should_jump {
            if relative {
                self.registers.pc += offset as i8 as u16;
            } else {
                self.registers.pc = offset;
            }
            self.log(&format!("Jumping to {:#04x}", self.registers.pc));
        }
    }

    fn write_short_to_stack(&mut self, value: u16) {
        self.registers.sp -= 2;
        self.write_word(self.registers.sp, value);
        self.log(&format!("SP: {:#06x} Writing {:#06x} to stack", self.registers.sp, value));
    }

}