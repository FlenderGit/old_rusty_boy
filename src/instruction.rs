
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
    _HL,
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
    JUMP(JumpCondition),

    // 8bit load/store/move instructions
    LOAD11(RegisterTarget, RegisterTarget),
    LOAD12(RegisterTarget, RegisterTarget16),
    LOAD21(RegisterTarget16, RegisterTarget),


    // 16bit load/store/move instructions
    LOAD22(RegisterTarget16, RegisterTarget16),

    // 8bit arithmetic/logical instructions
    INC(RegisterTarget),
    DEC(RegisterTarget),
    ADD(RegisterTarget, RegisterTarget),
    ADC(RegisterTarget, RegisterTarget),
    SUB(RegisterTarget, RegisterTarget),
    SBC(RegisterTarget, RegisterTarget),
    AND(RegisterTarget, RegisterTarget),
    XOR(RegisterTarget, RegisterTarget),
    OR(RegisterTarget, RegisterTarget),
    CP(RegisterTarget, RegisterTarget),
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

}

use InstructionType::*;
use RegisterTarget::*;
use RegisterTarget16::*;

//const list_instruction: [Instruction; 256]
const list_instruction: [Instruction; 199] = [

    // First line
    Instruction { name: "NOP", itype: NOP, ticks: 4 },
    Instruction { name: "LD BC, nn", itype: LOAD22(BC, INSTANT2), ticks: 12 },
    Instruction { name: "LD (BC), A", itype: LOAD21(BC, A), ticks: 8 },
    Instruction { name: "INC BC", itype: INC2(BC), ticks: 4 },
    Instruction { name: "INC B", itype: INC(B), ticks: 4 },
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

    // Sixth line
    Instruction { name: "LD D, B", itype: LOAD11(D, B), ticks: 4 },
    Instruction { name: "LD D, C", itype: LOAD11(D, C), ticks: 4 },
    Instruction { name: "LD D, D", itype: LOAD11(D, D), ticks: 4 },
    Instruction { name: "LD D, E", itype: LOAD11(D, E), ticks: 4 },
    Instruction { name: "LD D, H", itype: LOAD11(D, H), ticks: 4 },
    Instruction { name: "LD D, L", itype: LOAD11(D, L), ticks: 4 },
    Instruction { name: "LD D, (HL)", itype: LOAD12(D, HL), ticks: 8 },
    Instruction { name: "LD D, A", itype: LOAD11(D, A), ticks: 4 },
    Instruction { name: "LD E, B", itype: LOAD11(E, B), ticks: 4 },
    Instruction { name: "LD E, C", itype: LOAD11(E, C), ticks: 4 },
    Instruction { name: "LD E, D", itype: LOAD11(E, D), ticks: 4 },
    Instruction { name: "LD E, E", itype: LOAD11(E, E), ticks: 4 },
    Instruction { name: "LD E, H", itype: LOAD11(E, H), ticks: 4 },
    Instruction { name: "LD E, L", itype: LOAD11(E, L), ticks: 4 },
    Instruction { name: "LD E, (HL)", itype: LOAD12(E, HL), ticks: 8 },
    Instruction { name: "LD E, A", itype: LOAD11(E, A), ticks: 4 },

    // Seventh line
    Instruction { name: "LD H, B", itype: LOAD11(H, B), ticks: 4 },
    Instruction { name: "LD H, C", itype: LOAD11(H, C), ticks: 4 },
    Instruction { name: "LD H, D", itype: LOAD11(H, D), ticks: 4 },
    Instruction { name: "LD H, E", itype: LOAD11(H, E), ticks: 4 },
    Instruction { name: "LD H, H", itype: LOAD11(H, H), ticks: 4 },
    Instruction { name: "LD H, L", itype: LOAD11(H, L), ticks: 4 },
    Instruction { name: "LD H, (HL)", itype: LOAD12(H, HL), ticks: 8 },
    Instruction { name: "LD H, A", itype: LOAD11(H, A), ticks: 4 },
    Instruction { name: "LD L, B", itype: LOAD11(L, B), ticks: 4 },
    Instruction { name: "LD L, C", itype: LOAD11(L, C), ticks: 4 },
    Instruction { name: "LD L, D", itype: LOAD11(L, D), ticks: 4 },
    Instruction { name: "LD L, E", itype: LOAD11(L, E), ticks: 4 },
    Instruction { name: "LD L, H", itype: LOAD11(L, H), ticks: 4 },
    Instruction { name: "LD L, L", itype: LOAD11(L, L), ticks: 4 },
    Instruction { name: "LD L, (HL)", itype: LOAD12(L, HL), ticks: 8 },
    Instruction { name: "LD L, A", itype: LOAD11(L, A), ticks: 4 },

    // Eighth line
    Instruction { name: "LD (HL), B", itype: LOAD21(HL, B), ticks: 8 },
    Instruction { name: "LD (HL), C", itype: LOAD21(HL, C), ticks: 8 },
    Instruction { name: "LD (HL), D", itype: LOAD21(HL, D), ticks: 8 },
    Instruction { name: "LD (HL), E", itype: LOAD21(HL, E), ticks: 8 },
    Instruction { name: "LD (HL), H", itype: LOAD21(HL, H), ticks: 8 },
    Instruction { name: "LD (HL), L", itype: LOAD21(HL, L), ticks: 8 },
    Instruction { name: "HALT", itype: HALT, ticks: 4 },
    Instruction { name: "LD (HL), A", itype: LOAD21(HL, A), ticks: 8 },
    Instruction { name: "LD A, B", itype: LOAD11(A, B), ticks: 4 },
    Instruction { name: "LD A, C", itype: LOAD11(A, C), ticks: 4 },
    Instruction { name: "LD A, D", itype: LOAD11(A, D), ticks: 4 },
    Instruction { name: "LD A, E", itype: LOAD11(A, E), ticks: 4 },
    Instruction { name: "LD A, H", itype: LOAD11(A, H), ticks: 4 },
    Instruction { name: "LD A, L", itype: LOAD11(A, L), ticks: 4 },
    Instruction { name: "LD A, (HL)", itype: LOAD12(A, HL), ticks: 8 },
    Instruction { name: "LD A, A", itype: LOAD11(A, A), ticks: 4 },

    // Ninth line
    Instruction { name: "ADD A, B", itype: ADD(A, B), ticks: 4 },
    Instruction { name: "ADD A, C", itype: ADD(A, C), ticks: 4 },
    Instruction { name: "ADD A, D", itype: ADD(A, D), ticks: 4 },
    Instruction { name: "ADD A, E", itype: ADD(A, E), ticks: 4 },
    Instruction { name: "ADD A, H", itype: ADD(A, H), ticks: 4 },
    Instruction { name: "ADD A, L", itype: ADD(A, L), ticks: 4 },
    Instruction { name: "ADD A, (HL)", itype: ADD(A, _HL), ticks: 8 },
    Instruction { name: "ADD A, A", itype: ADD(A, A), ticks: 4 },
    Instruction { name: "ADC A, B", itype: ADC(A, B), ticks: 4 },
    Instruction { name: "ADC A, C", itype: ADC(A, C), ticks: 4 },
    Instruction { name: "ADC A, D", itype: ADC(A, D), ticks: 4 },
    Instruction { name: "ADC A, E", itype: ADC(A, E), ticks: 4 },
    Instruction { name: "ADC A, H", itype: ADC(A, H), ticks: 4 },
    Instruction { name: "ADC A, L", itype: ADC(A, L), ticks: 4 },
    Instruction { name: "ADC A, (HL)", itype: ADC(A, _HL), ticks: 8 },
    Instruction { name: "ADC A, A", itype: ADC(A, A), ticks: 4 },

    // Tenth line
    Instruction { name: "SUB B", itype: SUB(A, B), ticks: 4 },
    Instruction { name: "SUB C", itype: SUB(A, C), ticks: 4 },
    Instruction { name: "SUB D", itype: SUB(A, D), ticks: 4 },
    Instruction { name: "SUB E", itype: SUB(A, E), ticks: 4 },
    Instruction { name: "SUB H", itype: SUB(A, H), ticks: 4 },
    Instruction { name: "SUB L", itype: SUB(A, L), ticks: 4 },
    Instruction { name: "SUB (HL)", itype: SUB(A, _HL), ticks: 8 },
    Instruction { name: "SUB A", itype: SUB(A, A), ticks: 4 },
    Instruction { name: "SBC A, B", itype: SBC(A, B), ticks: 4 },
    Instruction { name: "SBC A, C", itype: SBC(A, C), ticks: 4 },
    Instruction { name: "SBC A, D", itype: SBC(A, D), ticks: 4 },
    Instruction { name: "SBC A, E", itype: SBC(A, E), ticks: 4 },
    Instruction { name: "SBC A, H", itype: SBC(A, H), ticks: 4 },
    Instruction { name: "SBC A, L", itype: SBC(A, L), ticks: 4 },
    Instruction { name: "SBC A, (HL)", itype: SBC(A, _HL), ticks: 8 },
    Instruction { name: "SBC A, A", itype: SBC(A, A), ticks: 4 },

    // Eleventh line
    Instruction { name: "AND B", itype: AND(A, B), ticks: 4 },
    Instruction { name: "AND C", itype: AND(A, C), ticks: 4 },
    Instruction { name: "AND D", itype: AND(A, D), ticks: 4 },
    Instruction { name: "AND E", itype: AND(A, E), ticks: 4 },
    Instruction { name: "AND H", itype: AND(A, H), ticks: 4 },
    Instruction { name: "AND L", itype: AND(A, L), ticks: 4 },
    Instruction { name: "AND (HL)", itype: AND(A, _HL), ticks: 8 },
    Instruction { name: "AND A", itype: AND(A, A), ticks: 4 },
    Instruction { name: "XOR B", itype: XOR(A, B), ticks: 4 },
    Instruction { name: "XOR C", itype: XOR(A, C), ticks: 4 },
    Instruction { name: "XOR D", itype: XOR(A, D), ticks: 4 },
    Instruction { name: "XOR E", itype: XOR(A, E), ticks: 4 },
    Instruction { name: "XOR H", itype: XOR(A, H), ticks: 4 },
    Instruction { name: "XOR L", itype: XOR(A, L), ticks: 4 },
    Instruction { name: "XOR (HL)", itype: XOR(A, _HL), ticks: 8 },
    Instruction { name: "XOR A", itype: XOR(A, A), ticks: 4 },

    // Twelfth line
    Instruction { name: "OR B", itype: OR(A, B), ticks: 4 },
    Instruction { name: "OR C", itype: OR(A, C), ticks: 4 },
    Instruction { name: "OR D", itype: OR(A, D), ticks: 4 },
    Instruction { name: "OR E", itype: OR(A, E), ticks: 4 },
    Instruction { name: "OR H", itype: OR(A, H), ticks: 4 },
    Instruction { name: "OR L", itype: OR(A, L), ticks: 4 },
    Instruction { name: "OR (HL)", itype: OR(A, _HL), ticks: 8 },
    Instruction { name: "OR A", itype: OR(A, A), ticks: 4 },
    Instruction { name: "CP B", itype: CP(A, B), ticks: 4 },
    Instruction { name: "CP C", itype: CP(A, C), ticks: 4 },
    Instruction { name: "CP D", itype: CP(A, D), ticks: 4 },
    Instruction { name: "CP E", itype: CP(A, E), ticks: 4 },
    Instruction { name: "CP H", itype: CP(A, H), ticks: 4 },
    Instruction { name: "CP L", itype: CP(A, L), ticks: 4 },
    Instruction { name: "CP (HL)", itype: CP(A, _HL), ticks: 8 },
    Instruction { name: "CP A", itype: CP(A, A), ticks: 4 },

    // Thirteenth line
    Instruction { name: "RET NZ", itype: NOP, ticks: 8 },
    Instruction { name: "POP BC", itype: NOP, ticks: 12 },
    Instruction { name: "JP NZ, nn", itype: NOP, ticks: 12 },
    Instruction { name: "JP nn", itype: JUMP(JumpCondition::NONE), ticks: 16 },
    Instruction { name: "CALL NZ, nn", itype: NOP, ticks: 12 },
    Instruction { name: "PUSH BC", itype: NOP, ticks: 16 },
    Instruction { name: "ADD A, n", itype: NOP, ticks: 8 },

];

