use crate::registers::Flag;

#[derive(Copy, Debug, Clone)]
pub struct Instruction {    
    pub name: &'static str,
    pub ticks: u8,
    pub itype: InstructionType,
}

impl Instruction {
 
    pub fn from(opcode: u8) -> Instruction {
        // Security check
        if opcode as usize > 0xff {
            panic!("Unknown opcode: {:02X}", opcode);
        }
        LIST_INSTRUCTION[opcode as usize]
    }

    pub fn from_cb(opcode: u8) -> Instruction {
        // Security check
        if opcode as usize > 0xff {
            panic!("Unknown opcode: {:02X}", opcode);
        }
        LIST_INSTRUCTION_CB[opcode as usize]
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
pub enum LdAction {
    SAVE,
    LOAD,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    INCREMENTATION,
    DECREMENTATION,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterTarget16 {
    BC,
    DE,
    HL,
    SP,
    AF,
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
    #[allow(non_camel_case_types)]
    PREFIX_CB,
    DI,
    EI,

    // Jumps/calls
    JR(JumpCondition),
    JUMP(JumpCondition),
    JP_HL,
    CALL(JumpCondition),
    RET(Flag),
    RST(u16),

    // 8bit load/store/move instructions
    LOAD11(RegisterTarget, RegisterTarget),
    LOAD12(RegisterTarget, RegisterTarget16),
    LOAD21(RegisterTarget16, RegisterTarget),
    LDH(LdAction, RegisterTarget),
    LD(LdAction, Operation),                    // LDI & LDD

    // 16bit load/store/move instructions
    LOAD22(RegisterTarget16, RegisterTarget16),
    POP(RegisterTarget16),
    PUSH(RegisterTarget16),

    // 8bit arithmetic/logical instructions
    INC(RegisterTarget),
    DEC(RegisterTarget),
    ADD(RegisterTarget, RegisterTarget),
    ADC(RegisterTarget, RegisterTarget),
    SUB(RegisterTarget),
    SBC(RegisterTarget, RegisterTarget),
    AND(RegisterTarget, RegisterTarget),
    XOR(RegisterTarget, RegisterTarget),
    OR(RegisterTarget, RegisterTarget),
    CP(RegisterTarget),
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

    // prefixed
    RLC(RegisterTarget),
    RRC(RegisterTarget),
    RL(RegisterTarget),
    RR(RegisterTarget),
    SLA(RegisterTarget),
    SRA(RegisterTarget),
    SWAP(RegisterTarget),
    SRL(RegisterTarget),
    BIT(u8, RegisterTarget),
    RES(u8, RegisterTarget),
    SET(u8, RegisterTarget),
    


    // Not implemented
    //SUB(RegisterTarget),
    //AND(RegisterTarget),
    //JUMP(JumpCondition),
    #[allow(non_camel_case_types)]
    NOT_IMPLEMENTED,

}

use InstructionType::*;
use RegisterTarget::*;
use RegisterTarget16::*;
use LdAction::*;

const LIST_INSTRUCTION: [Instruction; 256] = [

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
    Instruction { name: "DEC BC", itype: DEC2(BC), ticks: 8 },
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
    Instruction { name: "JR n", itype: JR(JumpCondition::NONE), ticks: 4 },
    Instruction { name: "ADD HL, DE", itype: ADD22(HL, DE), ticks: 8 },
    Instruction { name: "LD A, (DE)", itype: LOAD12(A, DE), ticks: 8 },
    Instruction { name: "DEC DE", itype: DEC2(DE), ticks: 8 },
    Instruction { name: "INC E", itype: INC(E), ticks: 4 },
    Instruction { name: "DEC E", itype: DEC(E), ticks: 4 },
    Instruction { name: "LD E, n", itype: LOAD11(E, INSTANT), ticks: 8 },
    Instruction { name: "RRA", itype: RRA, ticks: 4 },

    // Third line
    Instruction { name: "JR NZ, n", itype: JR(JumpCondition::NZ), ticks: 8 },
    Instruction { name: "LD HL, nn", itype: LOAD22(HL, INSTANT2), ticks: 12 },
    Instruction { name: "LDI (HL), A", itype: LD(SAVE, Operation::INCREMENTATION), ticks: 8 },
    Instruction { name: "INC HL", itype: INC2(HL), ticks: 4 },
    Instruction { name: "INC H", itype: INC(H), ticks: 4 },
    Instruction { name: "DEC H", itype: DEC(H), ticks: 4 },
    Instruction { name: "LD H, n", itype: LOAD11(H, INSTANT), ticks: 8 },
    Instruction { name: "DAA", itype: DDA, ticks: 4 },
    Instruction { name: "JR Z, n", itype: JR(JumpCondition::Z), ticks: 8 },
    Instruction { name: "ADD HL, HL", itype: ADD22(HL, HL), ticks: 8 },
    Instruction { name: "LD A, (HL)", itype: LD(LOAD, Operation::INCREMENTATION), ticks: 8 },
    Instruction { name: "DEC HL", itype: DEC2(HL), ticks: 8 },
    Instruction { name: "INC L", itype: INC(L), ticks: 4 },
    Instruction { name: "DEC L", itype: DEC(L), ticks: 4 },
    Instruction { name: "LD L, n", itype: LOAD11(L, INSTANT), ticks: 8 },
    Instruction { name: "CPL", itype: CPL, ticks: 4 },

    // Fourth line
    Instruction { name: "JR NC, n", itype: JR(JumpCondition::NC), ticks: 8 },
    Instruction { name: "LD SP, nn", itype: LOAD22(SP, INSTANT2), ticks: 12 },
    Instruction { name: "LDD (HL), A", itype: LD(SAVE, Operation::DECREMENTATION), ticks: 8 },
    Instruction { name: "INC SP", itype: INC2(SP), ticks: 4 },
    Instruction { name: "INC (HL)", itype: INC2(HL), ticks: 12 },
    Instruction { name: "DEC (HL)", itype: DEC2(HL), ticks: 12 },
    Instruction { name: "LD (HL), n", itype: LOAD21(HL, INSTANT), ticks: 12 },
    Instruction { name: "SCF", itype: SCF, ticks: 4 },
    Instruction { name: "JR C, n", itype: JR(JumpCondition::C), ticks: 8 },
    Instruction { name: "ADD HL, SP", itype: ADD22(HL, SP), ticks: 8 },
    Instruction { name: "LDD A, (HL)", itype: LD(LOAD, Operation::DECREMENTATION), ticks: 8 },
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
    Instruction { name: "SUB B", itype: SUB(B), ticks: 4 },
    Instruction { name: "SUB C", itype: SUB(C), ticks: 4 },
    Instruction { name: "SUB D", itype: SUB(D), ticks: 4 },
    Instruction { name: "SUB E", itype: SUB(E), ticks: 4 },
    Instruction { name: "SUB H", itype: SUB(H), ticks: 4 },
    Instruction { name: "SUB L", itype: SUB(L), ticks: 4 },
    Instruction { name: "SUB (HL)", itype: SUB(_HL), ticks: 8 },
    Instruction { name: "SUB A", itype: SUB(A), ticks: 4 },
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
    Instruction { name: "CP B", itype: CP(B), ticks: 4 },
    Instruction { name: "CP C", itype: CP(C), ticks: 4 },
    Instruction { name: "CP D", itype: CP(D), ticks: 4 },
    Instruction { name: "CP E", itype: CP(E), ticks: 4 },
    Instruction { name: "CP H", itype: CP(H), ticks: 4 },
    Instruction { name: "CP L", itype: CP(L), ticks: 4 },
    Instruction { name: "CP (HL)", itype: CP(_HL), ticks: 8 },
    Instruction { name: "CP A", itype: CP(A), ticks: 4 },

    // Thirteenth line
    Instruction { name: "RET NZ", itype: NOT_IMPLEMENTED, ticks: 8 },
    Instruction { name: "POP BC", itype: POP(BC), ticks: 12 },
    Instruction { name: "JP NZ, nn", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "JP nn", itype: JUMP(JumpCondition::NONE), ticks: 16 },
    Instruction { name: "CALL NZ, nn", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "PUSH BC", itype: PUSH(BC), ticks: 16 },
    Instruction { name: "ADD A, n", itype: ADD(A, INSTANT), ticks: 8 },
    Instruction { name: "RST 00H", itype: RST(0), ticks: 16 },
    Instruction { name: "RET Z", itype: RET(Flag::Zero), ticks: 8 },
    Instruction { name: "RET", itype: RET(Flag::None), ticks: 16 },
    Instruction { name: "JP Z, nn", itype: JUMP(JumpCondition::Z) , ticks: 12 },
    Instruction { name: "PREFIX CB", itype: PREFIX_CB, ticks: 4 },
    Instruction { name: "CALL Z, nn", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "CALL nn", itype: CALL(JumpCondition::NONE), ticks: 24 },
    Instruction { name: "ADC A, n", itype: ADC(A, INSTANT), ticks: 8 },
    Instruction { name: "RST 08H", itype: RST(0x0008), ticks: 16 },

    // Fourteenth line
    Instruction { name: "RET NC", itype: NOT_IMPLEMENTED, ticks: 8 },
    Instruction { name: "POP DE", itype: POP(DE), ticks: 12 },
    Instruction { name: "JP NC, nn", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 4 },
    Instruction { name: "CALL NC, nn", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "PUSH DE", itype: PUSH(DE), ticks: 16 },
    Instruction { name: "SUB n", itype: SUB(INSTANT), ticks: 8 },
    Instruction { name: "RST 10H", itype: RST(0x0010), ticks: 16 },
    Instruction { name: "RET C", itype: NOT_IMPLEMENTED, ticks: 8 },
    Instruction { name: "RETI", itype: NOT_IMPLEMENTED, ticks: 16 },
    Instruction { name: "JP C, nn", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 4 },
    Instruction { name: "CALL C, nn", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 16 },
    Instruction { name: "SBC A, n", itype: SBC(A, INSTANT), ticks: 8 },
    Instruction { name: "RST 18H", itype: RST(0x0018), ticks: 16 },

    // Fifteenth line
    Instruction { name: "LDH (n), A", itype: LDH(SAVE, INSTANT), ticks: 12 },
    Instruction { name: "POP HL", itype: POP(HL), ticks: 12 },
    Instruction { name: "LD (C), A", itype: LDH(SAVE, C), ticks: 8 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "PUSH HL", itype: PUSH(HL), ticks: 16 },
    Instruction { name: "AND n", itype: AND(A, INSTANT), ticks: 8 },
    Instruction { name: "RST 20H", itype: RST(0x0020), ticks: 16 },
    Instruction { name: "ADD SP, n", itype: ADD22(SP, INSTANT2), ticks: 16 },
    Instruction { name: "JP HL", itype: JP_HL, ticks: 4 },
    Instruction { name: "LD (nn), A", itype: LOAD21(INSTANT2, A), ticks: 16 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "XOR n", itype: XOR(A, INSTANT), ticks: 8 },
    Instruction { name: "RST 28H", itype: RST(0x0028), ticks: 16 },

    // Sixteenth line
    Instruction { name: "LDH A, (n)", itype: LDH(LOAD, INSTANT), ticks: 12 },
    Instruction { name: "POP AF", itype: POP(AF), ticks: 12 },
    Instruction { name: "LD A, (C)", itype: LOAD11(A, C), ticks: 8 },
    Instruction { name: "DI", itype: DI, ticks: 4 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "PUSH AF", itype: PUSH(AF), ticks: 16 },
    Instruction { name: "OR n", itype: OR(A, INSTANT), ticks: 8 },
    Instruction { name: "RST 30H", itype: RST(0x0030), ticks: 16 },
    Instruction { name: "LD HL, SP+n", itype: NOT_IMPLEMENTED, ticks: 12 },
    Instruction { name: "LD SP, HL", itype: LOAD22(SP, HL), ticks: 8 },
    Instruction { name: "LD A, (nn)", itype: LOAD12(A, INSTANT2), ticks: 16 },
    Instruction { name: "EI", itype: EI, ticks: 4 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "XX", itype: NOT_IMPLEMENTED, ticks: 0 },
    Instruction { name: "CP n", itype: CP(INSTANT), ticks: 8 },
    Instruction { name: "RST 38H", itype: RST(0x38) , ticks: 16 },

];


const LIST_INSTRUCTION_CB: [Instruction; 256] = [

    // First line
    Instruction { name: "RLC B", itype: RLC(B), ticks: 8 },
    Instruction { name: "RLC C", itype: RLC(C), ticks: 8 },
    Instruction { name: "RLC D", itype: RLC(D), ticks: 8 },
    Instruction { name: "RLC E", itype: RLC(E), ticks: 8 },
    Instruction { name: "RLC H", itype: RLC(H), ticks: 8 },
    Instruction { name: "RLC L", itype: RLC(L), ticks: 8 },
    Instruction { name: "RLC (HL)", itype: RLC(_HL), ticks: 16 },
    Instruction { name: "RLC A", itype: RLC(A), ticks: 8 },
    Instruction { name: "RRC B", itype: RRC(B), ticks: 8 },
    Instruction { name: "RRC C", itype: RRC(C), ticks: 8 },
    Instruction { name: "RRC D", itype: RRC(D), ticks: 8 },
    Instruction { name: "RRC E", itype: RRC(E), ticks: 8 },
    Instruction { name: "RRC H", itype: RRC(H), ticks: 8 },
    Instruction { name: "RRC L", itype: RRC(L), ticks: 8 },
    Instruction { name: "RRC (HL)", itype: RRC(_HL), ticks: 16 },
    Instruction { name: "RRC A", itype: RRC(A), ticks: 8 },

    // Second line
    Instruction { name: "RL B", itype: RL(B), ticks: 8 },
    Instruction { name: "RL C", itype: RL(C), ticks: 8 },
    Instruction { name: "RL D", itype: RL(D), ticks: 8 },
    Instruction { name: "RL E", itype: RL(E), ticks: 8 },
    Instruction { name: "RL H", itype: RL(H), ticks: 8 },
    Instruction { name: "RL L", itype: RL(L), ticks: 8 },
    Instruction { name: "RL (HL)", itype: RL(_HL), ticks: 16 },
    Instruction { name: "RL A", itype: RL(A), ticks: 8 },
    Instruction { name: "RR B", itype: RR(B), ticks: 8 },
    Instruction { name: "RR C", itype: RR(C), ticks: 8 },
    Instruction { name: "RR D", itype: RR(D), ticks: 8 },
    Instruction { name: "RR E", itype: RR(E), ticks: 8 },
    Instruction { name: "RR H", itype: RR(H), ticks: 8 },
    Instruction { name: "RR L", itype: RR(L), ticks: 8 },
    Instruction { name: "RR (HL)", itype: RR(_HL), ticks: 16 },
    Instruction { name: "RR A", itype: RR(A), ticks: 8 },

    // Third line
    Instruction { name: "SLA B", itype: SLA(B), ticks: 8 },
    Instruction { name: "SLA C", itype: SLA(C), ticks: 8 },
    Instruction { name: "SLA D", itype: SLA(D), ticks: 8 },
    Instruction { name: "SLA E", itype: SLA(E), ticks: 8 },
    Instruction { name: "SLA H", itype: SLA(H), ticks: 8 },
    Instruction { name: "SLA L", itype: SLA(L), ticks: 8 },
    Instruction { name: "SLA (HL)", itype: SLA(_HL), ticks: 16 },
    Instruction { name: "SLA A", itype: SLA(A), ticks: 8 },
    Instruction { name: "SRA B", itype: SRA(B), ticks: 8 },
    Instruction { name: "SRA C", itype: SRA(C), ticks: 8 },
    Instruction { name: "SRA D", itype: SRA(D), ticks: 8 },
    Instruction { name: "SRA E", itype: SRA(E), ticks: 8 },
    Instruction { name: "SRA H", itype: SRA(H), ticks: 8 },
    Instruction { name: "SRA L", itype: SRA(L), ticks: 8 },
    Instruction { name: "SRA (HL)", itype: SRA(_HL), ticks: 16 },
    Instruction { name: "SRA A", itype: SRA(A), ticks: 8 },

    // Fourth line
    Instruction { name: "SWAP B", itype: SWAP(B), ticks: 8 },
    Instruction { name: "SWAP C", itype: SWAP(C), ticks: 8 },
    Instruction { name: "SWAP D", itype: SWAP(D), ticks: 8 },
    Instruction { name: "SWAP E", itype: SWAP(E), ticks: 8 },
    Instruction { name: "SWAP H", itype: SWAP(H), ticks: 8 },
    Instruction { name: "SWAP L", itype: SWAP(L), ticks: 8 },
    Instruction { name: "SWAP (HL)", itype: SWAP(_HL), ticks: 16 },
    Instruction { name: "SWAP A", itype: SWAP(A), ticks: 8 },
    Instruction { name: "SRL B", itype: SRL(B), ticks: 8 },
    Instruction { name: "SRL C", itype: SRL(C), ticks: 8 },
    Instruction { name: "SRL D", itype: SRL(D), ticks: 8 },
    Instruction { name: "SRL E", itype: SRL(E), ticks: 8 },
    Instruction { name: "SRL H", itype: SRL(H), ticks: 8 },
    Instruction { name: "SRL L", itype: SRL(L), ticks: 8 },
    Instruction { name: "SRL (HL)", itype: SRL(_HL), ticks: 16 },
    Instruction { name: "SRL A", itype: SRL(A), ticks: 8 },

    // Fifth line
    Instruction { name: "BIT 0, B", itype: BIT(0, B), ticks: 8 },
    Instruction { name: "BIT 0, C", itype: BIT(0, C), ticks: 8 },
    Instruction { name: "BIT 0, D", itype: BIT(0, D), ticks: 8 },
    Instruction { name: "BIT 0, E", itype: BIT(0, E), ticks: 8 },
    Instruction { name: "BIT 0, H", itype: BIT(0, H), ticks: 8 },
    Instruction { name: "BIT 0, L", itype: BIT(0, L), ticks: 8 },
    Instruction { name: "BIT 0, (HL)", itype: BIT(0, _HL), ticks: 16 },
    Instruction { name: "BIT 0, A", itype: BIT(0, A), ticks: 8 },
    Instruction { name: "BIT 1, B", itype: BIT(1, B), ticks: 8 },
    Instruction { name: "BIT 1, C", itype: BIT(1, C), ticks: 8 },
    Instruction { name: "BIT 1, D", itype: BIT(1, D), ticks: 8 },
    Instruction { name: "BIT 1, E", itype: BIT(1, E), ticks: 8 },
    Instruction { name: "BIT 1, H", itype: BIT(1, H), ticks: 8 },
    Instruction { name: "BIT 1, L", itype: BIT(1, L), ticks: 8 },
    Instruction { name: "BIT 1, (HL)", itype: BIT(1, _HL), ticks: 16 },
    Instruction { name: "BIT 1, A", itype: BIT(1, A), ticks: 8 },

    // Sixth line
    Instruction { name: "BIT 2, B", itype: BIT(2, B), ticks: 8 },
    Instruction { name: "BIT 2, C", itype: BIT(2, C), ticks: 8 },
    Instruction { name: "BIT 2, D", itype: BIT(2, D), ticks: 8 },
    Instruction { name: "BIT 2, E", itype: BIT(2, E), ticks: 8 },
    Instruction { name: "BIT 2, H", itype: BIT(2, H), ticks: 8 },
    Instruction { name: "BIT 2, L", itype: BIT(2, L), ticks: 8 },
    Instruction { name: "BIT 2, (HL)", itype: BIT(2, _HL), ticks: 16 },
    Instruction { name: "BIT 2, A", itype: BIT(2, A), ticks: 8 },
    Instruction { name: "BIT 3, B", itype: BIT(3, B), ticks: 8 },
    Instruction { name: "BIT 3, C", itype: BIT(3, C), ticks: 8 },
    Instruction { name: "BIT 3, D", itype: BIT(3, D), ticks: 8 },
    Instruction { name: "BIT 3, E", itype: BIT(3, E), ticks: 8 },
    Instruction { name: "BIT 3, H", itype: BIT(3, H), ticks: 8 },
    Instruction { name: "BIT 3, L", itype: BIT(3, L), ticks: 8 },
    Instruction { name: "BIT 3, (HL)", itype: BIT(3, _HL), ticks: 16 },
    Instruction { name: "BIT 3, A", itype: BIT(3, A), ticks: 8 },

    // Seventh line
    Instruction { name: "BIT 4, B", itype: BIT(4, B), ticks: 8 },
    Instruction { name: "BIT 4, C", itype: BIT(4, C), ticks: 8 },
    Instruction { name: "BIT 4, D", itype: BIT(4, D), ticks: 8 },
    Instruction { name: "BIT 4, E", itype: BIT(4, E), ticks: 8 },
    Instruction { name: "BIT 4, H", itype: BIT(4, H), ticks: 8 },
    Instruction { name: "BIT 4, L", itype: BIT(4, L), ticks: 8 },
    Instruction { name: "BIT 4, (HL)", itype: BIT(4, _HL), ticks: 16 },
    Instruction { name: "BIT 4, A", itype: BIT(4, A), ticks: 8 },
    Instruction { name: "BIT 5, B", itype: BIT(5, B), ticks: 8 },
    Instruction { name: "BIT 5, C", itype: BIT(5, C), ticks: 8 },
    Instruction { name: "BIT 5, D", itype: BIT(5, D), ticks: 8 },
    Instruction { name: "BIT 5, E", itype: BIT(5, E), ticks: 8 },
    Instruction { name: "BIT 5, H", itype: BIT(5, H), ticks: 8 },
    Instruction { name: "BIT 5, L", itype: BIT(5, L), ticks: 8 },
    Instruction { name: "BIT 5, (HL)", itype: BIT(5, _HL), ticks: 16 },
    Instruction { name: "BIT 5, A", itype: BIT(5, A), ticks: 8 },

    // Eighth line
    Instruction { name: "BIT 6, B", itype: BIT(6, B), ticks: 8 },
    Instruction { name: "BIT 6, C", itype: BIT(6, C), ticks: 8 },
    Instruction { name: "BIT 6, D", itype: BIT(6, D), ticks: 8 },
    Instruction { name: "BIT 6, E", itype: BIT(6, E), ticks: 8 },
    Instruction { name: "BIT 6, H", itype: BIT(6, H), ticks: 8 },
    Instruction { name: "BIT 6, L", itype: BIT(6, L), ticks: 8 },
    Instruction { name: "BIT 6, (HL)", itype: BIT(6, _HL), ticks: 16 },
    Instruction { name: "BIT 6, A", itype: BIT(6, A), ticks: 8 },
    Instruction { name: "BIT 7, B", itype: BIT(7, B), ticks: 8 },
    Instruction { name: "BIT 7, C", itype: BIT(7, C), ticks: 8 },
    Instruction { name: "BIT 7, D", itype: BIT(7, D), ticks: 8 },
    Instruction { name: "BIT 7, E", itype: BIT(7, E), ticks: 8 },
    Instruction { name: "BIT 7, H", itype: BIT(7, H), ticks: 8 },
    Instruction { name: "BIT 7, L", itype: BIT(7, L), ticks: 8 },
    Instruction { name: "BIT 7, (HL)", itype: BIT(7, _HL), ticks: 16 },
    Instruction { name: "BIT 7, A", itype: BIT(7, A), ticks: 8 },

    // Ninth line
    Instruction { name: "RES 0, B", itype: RES(0, B), ticks: 8 },
    Instruction { name: "RES 0, C", itype: RES(0, C), ticks: 8 },
    Instruction { name: "RES 0, D", itype: RES(0, D), ticks: 8 },
    Instruction { name: "RES 0, E", itype: RES(0, E), ticks: 8 },
    Instruction { name: "RES 0, H", itype: RES(0, H), ticks: 8 },
    Instruction { name: "RES 0, L", itype: RES(0, L), ticks: 8 },
    Instruction { name: "RES 0, (HL)", itype: RES(0, _HL), ticks: 16 },
    Instruction { name: "RES 0, A", itype: RES(0, A), ticks: 8 },
    Instruction { name: "RES 1, B", itype: RES(1, B), ticks: 8 },
    Instruction { name: "RES 1, C", itype: RES(1, C), ticks: 8 },
    Instruction { name: "RES 1, D", itype: RES(1, D), ticks: 8 },
    Instruction { name: "RES 1, E", itype: RES(1, E), ticks: 8 },
    Instruction { name: "RES 1, H", itype: RES(1, H), ticks: 8 },
    Instruction { name: "RES 1, L", itype: RES(1, L), ticks: 8 },
    Instruction { name: "RES 1, (HL)", itype: RES(1, _HL), ticks: 16 },
    Instruction { name: "RES 1, A", itype: RES(1, A), ticks: 8 },
    
    // Tenth line
    Instruction { name: "RES 2, B", itype: RES(2, B), ticks: 8 },
    Instruction { name: "RES 2, C", itype: RES(2, C), ticks: 8 },
    Instruction { name: "RES 2, D", itype: RES(2, D), ticks: 8 },
    Instruction { name: "RES 2, E", itype: RES(2, E), ticks: 8 },
    Instruction { name: "RES 2, H", itype: RES(2, H), ticks: 8 },
    Instruction { name: "RES 2, L", itype: RES(2, L), ticks: 8 },
    Instruction { name: "RES 2, (HL)", itype: RES(2, _HL), ticks: 16 },
    Instruction { name: "RES 2, A", itype: RES(2, A), ticks: 8 },
    Instruction { name: "RES 3, B", itype: RES(3, B), ticks: 8 },
    Instruction { name: "RES 3, C", itype: RES(3, C), ticks: 8 },
    Instruction { name: "RES 3, D", itype: RES(3, D), ticks: 8 },
    Instruction { name: "RES 3, E", itype: RES(3, E), ticks: 8 },
    Instruction { name: "RES 3, H", itype: RES(3, H), ticks: 8 },
    Instruction { name: "RES 3, L", itype: RES(3, L), ticks: 8 },
    Instruction { name: "RES 3, (HL)", itype: RES(3, _HL), ticks: 16 },
    Instruction { name: "RES 3, A", itype: RES(3, A), ticks: 8 },

    // Eleventh line
    Instruction { name: "RES 4, B", itype: RES(4, B), ticks: 8 },
    Instruction { name: "RES 4, C", itype: RES(4, C), ticks: 8 },
    Instruction { name: "RES 4, D", itype: RES(4, D), ticks: 8 },
    Instruction { name: "RES 4, E", itype: RES(4, E), ticks: 8 },
    Instruction { name: "RES 4, H", itype: RES(4, H), ticks: 8 },
    Instruction { name: "RES 4, L", itype: RES(4, L), ticks: 8 },
    Instruction { name: "RES 4, (HL)", itype: RES(4, _HL), ticks: 16 },
    Instruction { name: "RES 4, A", itype: RES(4, A), ticks: 8 },
    Instruction { name: "RES 5, B", itype: RES(5, B), ticks: 8 },
    Instruction { name: "RES 5, C", itype: RES(5, C), ticks: 8 },
    Instruction { name: "RES 5, D", itype: RES(5, D), ticks: 8 },
    Instruction { name: "RES 5, E", itype: RES(5, E), ticks: 8 },
    Instruction { name: "RES 5, H", itype: RES(5, H), ticks: 8 },
    Instruction { name: "RES 5, L", itype: RES(5, L), ticks: 8 },
    Instruction { name: "RES 5, (HL)", itype: RES(5, _HL), ticks: 16 },
    Instruction { name: "RES 5, A", itype: RES(5, A), ticks: 8 },

    // Twelfth line
    Instruction { name: "RES 6, B", itype: RES(6, B), ticks: 8 },
    Instruction { name: "RES 6, C", itype: RES(6, C), ticks: 8 },
    Instruction { name: "RES 6, D", itype: RES(6, D), ticks: 8 },
    Instruction { name: "RES 6, E", itype: RES(6, E), ticks: 8 },
    Instruction { name: "RES 6, H", itype: RES(6, H), ticks: 8 },
    Instruction { name: "RES 6, L", itype: RES(6, L), ticks: 8 },
    Instruction { name: "RES 6, (HL)", itype: RES(6, _HL), ticks: 16 },
    Instruction { name: "RES 6, A", itype: RES(6, A), ticks: 8 },
    Instruction { name: "RES 7, B", itype: RES(7, B), ticks: 8 },
    Instruction { name: "RES 7, C", itype: RES(7, C), ticks: 8 },
    Instruction { name: "RES 7, D", itype: RES(7, D), ticks: 8 },
    Instruction { name: "RES 7, E", itype: RES(7, E), ticks: 8 },
    Instruction { name: "RES 7, H", itype: RES(7, H), ticks: 8 },
    Instruction { name: "RES 7, L", itype: RES(7, L), ticks: 8 },
    Instruction { name: "RES 7, (HL)", itype: RES(7, _HL), ticks: 16 },
    Instruction { name: "RES 7, A", itype: RES(7, A), ticks: 8 },

    // Thirteenth line
    Instruction { name: "SET 0, B", itype: SET(0, B), ticks: 8 },
    Instruction { name: "SET 0, C", itype: SET(0, C), ticks: 8 },
    Instruction { name: "SET 0, D", itype: SET(0, D), ticks: 8 },
    Instruction { name: "SET 0, E", itype: SET(0, E), ticks: 8 },
    Instruction { name: "SET 0, H", itype: SET(0, H), ticks: 8 },
    Instruction { name: "SET 0, L", itype: SET(0, L), ticks: 8 },
    Instruction { name: "SET 0, (HL)", itype: SET(0, _HL), ticks: 16 },
    Instruction { name: "SET 0, A", itype: SET(0, A), ticks: 8 },
    Instruction { name: "SET 1, B", itype: SET(1, B), ticks: 8 },
    Instruction { name: "SET 1, C", itype: SET(1, C), ticks: 8 },
    Instruction { name: "SET 1, D", itype: SET(1, D), ticks: 8 },
    Instruction { name: "SET 1, E", itype: SET(1, E), ticks: 8 },
    Instruction { name: "SET 1, H", itype: SET(1, H), ticks: 8 },
    Instruction { name: "SET 1, L", itype: SET(1, L), ticks: 8 },
    Instruction { name: "SET 1, (HL)", itype: SET(1, _HL), ticks: 16 },
    Instruction { name: "SET 1, A", itype: SET(1, A), ticks: 8 },

    // Fourteenth line
    Instruction { name: "SET 2, B", itype: SET(2, B), ticks: 8 },
    Instruction { name: "SET 2, C", itype: SET(2, C), ticks: 8 },
    Instruction { name: "SET 2, D", itype: SET(2, D), ticks: 8 },
    Instruction { name: "SET 2, E", itype: SET(2, E), ticks: 8 },
    Instruction { name: "SET 2, H", itype: SET(2, H), ticks: 8 },
    Instruction { name: "SET 2, L", itype: SET(2, L), ticks: 8 },
    Instruction { name: "SET 2, (HL)", itype: SET(2, _HL), ticks: 16 },
    Instruction { name: "SET 2, A", itype: SET(2, A), ticks: 8 },
    Instruction { name: "SET 3, B", itype: SET(3, B), ticks: 8 },
    Instruction { name: "SET 3, C", itype: SET(3, C), ticks: 8 },
    Instruction { name: "SET 3, D", itype: SET(3, D), ticks: 8 },
    Instruction { name: "SET 3, E", itype: SET(3, E), ticks: 8 },
    Instruction { name: "SET 3, H", itype: SET(3, H), ticks: 8 },
    Instruction { name: "SET 3, L", itype: SET(3, L), ticks: 8 },
    Instruction { name: "SET 3, (HL)", itype: SET(3, _HL), ticks: 16 },
    Instruction { name: "SET 3, A", itype: SET(3, A), ticks: 8 },

    // Fifteenth line
    Instruction { name: "SET 4, B", itype: SET(4, B), ticks: 8 },
    Instruction { name: "SET 4, C", itype: SET(4, C), ticks: 8 },
    Instruction { name: "SET 4, D", itype: SET(4, D), ticks: 8 },
    Instruction { name: "SET 4, E", itype: SET(4, E), ticks: 8 },
    Instruction { name: "SET 4, H", itype: SET(4, H), ticks: 8 },
    Instruction { name: "SET 4, L", itype: SET(4, L), ticks: 8 },
    Instruction { name: "SET 4, (HL)", itype: SET(4, _HL), ticks: 16 },
    Instruction { name: "SET 4, A", itype: SET(4, A), ticks: 8 },
    Instruction { name: "SET 5, B", itype: SET(5, B), ticks: 8 },
    Instruction { name: "SET 5, C", itype: SET(5, C), ticks: 8 },
    Instruction { name: "SET 5, D", itype: SET(5, D), ticks: 8 },
    Instruction { name: "SET 5, E", itype: SET(5, E), ticks: 8 },
    Instruction { name: "SET 5, H", itype: SET(5, H), ticks: 8 },
    Instruction { name: "SET 5, L", itype: SET(5, L), ticks: 8 },
    Instruction { name: "SET 5, (HL)", itype: SET(5, _HL), ticks: 16 },
    Instruction { name: "SET 5, A", itype: SET(5, A), ticks: 8 },

    // Sixteenth line
    Instruction { name: "SET 6, B", itype: SET(6, B), ticks: 8 },
    Instruction { name: "SET 6, C", itype: SET(6, C), ticks: 8 },
    Instruction { name: "SET 6, D", itype: SET(6, D), ticks: 8 },
    Instruction { name: "SET 6, E", itype: SET(6, E), ticks: 8 },
    Instruction { name: "SET 6, H", itype: SET(6, H), ticks: 8 },
    Instruction { name: "SET 6, L", itype: SET(6, L), ticks: 8 },
    Instruction { name: "SET 6, (HL)", itype: SET(6, _HL), ticks: 16 },
    Instruction { name: "SET 6, A", itype: SET(6, A), ticks: 8 },
    Instruction { name: "SET 7, B", itype: SET(7, B), ticks: 8 },
    Instruction { name: "SET 7, C", itype: SET(7, C), ticks: 8 },
    Instruction { name: "SET 7, D", itype: SET(7, D), ticks: 8 },
    Instruction { name: "SET 7, E", itype: SET(7, E), ticks: 8 },
    Instruction { name: "SET 7, H", itype: SET(7, H), ticks: 8 },
    Instruction { name: "SET 7, L", itype: SET(7, L), ticks: 8 },
    Instruction { name: "SET 7, (HL)", itype: SET(7, _HL), ticks: 16 },
    Instruction { name: "SET 7, A", itype: SET(7, A), ticks: 8 },
];
