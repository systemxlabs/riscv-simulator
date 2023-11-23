use crate::alu;
use crate::info::Word;
use crate::register::RegisterKind;

pub struct Immediate(i32);
impl Immediate {
    pub fn from(v: i32) -> Self {
        Self(v)
    }
}

pub struct ShiftAmount(u8);
impl ShiftAmount {
    /// shift amount represented by 5 bits and is unsigned.
    pub fn from(v: u8) -> Self {
        assert!(v < 31, "Exceed max value");
        Self(v)
    }
}

pub enum Instruction {
    /// Integer Computation
    /// add (immediate)
    ADD(RegisterKind, RegisterKind, RegisterKind),
    ANDI(RegisterKind, RegisterKind, Immediate),
    /// subtract
    SUB(RegisterKind, RegisterKind, RegisterKind),
    /// and (immediate)
    AND(RegisterKind, RegisterKind, RegisterKind),
    ADDI(RegisterKind, RegisterKind, Immediate),
    /// or (immediate)
    OR(RegisterKind, RegisterKind, RegisterKind),
    ORI(RegisterKind, RegisterKind, Immediate),
    /// xor (immediate)
    XOR(RegisterKind, RegisterKind, RegisterKind),
    XORI(RegisterKind, RegisterKind, Immediate),
    /// shift left logical (immediate)
    SLL(RegisterKind, RegisterKind, ShiftAmount),
    SLLI(RegisterKind, RegisterKind, ShiftAmount),
    /// shift right arithmetic (immediate)
    SRA(RegisterKind, RegisterKind, ShiftAmount),
    SRAI(RegisterKind, RegisterKind, ShiftAmount),
    /// shift right logical (immediate)
    SRL(RegisterKind, RegisterKind, ShiftAmount),
    SRLI(RegisterKind, RegisterKind, ShiftAmount),
    /// load upper immediate
    LUI(RegisterKind, Immediate),
    /// add upper immediate to pc
    AUIPC(RegisterKind, Immediate),
    /// set less than (immediate) (unsigned)
    SLT(RegisterKind, RegisterKind, RegisterKind),
    SLTU(RegisterKind, RegisterKind, RegisterKind),
    SLTI(RegisterKind, RegisterKind, Immediate),
    SLTIU(RegisterKind, RegisterKind, Immediate),

    /// Control Transfer
    /// branch equal
    BEQ(RegisterKind, RegisterKind, Immediate),
    /// branch not equal
    BNE(RegisterKind, RegisterKind, Immediate),
    /// branch greater than or equal (unsigned)
    BGE(RegisterKind, RegisterKind, Immediate),
    BGEU(RegisterKind, RegisterKind, Immediate),
    /// branch less than (unsigned)
    BLT(RegisterKind, RegisterKind, Immediate),
    BLTU(RegisterKind, RegisterKind, Immediate),
    /// jump and link (register)
    JAL(RegisterKind, Immediate),
    JALR(RegisterKind, Immediate),

    /// Loads and Stores
    /// load byte (unsigned)
    LB(RegisterKind, RegisterKind),
    LBU(RegisterKind, RegisterKind),
    /// load half word (unsigned)
    LH(RegisterKind, RegisterKind),
    LHU(RegisterKind, RegisterKind),
    /// load word
    LW(RegisterKind, RegisterKind),
    /// store byte
    SB(RegisterKind, RegisterKind),
    /// store half word
    SH(RegisterKind, RegisterKind),
    /// store word
    SW(RegisterKind, RegisterKind),
}

impl Instruction {
    pub fn word(&self) -> Word {
        todo!()
    }

    pub fn as_alu_operation(&self) -> alu::Operation {
        match self {
            Instruction::ADD(_, _, _) | Instruction::ADDI(_, _, _) => alu::Operation::ADD,
            Instruction::AND(_, _, _) | Instruction::ANDI(_, _, _) => alu::Operation::AND,
            Instruction::SUB(_, _, _) => alu::Operation::SUB,
            Instruction::OR(_, _, _) | Instruction::ORI(_, _, _) => alu::Operation::OR,
            Instruction::XOR(_, _, _) | Instruction::XORI(_, _, _) => alu::Operation::XOR,
            Instruction::SLL(_, _, _) | Instruction::SLLI(_, _, _) => alu::Operation::SLL,
            Instruction::SRA(_, _, _) | Instruction::SRAI(_, _, _) => alu::Operation::SRA,
            Instruction::SRL(_, _, _) | Instruction::SRLI(_, _, _) => alu::Operation::SRA,
            Instruction::SLT(_, _, _) | Instruction::SLTI(_, _, _) => alu::Operation::SLT,
            Instruction::SLTU(_, _, _) | Instruction::SLTIU(_, _, _) => alu::Operation::SLTU,
            _ => panic!("Can not convert to alu operation"),
        }
    }
}
