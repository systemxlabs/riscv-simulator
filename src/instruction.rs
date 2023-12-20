use crate::info::{parse_bits, Bit, Word};
use crate::register::RegisterKind;

pub struct Instruction(Word);

pub fn decode_inst(inst: Word) -> InstructionFormat {
    let opcode = inst.bits(0..7);
    if opcode == parse_bits("1110110") || opcode == parse_bits("1110100") {
        // U format
        return InstructionFormat::U {
            opcode: opcode.try_into().unwrap(),
            rd: inst.bits(7..12).try_into().unwrap(),
            imm12_31: inst.bits(12..32).try_into().unwrap(),
        };
    }
    if opcode == parse_bits("1111011") {
        // J format
        return InstructionFormat::J {
            opcode: opcode.try_into().unwrap(),
            rd: inst.bits(7..12).try_into().unwrap(),
            imm12_19: inst.bits(12..19).try_into().unwrap(),
            imm11: inst.bits(19..20)[0],
            imm1_10: inst.bits(20..31).try_into().unwrap(),
            imm20: inst.bits(31..32)[0],
        };
    }
    if opcode == parse_bits("1110011")
        || opcode == parse_bits("1100000")
        || opcode == parse_bits("1100100")
        || opcode == parse_bits("1111000")
        || opcode == parse_bits("1100111")
    {
        // I format
        return InstructionFormat::I {
            opcode: opcode.try_into().unwrap(),
            rd: inst.bits(7..12).try_into().unwrap(),
            func3: inst.bits(12..15).try_into().unwrap(),
            rs1: inst.bits(15..20).try_into().unwrap(),
            imm0_11: inst.bits(20..32).try_into().unwrap(),
        };
    }
    if opcode == parse_bits("1100011") {
        // B format
        return InstructionFormat::B {
            opcode: opcode.try_into().unwrap(),
            imm11: inst.bits(7..8)[0],
            imm1_4: inst.bits(8..12).try_into().unwrap(),
            func3: inst.bits(12..15).try_into().unwrap(),
            rs1: inst.bits(15..20).try_into().unwrap(),
            rs2: inst.bits(20..25).try_into().unwrap(),
            imm5_10: inst.bits(25..31).try_into().unwrap(),
            imm12: inst.bits(31..32)[0],
        };
    }
    if opcode == parse_bits("1100010") {
        // S format
        return InstructionFormat::S {
            opcode: opcode.try_into().unwrap(),
            imm0_4: inst.bits(7..12).try_into().unwrap(),
            func3: inst.bits(12..15).try_into().unwrap(),
            rs1: inst.bits(15..20).try_into().unwrap(),
            rs2: inst.bits(20..25).try_into().unwrap(),
            imm5_11: inst.bits(25..32).try_into().unwrap(),
        };
    }
    if opcode == parse_bits("1100110") {
        // R format
        return InstructionFormat::R {
            opcode: opcode.try_into().unwrap(),
            rd: inst.bits(7..12).try_into().unwrap(),
            func3: inst.bits(12..15).try_into().unwrap(),
            rs1: inst.bits(15..20).try_into().unwrap(),
            rs2: inst.bits(20..25).try_into().unwrap(),
            func7: inst.bits(25..32).try_into().unwrap(),
        };
    }
    panic!("instruction opcode is illegal")
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

#[cfg(test)]
mod tests {
    use crate::info::{Word, BIT_0, BIT_1};
    use crate::instruction::{decode_inst, InstructionFormat};

    #[test]
    fn test_decode() {
        let fmt = decode_inst(Word::from_str("11001100010000001000000100000000"));
        let InstructionFormat::R {
            opcode,
            rd,
            func3,
            rs1,
            rs2,
            func7,
        } = fmt
        else {
            panic!()
        };
        assert_eq!(opcode, [BIT_1, BIT_1, BIT_0, BIT_0, BIT_1, BIT_1, BIT_0]);
        assert_eq!(rd, [BIT_0, BIT_0, BIT_1, BIT_0, BIT_0]);
        assert_eq!(func3, [BIT_0, BIT_0, BIT_0]);
        assert_eq!(rs1, [BIT_0, BIT_1, BIT_0, BIT_0, BIT_0]);
        assert_eq!(rs2, [BIT_0, BIT_0, BIT_0, BIT_1, BIT_0]);
        assert_eq!(func7, [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0]);
    }
}
