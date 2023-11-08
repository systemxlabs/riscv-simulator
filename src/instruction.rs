use crate::register::RegisterKind;

pub enum Instruction {
    /// U type instructions
    /// load upper immediate
    LUI(RegisterKind, i32),
    /// add upper immediate to pc
    AUIPC(RegisterKind, i32),

    /// J type instructions
    /// jump and link
    JAL(i32),

    /// I type instructions
    /// jump and link register
    JALR(RegisterKind, i32),
    /// load byte
    LB(RegisterKind, RegisterKind),
    /// load byte unsigned
    LBU(RegisterKind, RegisterKind),
    /// load half word
    LH(RegisterKind, RegisterKind),
    /// load half word unsigned
    LHU(RegisterKind, RegisterKind),
    /// load word
    LW(RegisterKind, RegisterKind),
    /// add immediate
    ADDI(RegisterKind, RegisterKind, i32),
    /// and immediate
    ANDI(RegisterKind, RegisterKind, i32),
    /// or immediate
    ORI(RegisterKind, RegisterKind, i32),
    /// xor immediate
    XORI(RegisterKind, RegisterKind, i32),
    /// set less than immediate
    SLTI(RegisterKind, RegisterKind, i32),
    /// set less than immediate unsigned
    SLTIU(RegisterKind, RegisterKind, i32),
    /// shift left logical immediate
    SLLI(RegisterKind, RegisterKind, usize),
    /// shift right logical immediate
    SRLI(RegisterKind, RegisterKind, usize),
    /// shift right arithmetic immediate
    SRAI(RegisterKind, RegisterKind, usize),

    /// B type instructions
    /// branch equal
    BEQ(RegisterKind, RegisterKind, i32),
    /// branch not equal
    BNE(RegisterKind, RegisterKind, i32),
    /// branch less than
    BLT(RegisterKind, RegisterKind, i32),
    /// branch less than unsigned
    BLTU(RegisterKind, RegisterKind, i32),
    /// branch greater than or equal
    BGE(RegisterKind, RegisterKind, i32),
    /// branch greater than or equal unsigned
    BGEU(RegisterKind, RegisterKind, i32),

    /// S type instructions
    /// store byte
    SB(RegisterKind, RegisterKind),
    /// store half word
    SH(RegisterKind, RegisterKind),
    /// store word
    SW(RegisterKind, RegisterKind),

    /// R type instructions
    /// add
    ADD(RegisterKind, RegisterKind, RegisterKind),
    /// subtract
    SUB(RegisterKind, RegisterKind, RegisterKind),
    /// and
    AND(RegisterKind, RegisterKind, RegisterKind),
    /// or
    OR(RegisterKind, RegisterKind, RegisterKind),
    /// xor
    XOR(RegisterKind, RegisterKind, RegisterKind),
    /// set less than
    SLT(RegisterKind, RegisterKind, RegisterKind),
    /// set less than unsigned
    SLTU(RegisterKind, RegisterKind, RegisterKind),
    /// shift left logical
    SLL(RegisterKind, RegisterKind, usize),
    /// shift right logical
    SRL(RegisterKind, RegisterKind, usize),
    /// shift right arithmetic
    SRA(RegisterKind, RegisterKind, usize),
}
