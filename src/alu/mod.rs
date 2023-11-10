pub mod full_adder;
pub mod half_adder;
pub mod multi_adder;

use crate::binary::{Byte, Word};

/// Arithmetic logic unit
pub struct Alu;

impl Alu {
    pub fn exec(op: Byte, input1: Word, input2: Word) -> Word {
        todo!()
    }
}
