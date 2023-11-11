use crate::binary::{Bit, BIT_0, BIT_1};

pub const BYTE_BIT_SIZE: usize = 8; // 4 bits

/// A byte is a group of 8 bits.
/// Little endian.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Byte(Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit);

impl Byte {
    pub const ALL_ONE: Byte = Byte(BIT_1, BIT_1, BIT_1, BIT_1, BIT_1, BIT_1, BIT_1, BIT_1);

    pub fn new() -> Self {
        Self(BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0)
    }

    pub fn set(&mut self, index: usize, value: Bit) {
        match index {
            0 => self.0 = value,
            1 => self.1 = value,
            2 => self.2 = value,
            3 => self.3 = value,
            4 => self.4 = value,
            5 => self.5 = value,
            6 => self.6 = value,
            7 => self.7 = value,
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn bit(&self, index: usize) -> Bit {
        match index {
            0 => self.0,
            1 => self.1,
            2 => self.2,
            3 => self.3,
            4 => self.4,
            5 => self.5,
            6 => self.6,
            7 => self.7,
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn from_str(s: &str) -> Self {
        assert_eq!(s.len(), 8);
        Self(
            Bit::from_str(&s[0..1]),
            Bit::from_str(&s[1..2]),
            Bit::from_str(&s[2..3]),
            Bit::from_str(&s[3..4]),
            Bit::from_str(&s[4..5]),
            Bit::from_str(&s[5..6]),
            Bit::from_str(&s[6..7]),
            Bit::from_str(&s[7..8]),
        )
    }

    /// left shift
    pub fn lshift(&mut self, value: usize) {
        for _ in 0..value {
            self.0 = self.1;
            self.1 = self.2;
            self.2 = self.3;
            self.3 = self.4;
            self.4 = self.5;
            self.5 = self.6;
            self.6 = self.7;
            self.7 = BIT_0;
        }
    }

    /// right shift
    pub fn rshift(&mut self, value: usize) {
        for _ in 0..value {
            self.7 = self.6;
            self.6 = self.5;
            self.5 = self.4;
            self.4 = self.3;
            self.3 = self.2;
            self.2 = self.1;
            self.1 = self.0;
            self.0 = BIT_0;
        }
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Byte;
    use crate::binary::{BIT_0, BIT_1};

    #[test]
    fn byte_get_set_bit() {
        let mut byte = Byte::ALL_ONE;
        assert_eq!(byte.bit(0), BIT_1);
        byte.set(0, BIT_0);
        assert_eq!(byte.bit(0), BIT_0);
    }

    #[test]
    fn byte_shift() {
        let mut byte = Byte::ALL_ONE;
        byte.lshift(2);
        assert_eq!(format!("{:?}", byte), "11111100");
        byte.rshift(3);
        assert_eq!(format!("{:?}", byte), "00011111");
    }
}
