
#[derive(Copy, Debug, Clone)]
pub struct Instruction {    
    pub name: &'static str,
    pub ticks: u8,
    pub itype: InstructionType,
}

impl Instruction {
 
    pub fn from(opcode: u8) -> Instruction {
        // Security check
        if opcode as usize >= list_instruction.len() {
            panic!("Unknown opcode: {:02X}", opcode);
        }
        list_instruction[opcode as usize]
    }

}

#[derive(Debug, Clone, Copy)]
pub enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    //HL,
    INSTANT,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterTarget16 {
    BC,
    DE,
    HL,
    SP,
    INSTANT2,
}

#[derive(Debug, Clone, Copy)]
pub enum JumpCondition {
    NZ,
    Z,
    NC,
    C,
    NONE,
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionType {
    
    // Misc/control instructions
    NOP,
    STOP,
    HALT,
    PREFIX_CB,
    DI,
    EI,

    // Jumps/calls
    JR(JumpCondition, RegisterTarget),

    // 8bit load/store/move instructions
    LOAD11(RegisterTarget, RegisterTarget),
    LOAD12(RegisterTarget, RegisterTarget16),
    LOAD21(RegisterTarget16, RegisterTarget),


    // 16bit load/store/move instructions
    LOAD22(RegisterTarget16, RegisterTarget16),

    // 8bit arithmetic/logical instructions
    INC(RegisterTarget),
    DEC(RegisterTarget),
    DDA,
    CPL,
    SCF,
    CCF,

    // 16bit arithmetic/logical instructions
    INC2(RegisterTarget16),
    DEC2(RegisterTarget16),
    ADD22 (RegisterTarget16, RegisterTarget16),


    // 8bit rotations/shifts and bit instructions
    RLCA,
    RRCA,
    RRA,
    


    // Not implemented
    //SUB(RegisterTarget),
    //AND(RegisterTarget),
    //JUMP(JumpCondition),
    ADD(RegisterTarget, RegisterTarget),

}

use InstructionType::*;
use RegisterTarget::*;
use RegisterTarget16::*;

//const list_instruction: [Instruction; 256]
const list_instruction: [Instruction; 79] = [

    // First line
    Instruction { name: "NOP", itype: NOP, ticks: 4 },
    Instruction { name: "LD BC, nn", itype: LOAD22(BC, INSTANT2), ticks: 12 },
    Instruction { name: "LD (BC), A", itype: LOAD21(BC, A), ticks: 8 },
    Instruction { name: "INC BC", itype: INC2(BC), ticks: 4 },
    Instruction { name: "DEC B", itype: DEC(B), ticks: 4 },
    Instruction { name: "LD B, n", itype: LOAD11(B, INSTANT), ticks: 8 },
    Instruction { name: "RLCA", itype: RLCA, ticks: 4 },
    Instruction { name: "LD (nn), SP", itype: LOAD22(INSTANT2, SP), ticks: 20 },
    Instruction { name: "ADD HL, BC", itype: ADD22(HL, BC), ticks: 8 },
    Instruction { name: "LD A, (BC)", itype: LOAD12(A, BC), ticks: 8 },
    Instruction { name: "DEC BC", itype: DEC(B), ticks: 8 },
    Instruction { name: "INC C", itype: INC(C), ticks: 4 },
    Instruction { name: "DEC C", itype: DEC(C), ticks: 4 },
    Instruction { name: "LD C, n", itype: LOAD11(C, INSTANT), ticks: 8 },
    Instruction { name: "RRCA", itype: RRCA, ticks: 4 },

    // Second line
    Instruction { name: "STOP", itype: STOP, ticks: 4 },
    Instruction { name: "LD DE, nn", itype: LOAD22(DE, INSTANT2), ticks: 12 },
    Instruction { name: "LD (DE), A", itype: LOAD21(DE, A), ticks: 8 },
    Instruction { name: "INC DE", itype: INC2(DE), ticks: 4 },
    Instruction { name: "INC D", itype: INC(D), ticks: 4 },
    Instruction { name: "DEC D", itype: DEC(D), ticks: 4 },
    Instruction { name: "LD D, n", itype: LOAD11(D, INSTANT), ticks: 8 },
    Instruction { name: "RLA", itype: RLCA, ticks: 4 },
    Instruction { name: "JR n", itype: JR(JumpCondition::NONE, INSTANT), ticks: 4 },
    Instruction { name: "ADD HL, DE", itype: ADD22(HL, DE), ticks: 8 },
    Instruction { name: "LD A, (DE)", itype: LOAD12(A, DE), ticks: 8 },
    Instruction { name: "DEC DE", itype: DEC2(DE), ticks: 8 },
    Instruction { name: "INC E", itype: INC(E), ticks: 4 },
    Instruction { name: "DEC E", itype: DEC(E), ticks: 4 },
    Instruction { name: "LD E, n", itype: LOAD11(E, INSTANT), ticks: 8 },
    Instruction { name: "RRA", itype: RRA, ticks: 4 },

    // Third line
    Instruction { name: "JR NZ, n", itype: JR(JumpCondition::NZ, INSTANT), ticks: 8 },
    Instruction { name: "LD HL, nn", itype: LOAD22(HL, INSTANT2), ticks: 12 },
    Instruction { name: "LD (HL+), A", itype: LOAD21(HL, A), ticks: 8 },
    Instruction { name: "INC HL", itype: INC2(HL), ticks: 4 },
    Instruction { name: "INC H", itype: INC(H), ticks: 4 },
    Instruction { name: "DEC H", itype: DEC(H), ticks: 4 },
    Instruction { name: "LD H, n", itype: LOAD11(H, INSTANT), ticks: 8 },
    Instruction { name: "DAA", itype: DDA, ticks: 4 },
    Instruction { name: "JR Z, n", itype: JR(JumpCondition::Z, INSTANT), ticks: 8 },
    Instruction { name: "ADD HL, HL", itype: ADD22(HL, HL), ticks: 8 },
    Instruction { name: "LD A, (HL+)", itype: LOAD12(A, HL), ticks: 8 },
    Instruction { name: "DEC HL", itype: DEC2(HL), ticks: 8 },
    Instruction { name: "INC L", itype: INC(L), ticks: 4 },
    Instruction { name: "DEC L", itype: DEC(L), ticks: 4 },
    Instruction { name: "LD L, n", itype: LOAD11(L, INSTANT), ticks: 8 },
    Instruction { name: "CPL", itype: CPL, ticks: 4 },

    // Fourth line
    Instruction { name: "JR NC, n", itype: JR(JumpCondition::NC, INSTANT), ticks: 8 },
    Instruction { name: "LD SP, nn", itype: LOAD22(SP, INSTANT2), ticks: 12 },
    Instruction { name: "LD (HL-), A", itype: LOAD21(HL, A), ticks: 8 },
    Instruction { name: "INC SP", itype: INC2(SP), ticks: 4 },
    Instruction { name: "INC (HL)", itype: INC2(HL), ticks: 12 },
    Instruction { name: "DEC (HL)", itype: DEC2(HL), ticks: 12 },
    Instruction { name: "LD (HL), n", itype: LOAD21(HL, INSTANT), ticks: 12 },
    Instruction { name: "SCF", itype: SCF, ticks: 4 },
    Instruction { name: "JR C, n", itype: JR(JumpCondition::C, INSTANT), ticks: 8 },
    Instruction { name: "ADD HL, SP", itype: ADD22(HL, SP), ticks: 8 },
    Instruction { name: "LD A, (HL-)", itype: LOAD12(A, HL), ticks: 8 },
    Instruction { name: "DEC SP", itype: DEC2(SP), ticks: 8 },
    Instruction { name: "INC A", itype: INC(A), ticks: 4 },
    Instruction { name: "DEC A", itype: DEC(A), ticks: 4 },
    Instruction { name: "LD A, n", itype: LOAD11(A, INSTANT), ticks: 8 },
    Instruction { name: "CCF", itype: CCF, ticks: 4 },

    // Fifth line
    Instruction { name: "LD B, B", itype: LOAD11(B, B), ticks: 4 },
    Instruction { name: "LD B, C", itype: LOAD11(B, C), ticks: 4 },
    Instruction { name: "LD B, D", itype: LOAD11(B, D), ticks: 4 },
    Instruction { name: "LD B, E", itype: LOAD11(B, E), ticks: 4 },
    Instruction { name: "LD B, H", itype: LOAD11(B, H), ticks: 4 },
    Instruction { name: "LD B, L", itype: LOAD11(B, L), ticks: 4 },
    Instruction { name: "LD B, (HL)", itype: LOAD12(B, HL), ticks: 8 },
    Instruction { name: "LD B, A", itype: LOAD11(B, A), ticks: 4 },
    Instruction { name: "LD C, B", itype: LOAD11(C, B), ticks: 4 },
    Instruction { name: "LD C, C", itype: LOAD11(C, C), ticks: 4 },
    Instruction { name: "LD C, D", itype: LOAD11(C, D), ticks: 4 },
    Instruction { name: "LD C, E", itype: LOAD11(C, E), ticks: 4 },
    Instruction { name: "LD C, H", itype: LOAD11(C, H), ticks: 4 },
    Instruction { name: "LD C, L", itype: LOAD11(C, L), ticks: 4 },
    Instruction { name: "LD C, (HL)", itype: LOAD12(C, HL), ticks: 8 },
    Instruction { name: "LD C, A", itype: LOAD11(C, A), ticks: 4 },
];

