use crate::alu::full_adder::FullAdder;
use crate::info::{Bit, Word, BYTE_BIT_SIZE, WORD_BYTE_SIZE};

/// Multi bit adder
pub struct MultiAdder;

impl MultiAdder {
    /// input two words + carry_in, output carry + sum
    pub fn exec(input1: Word, input2: Word, mut carry_in: Bit) -> (Bit, Word) {
        let mut sum_word = Word::new();

        for i in 0..WORD_BYTE_SIZE * BYTE_BIT_SIZE {
            let bit_a = input1.bit(i);
            let bit_b = input2.bit(i);

            let (carry_out, sum) = FullAdder::exec(bit_a, bit_b, carry_in);
            carry_in = carry_out;
            sum_word.set_bit(i, sum);
        }

        (carry_in, sum_word)
    }
}

#[cfg(test)]
mod tests {
    use crate::alu::multi_adder::MultiAdder;
    use crate::info::{Byte, Word, BIT_0, BIT_1};

    #[test]
    fn multi_adder() {
        let mut word1 = Word::new();
        word1.set_byte(0, Byte::ALL_ONE);
        word1.set_byte(2, Byte::ALL_ONE);

        let mut word2 = Word::new();
        word2.set_byte(1, Byte::ALL_ONE);
        word2.set_byte(3, Byte::ALL_ONE);

        let (carry, sum) = MultiAdder::exec(word1, word2, BIT_0);
        assert_eq!(carry, BIT_0);
        assert_eq!(
            sum,
            Word::from(Byte::ALL_ONE, Byte::ALL_ONE, Byte::ALL_ONE, Byte::ALL_ONE)
        );

        let mut word1 = Word::new();
        word1.set_byte(0, Byte::ALL_ONE);
        word1.set_byte(2, Byte::ALL_ONE);
        word1.set_bit(31, BIT_1);

        let mut word2 = Word::new();
        word2.set_byte(1, Byte::ALL_ONE);
        word2.set_byte(3, Byte::ALL_ONE);

        let (carry, sum) = MultiAdder::exec(word1, word2, BIT_0);
        assert_eq!(carry, BIT_1);
        assert_eq!(
            sum,
            Word::from(
                Byte::ALL_ONE,
                Byte::ALL_ONE,
                Byte::ALL_ONE,
                Byte::from_str("11111110")
            )
        );
    }
}
