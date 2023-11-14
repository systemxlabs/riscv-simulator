use crate::binary::Word;
use crate::instruction::Instruction;

/// Control unit
pub struct Cu {
    /// Program counter
    pc: Word,
    /// Instruction register
    ir: Word,
}

impl Cu {
    pub fn new() -> Self {
        Self {
            pc: Word::new(),
            ir: Word::new(),
        }
    }
    pub fn loop_exec(instruction: Instruction) {
        todo!()
    }
}
