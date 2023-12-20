use crate::alu;
use crate::info::{Bit, Word};
use crate::info::{BIT_0, BIT_1};
use crate::register::RegisterKind;

pub struct Instruction(Word);

impl Instruction {
    pub fn opcode(&self) -> [Bit; 7] {
        [
            self.0.bit(0),
            self.0.bit(1),
            self.0.bit(2),
            self.0.bit(3),
            self.0.bit(4),
            self.0.bit(5),
            self.0.bit(6),
        ]
    }
    // pub fn format(&self) -> InstructionFormat {
    //     let opcode = self.opcode();
    //     match opcode {
    //         // 1110110
    //         [BIT_1, BIT_1, BIT_1, BIT_0, BIT_1, BIT_1, BIT_0] => InstructionFormat::U,
    //         // 1111011
    //         [BIT_1, BIT_1, BIT_1, BIT_1, BIT_0, BIT_1, BIT_1] => InstructionFormat::J,
    //         // 1100000
    //         [BIT_1, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0] => InstructionFormat::I,
    //         // 1100011
    //         [BIT_1, BIT_1, BIT_0, BIT_0, BIT_0, BIT_1, BIT_1] => InstructionFormat::B,
    //         // 1100010
    //         [BIT_1, BIT_1, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0] => InstructionFormat::S,
    //         [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0] => {}
    //         [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0] => {}
    //         _ => panic!("instruction opcode is illegal"),
    //     }
    //     todo!()
    // }
}

pub enum InstructionFormat {
    R {
        opcode: [Bit; 7],
        rd: [Bit; 5],
        func3: [Bit; 3],
        rs1: [Bit; 5],
        rs2: [Bit; 5],
        func7: [Bit; 7],
    },

    I {
        opcode: [Bit; 7],
        rd: [Bit; 5],
        func3: [Bit; 3],
        rs1: [Bit; 5],
        imm0_11: [Bit; 12],
    },

    S {
        opcode: [Bit; 7],
        imm0_4: [Bit; 5],
        func3: [Bit; 3],
        rs1: [Bit; 5],
        rs2: [Bit; 5],
        imm5_11: [Bit; 7],
    },

    B {
        opcode: [Bit; 7],
        imm11: Bit,
        imm1_4: [Bit; 4],
        func3: [Bit; 3],
        rs1: [Bit; 5],
        rs2: [Bit; 5],
        imm5_10: [Bit; 6],
        imm12: Bit,
    },

    U {
        opcode: [Bit; 7],
        rd: [Bit; 5],
        imm12_31: [Bit; 20],
    },

    J {
        opcode: [Bit; 7],
        rd: [Bit; 5],
        imm12_19: [Bit; 8],
        imm11: Bit,
        imm1_10: [Bit; 10],
        imm20: Bit,
    },
}

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

pub enum InstructionType {
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

    // pub fn as_alu_operation(&self) -> alu::Operation {
    //     match self {
    //         Instruction::ADD(_, _, _) | Instruction::ADDI(_, _, _) => alu::Operation::ADD,
    //         Instruction::AND(_, _, _) | Instruction::ANDI(_, _, _) => alu::Operation::AND,
    //         Instruction::SUB(_, _, _) => alu::Operation::SUB,
    //         Instruction::OR(_, _, _) | Instruction::ORI(_, _, _) => alu::Operation::OR,
    //         Instruction::XOR(_, _, _) | Instruction::XORI(_, _, _) => alu::Operation::XOR,
    //         Instruction::SLL(_, _, _) | Instruction::SLLI(_, _, _) => alu::Operation::SLL,
    //         Instruction::SRA(_, _, _) | Instruction::SRAI(_, _, _) => alu::Operation::SRA,
    //         Instruction::SRL(_, _, _) | Instruction::SRLI(_, _, _) => alu::Operation::SRA,
    //         Instruction::SLT(_, _, _) | Instruction::SLTI(_, _, _) => alu::Operation::SLT,
    //         Instruction::SLTU(_, _, _) | Instruction::SLTIU(_, _, _) => alu::Operation::SLTU,
    //         _ => panic!("Can not convert to alu operation"),
    //     }
    // }
}
