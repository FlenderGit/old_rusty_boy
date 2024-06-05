use crate::memory::Memory;
use crate::registers::{Registers, Flag};
use crate::instruction::{Instruction, InstructionType, JumpCondition, RegisterTarget, RegisterTarget16, LhdAction};

use std::cell::RefCell;
use std::rc::Rc;

pub struct CPU {
    pub registers: Registers,
    memory: Rc<RefCell<Memory>>,
    ticks: u64,
}

impl CPU {


    pub fn new(memory: Rc<RefCell<Memory>>) -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Rc::clone(&memory),
            ticks: 0,
        }
    }

    pub fn step(&mut self) {
        let optcode = self.memory.borrow_mut().fetch_byte();
        let instruction = Instruction::from(optcode);

        //println!("Pc: {:#06x} {:#06x} {:?}", self.memory.borrow().pc - 1, optcode , instruction);

        self.execute(instruction);
        self.ticks += instruction.ticks as u64;
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.itype {

            // Misc/control instructions
            InstructionType::NOP => {},
            InstructionType::DI => {
                self.memory.borrow_mut().interrupt_enable = 0;
            },
            

            // Jumps/calls
            InstructionType::JR(condition) => {
                let should_jump = match condition {
                    JumpCondition::NONE => true,
                    JumpCondition::NZ => !self.registers.get_flag(Flag::Zero),
                    JumpCondition::Z => self.registers.get_flag(Flag::Zero),
                    JumpCondition::NC => !self.registers.get_flag(Flag::Carry),
                    JumpCondition::C => self.registers.get_flag(Flag::Carry),
                    //_ => panic!("Invalid condition for JR"),
                };
                self.jump(should_jump, true);
            },

            InstructionType::JUMP(condition) => {
                let should_jump = match condition {
                    JumpCondition::NONE => true,
                    JumpCondition::NZ => !self.registers.get_flag(Flag::Zero),
                    JumpCondition::Z => self.registers.get_flag(Flag::Zero),
                    _ => panic!("Invalid condition for JUMP"),
                };
                self.jump_2(should_jump, false);
            },

            // 8bit load/store/move instructions
            InstructionType::LOAD11(targer, value) => {
                let value = self.get_register_value(value);
                self.set_register_value(targer, value);
            },
            InstructionType::LOAD12(target, value) => {
                let value = self.get_register_value16(value);
                self.set_register_value(target, value as u8);
            },
            InstructionType::LOAD21(target, value) => {
                let value = self.get_register_value(value);
                self.set_register_value16(target, value as u16);
            },
            InstructionType::LDH(instruction) => {
                let value = self.memory.borrow_mut().fetch_byte();
                let address = 0xff00 + value as u16;
                match instruction {
                    LhdAction::LOAD => {
                        self.registers.a = self.memory.borrow().read_byte(address);
                    },
                    LhdAction::SAVE => {
                        self.memory.borrow_mut().write_byte(address, self.registers.a);
                    },
                };
            },

            // 16bit load/store/move instructions
            InstructionType::LOAD22(target, value) => {
                let value = self.get_register_value16(value);
                self.set_register_value16(target, value);
            },


            // 8bit arithmetic/logical instructions
            InstructionType::INC(target) => {
                let value = self.get_register_value(target);
                let result = self.inc(value);
                self.set_register_value(target, result);
            },
            InstructionType::DEC(target) => {
                let value = self.get_register_value(target);
                let result = self.dec(value);
                self.set_register_value(target, result);
            },
            InstructionType::ADD(target, value) => {
                let value = self.get_register_value(value);
                let register = self.get_register_value(target);
                let result = self.add(register, value);
                self.set_register_value(target, result);
            },
            InstructionType::ADC(target, value) => {
                let value = self.get_register_value(value);
                let register = self.get_register_value(target);
                let carry = self.registers.get_flag(Flag::Carry);
                let result = self.add(register, value + carry as u8);
                self.set_register_value(target, result);
            },
            InstructionType::SUB(target) => {
                let value = self.get_register_value(target);
                let register = self.registers.a;
                let result = register.wrapping_sub(value);
                self.registers.a = result;
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, true);
                self.registers.set_flag(Flag::HalfCarry, (register & 0x0f) < (value & 0x0f));
                self.registers.set_flag(Flag::Carry, register < value);
            },
            InstructionType::AND(target, value) => {
                let value = self.get_register_value(value);
                let register = self.get_register_value(target);
                let result = register & value;
                self.set_register_value(target, result);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, false);
                self.registers.set_flag(Flag::HalfCarry, true);
                self.registers.set_flag(Flag::Carry, false);
            },
            InstructionType::XOR(target, value) => {
                let value = self.get_register_value(value);
                let register = self.get_register_value(target);
                let result = register ^ value;
                self.set_register_value(target, result);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, false);
                self.registers.set_flag(Flag::HalfCarry, false);
                self.registers.set_flag(Flag::Carry, false);
            },
            InstructionType::OR(target, value) => {
                let value = self.get_register_value(value);
                let register = self.get_register_value(target);
                let result = register | value;
                self.set_register_value(target, result);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, false);
                self.registers.set_flag(Flag::HalfCarry, false);
                self.registers.set_flag(Flag::Carry, false);
            },
            InstructionType::CP(target) => {
                let value = self.get_register_value(target);
                let register = self.registers.a;
                let result = register.wrapping_sub(value);
                self.registers.set_flag(Flag::Zero, result == 0);
                self.registers.set_flag(Flag::Sub, true);
                self.registers.set_flag(Flag::HalfCarry, (register & 0x0f) < (value & 0x0f));
                self.registers.set_flag(Flag::Carry, register < value);
            },

            // 16bit arithmetic/logical instructions
            InstructionType::INC2(target) => {
                let value = self.get_register_value16(target);
                let result = value.wrapping_add(1);
                self.set_register_value16(target, result);
            },
            InstructionType::DEC2(target) => {
                let value = self.get_register_value16(target);
                let result = value.wrapping_sub(1);
                self.set_register_value16(target, result);
            },

            _ => panic!("{:#06x} : Unimplemented instruction: {:?} ", self.memory.borrow().pc, instruction),
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

    fn add(&mut self, a: u8, b: u8) -> u8 {
        let result = a.wrapping_add(b);
        self.registers.set_flag(Flag::Zero, result == 0);
        self.registers.set_flag(Flag::Sub, false);
        self.registers.set_flag(Flag::HalfCarry, (a & 0x0f) + (b & 0x0f) > 0x0f);
        self.registers.set_flag(Flag::Carry, (a as u16 + b as u16) > 0xff);
        result
    }

    fn jump(&mut self, should_jump: bool, relative: bool) {
        let offset = self.memory.borrow_mut().fetch_byte();
        if should_jump {
            if relative {
                //self.log(&format!("Offset {}", (offset as i8).abs() as u16));
                self.memory.borrow_mut().pc -= (offset as i8).abs() as u16 ;
            } else {
                self.memory.borrow_mut().pc = offset as u16;
            }
            //self.log(&format!("Jumping to {:#04x}", self.registers.pc));
        }
    }
    

    fn jump_2(&mut self, should_jump: bool, relative: bool) {
        let offset = self.memory.borrow_mut().fetch_short();
        if should_jump {
            if relative {
                self.memory.borrow_mut().pc += offset as i8 as u16;
            } else {
                self.memory.borrow_mut().pc = offset;
            }
            //self.log(&format!("Jumping to {:#04x}", self.registers.pc));
        }
    }

    fn get_register_value(&self, target: RegisterTarget) -> u8 {
        match target {
            RegisterTarget::A => self.registers.a,
            RegisterTarget::B => self.registers.b,
            RegisterTarget::C => self.registers.c,
            RegisterTarget::D => self.registers.d,
            RegisterTarget::E => self.registers.e,
            RegisterTarget::H => self.registers.h,
            RegisterTarget::L => self.registers.l,
            RegisterTarget::INSTANT => self.memory.borrow_mut().fetch_byte(),
            _ => panic!("Invalid register target"),
        }
    }

    fn get_register_value16(&self, target: RegisterTarget16) -> u16 {
        match target {
            RegisterTarget16::BC => self.registers.bc(),
            RegisterTarget16::DE => self.registers.de(),
            RegisterTarget16::HL => self.registers.hl(),
            RegisterTarget16::SP => self.memory.borrow().sp,
            RegisterTarget16::INSTANT2 => self.memory.borrow_mut().fetch_short(),
            //_ => panic!("Invalid register target"),
        }
    }

    fn set_register_value(&mut self, target: RegisterTarget, value: u8) {
        match target {
            RegisterTarget::A => self.registers.a = value,
            RegisterTarget::B => self.registers.b = value,
            RegisterTarget::C => self.registers.c = value,
            RegisterTarget::D => self.registers.d = value,
            RegisterTarget::E => self.registers.e = value,
            RegisterTarget::H => self.registers.h = value,
            RegisterTarget::L => self.registers.l = value,
            _ => panic!("Invalid register target"),
        }
    }

    fn set_register_value16(&mut self, target: RegisterTarget16, value: u16) {
        match target {
            RegisterTarget16::BC => self.registers.set_bc(value),
            RegisterTarget16::DE => self.registers.set_de(value),
            RegisterTarget16::HL => self.registers.set_hl(value),
            RegisterTarget16::SP => self.memory.borrow_mut().sp = value,
            _ => panic!("Invalid register target"),
        }
    }


}