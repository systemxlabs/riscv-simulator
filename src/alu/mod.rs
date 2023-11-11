pub mod full_adder;
pub mod half_adder;
pub mod multi_adder;

use crate::alu::multi_adder::MultiAdder;
use crate::binary::{Bit, Byte, Word, BIT_0, BIT_1, BYTE_BIT_SIZE, WORD_BYTE_SIZE};
use crate::gate::and::AndGate;
use crate::gate::not::NotGate;
use crate::gate::or::OrGate;
use crate::gate::xor::XorGate;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    ADD,
    SUB,
    OR,
    AND,
    XOR,
    // set less than (unsigned)
    SLT,
    SLTU,
    // shift left logical
    SLL,
    // shift right logical/arithmetic
    SRL,
    SRA,
}

/// Arithmetic logic unit
pub struct Alu;

impl Alu {
    pub fn exec(&self, op: Operation, input1: Word, input2: Word) -> Word {
        match op {
            Operation::ADD => self.add(input1, input2),
            Operation::SUB => self.add(input1, self.negative(input2)),
            Operation::OR => self.bit_or(input1, input2),
            Operation::AND => self.bit_and(input1, input2),
            Operation::XOR => self.bit_xor(input1, input2),
            Operation::SLT => todo!(),
            Operation::SLTU => todo!(),
            Operation::SLL => todo!(),
            Operation::SRL => todo!(),
            Operation::SRA => todo!(),
        }
    }

    fn add(&self, input1: Word, input2: Word) -> Word {
        MultiAdder::exec(input1, input2, BIT_0).1
    }

    fn bit_and(&self, input1: Word, input2: Word) -> Word {
        let mut word = Word::new();

        for i in 0..WORD_BYTE_SIZE * BYTE_BIT_SIZE {
            let bit_a = input1.bit(i);
            let bit_b = input2.bit(i);

            word.set_bit(i, AndGate::exec(bit_a, bit_b));
        }

        word
    }

    fn bit_or(&self, input1: Word, input2: Word) -> Word {
        let mut word = Word::new();

        for i in 0..WORD_BYTE_SIZE * BYTE_BIT_SIZE {
            let bit_a = input1.bit(i);
            let bit_b = input2.bit(i);

            word.set_bit(i, OrGate::exec(bit_a, bit_b));
        }

        word
    }

    fn bit_not(&self, input: Word) -> Word {
        let mut word = Word::new();

        for i in 0..WORD_BYTE_SIZE * BYTE_BIT_SIZE {
            let bit = input.bit(i);

            word.set_bit(i, NotGate::exec(bit));
        }

        word
    }

    fn bit_xor(&self, input1: Word, input2: Word) -> Word {
        let mut word = Word::new();

        for i in 0..WORD_BYTE_SIZE * BYTE_BIT_SIZE {
            let bit_a = input1.bit(i);
            let bit_b = input2.bit(i);

            word.set_bit(i, XorGate::exec(bit_a, bit_b));
        }

        word
    }

    fn negative(&self, input: Word) -> Word {
        let word_one = Word::from_str("10000000000000000000000000000000");
        let word = self.bit_not(input);
        MultiAdder::exec(word, word_one, BIT_0).1
    }
}

#[cfg(test)]
mod tests {
    use crate::alu::Alu;
    use crate::binary::Word;

    #[test]
    fn negative() {
        let alu = Alu {};

        // complement of -5
        let word = Word::from_str("11011111111111111111111111111111");
        // result should be complement of 5
        assert_eq!(
            format!("{:?}", alu.negative(word)),
            "10100000000000000000000000000000"
        );

        // complement of 5
        let word = Word::from_str("10100000000000000000000000000000");
        // result should be complement of -5
        assert_eq!(
            format!("{:?}", alu.negative(word)),
            "11011111111111111111111111111111"
        );
    }
}
