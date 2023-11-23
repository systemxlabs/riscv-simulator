use crate::binary::byte::{BYTE_ONE, EMPTY_BYTE};
use crate::binary::{Bit, Byte, BYTE_BIT_SIZE};

pub const WORD_BYTE_SIZE: usize = 4; // 4 bytes

pub const EMPTY_WORD: Word = Word(EMPTY_BYTE, EMPTY_BYTE, EMPTY_BYTE, EMPTY_BYTE);
pub const WORD_ONE: Word = Word(BYTE_ONE, EMPTY_BYTE, EMPTY_BYTE, EMPTY_BYTE);

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Word(Byte, Byte, Byte, Byte);

impl Word {
    pub fn new() -> Self {
        Self(Byte::new(), Byte::new(), Byte::new(), Byte::new())
    }

    pub fn from(byte0: Byte, byte1: Byte, byte2: Byte, byte3: Byte) -> Self {
        Self(byte0, byte1, byte2, byte3)
    }

    pub fn from_str(s: &str) -> Self {
        assert_eq!(s.len(), 32);
        Self(
            Byte::from_str(&s[0..8]),
            Byte::from_str(&s[8..16]),
            Byte::from_str(&s[16..24]),
            Byte::from_str(&s[24..32]),
        )
    }

    pub fn set_byte(&mut self, byte_index: usize, value: Byte) {
        match byte_index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn byte(&self, index: usize) -> Byte {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn bit(&self, index: usize) -> Bit {
        let byte = self.byte(index / BYTE_BIT_SIZE);
        byte.bit(index % BYTE_BIT_SIZE)
    }

    pub fn set_bit(&mut self, bit_index: usize, value: Bit) {
        let byte_index = bit_index / BYTE_BIT_SIZE;
        match byte_index {
            0 => self.0.set(bit_index % BYTE_BIT_SIZE, value),
            1 => self.1.set(bit_index % BYTE_BIT_SIZE, value),
            2 => self.2.set(bit_index % BYTE_BIT_SIZE, value),
            3 => self.3.set(bit_index % BYTE_BIT_SIZE, value),
            _ => panic!("Index out of bounds"),
        }
    }

    /// mainly for debugging
    pub fn display_in_big_endian(&self) -> String {
        let display = format!("{:?}", self);
        display
            .chars()
            .rev()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}{:?}{:?}", self.0, self.1, self.2, self.3)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::{Byte, Word, BIT_0, BIT_1};

    #[test]
    fn get_set_bit() {
        let mut word = Word::new();
        word.set_byte(0, Byte::ALL_ONE);
        word.set_byte(2, Byte::ALL_ONE);

        assert_eq!(word.bit(0), BIT_1);
        assert_eq!(word.bit(7), BIT_1);
        assert_eq!(word.bit(31), BIT_0);

        word.set_bit(7, BIT_0);
        word.set_bit(31, BIT_1);

        assert_eq!(word.bit(7), BIT_0);
        assert_eq!(word.bit(31), BIT_1);
    }
}
