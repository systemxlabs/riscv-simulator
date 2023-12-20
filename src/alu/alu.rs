use crate::alu::multi_adder::MultiAdder;
use crate::alu::shifter::Shifter;
use crate::circuit::gate::AndGate;
use crate::circuit::gate::NotGate;
use crate::circuit::gate::OrGate;
use crate::circuit::gate::XorGate;
use crate::info::{Bit, Byte, Word, BIT_0, BIT_1, BYTE_BIT_SIZE, WORD_BYTE_SIZE};

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
    // TODO flags: zero, overflow, sign, carry
    pub fn exec(&self, op: Operation, input1: Word, input2: Word) -> Word {
        match op {
            Operation::ADD => self.add(input1, input2, BIT_0),
            // Note that this is not like converting original code to complement,
            // not need to keep the sign bit.
            Operation::SUB => self.add(input1, self.bit_not(input2), BIT_1),
            Operation::OR => self.bit_or(input1, input2),
            Operation::AND => self.bit_and(input1, input2),
            Operation::XOR => self.bit_xor(input1, input2),
            Operation::SLT => todo!(),
            Operation::SLTU => todo!(),
            Operation::SLL | Operation::SRL | Operation::SRA => Shifter::exec(op, input1, input2),
        }
    }

    fn add(&self, input1: Word, input2: Word, carry_in: Bit) -> Word {
        MultiAdder::exec(input1, input2, carry_in).1
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
}

#[cfg(test)]
mod tests {
    use crate::alu::{Alu, Operation};
    use crate::info::{Word, BIT_0};

    #[test]
    fn add() {
        let alu = Alu {};

        // complement of -5
        let input1 = Word::from_str("11011111111111111111111111111111");
        // complement of 5
        let input2 = Word::from_str("10100000000000000000000000000000");
        // result should be 0
        assert_eq!(
            format!("{:?}", alu.exec(Operation::ADD, input1, input2)),
            "00000000000000000000000000000000"
        );
    }

    #[test]
    fn sub() {
        let alu = Alu {};

        // complement of -5
        let word1 = Word::from_str("11011111111111111111111111111111");
        // complement of 5
        let word2 = Word::from_str("10100000000000000000000000000000");
        // result should be complement of -10
        assert_eq!(
            format!("{:?}", alu.exec(Operation::SUB, word1, word2)),
            "01101111111111111111111111111111"
        );
    }
}
