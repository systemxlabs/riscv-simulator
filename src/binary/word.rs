use crate::binary::{Bit, Byte, BYTE_BIT_SIZE};

pub const WORD_BYTE_SIZE: usize = 4; // 4 bytes

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Word(Byte, Byte, Byte, Byte);

impl Word {
    pub fn new() -> Self {
        Self(Byte::new(), Byte::new(), Byte::new(), Byte::new())
    }

    pub fn from(byte0: Byte, byte1: Byte, byte2: Byte, byte3: Byte) -> Self {
        Self(byte0, byte1, byte2, byte3)
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
}

#[cfg(test)]
mod tests {
    use crate::binary::{Byte, Word, BIT_0, BIT_1};

    #[test]
    fn get_set_bit() {
        let mut word = Word::new();
        word.set_byte(0, Byte::from_u8(255));
        word.set_byte(2, Byte::from_u8(255));

        assert_eq!(word.bit(0), BIT_1);
        assert_eq!(word.bit(7), BIT_1);
        assert_eq!(word.bit(31), BIT_0);

        word.set_bit(7, BIT_0);
        word.set_bit(31, BIT_1);

        assert_eq!(word.bit(7), BIT_0);
        assert_eq!(word.bit(31), BIT_1);
    }
}
