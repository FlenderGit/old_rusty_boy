use crate::{memory::Memory, registers::Registers};

pub struct CPU {
    registers: Registers,
    memory: Memory,
}

impl CPU {
    
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

}