use crate::alu::Operation;
use crate::binary::Word;

pub struct Shifter;

impl Shifter {
    pub fn exec(op: Operation, input: Word, shift: Word) -> Word {
        match op {
            Operation::SLL => todo!(),
            Operation::SRL => todo!(),
            Operation::SRA => todo!(),
            _ => panic!("Invalid operation for shifter"),
        }
    }

    fn shift_left_logical(&self, input: Word, shift: Word) -> Word {
        todo!()
    }
}