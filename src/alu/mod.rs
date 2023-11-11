pub mod full_adder;
pub mod half_adder;
pub mod multi_adder;

use crate::alu::multi_adder::MultiAdder;
use crate::binary::{Byte, Word, BYTE_BIT_SIZE, WORD_BYTE_SIZE};
use crate::gate::and::AndGate;
use crate::gate::not::NotGate;
use crate::gate::or::OrGate;
use crate::gate::xor::XorGate;

/// Arithmetic logic unit
pub struct Alu;

impl Alu {
    pub fn exec(&self, op: Byte, input1: Word, input2: Word) -> Word {
        todo!()
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

    fn complement(&self, input: Word) -> Word {
        let input2 = Word::from_str(
            "00000000\
                00000000\
                00000000\
                00000001",
        );
        self.bit_xor(input, input2)
    }
}

#[cfg(test)]
mod tests {
    use crate::alu::Alu;
    use crate::binary::Word;

    #[test]
    fn complement() {
        let alu = Alu {};
        let word = Word::from_str(
            "00100000\
                00001000\
                00000010\
                00000001",
        );
        // assert_eq!(alu.complement(word), word);
        println!("{}", u32::MAX)
    }
}
