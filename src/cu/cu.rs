use crate::alu::multi_adder::MultiAdder;
use crate::info::Word;
use crate::instruction::Instruction;

/// Control unit
pub struct Cu {
    /// Program counter
    pc: Word,
    adder: MultiAdder,
    /// Instruction register
    ir: Word,
}

impl Cu {
    pub fn new() -> Self {
        Self {
            pc: Word::new(),
            adder: MultiAdder {},
            ir: Word::new(),
        }
    }
    pub fn loop_exec(&self, instruction: Instruction) {
        todo!()
    }
}
