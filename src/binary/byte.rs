use crate::binary::{Bit, BIT_0, BIT_1};

/// A byte is a group of 8 bits.
/// Little endian.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Byte(Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit);

impl Byte {
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

    pub fn get(&self, index: usize) -> Bit {
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

    pub fn to_u8(&self) -> u8 {
        let mut result = 0;
        if self.0.get() {
            result += 1;
        }
        if self.1.get() {
            result += 2;
        }
        if self.2.get() {
            result += 4;
        }
        if self.3.get() {
            result += 8;
        }
        if self.4.get() {
            result += 16;
        }
        if self.5.get() {
            result += 32;
        }
        if self.6.get() {
            result += 64;
        }
        if self.7.get() {
            result += 128;
        }
        result
    }

    pub fn from_u8(mut value: u8) -> Self {
        let mut result = Self::new();
        if value / 128 >= 1 {
            result.7 = BIT_1;
            value %= 128;
        }
        if value / 64 >= 1 {
            result.6 = BIT_1;
            value %= 64;
        }
        if value / 32 >= 1 {
            result.5 = BIT_1;
            value %= 32;
        }
        if value / 16 >= 1 {
            result.4 = BIT_1;
            value %= 16;
        }
        if value / 8 >= 1 {
            result.3 = BIT_1;
            value %= 8;
        }
        if value / 4 >= 1 {
            result.2 = BIT_1;
            value %= 4;
        }
        if value / 2 >= 1 {
            result.1 = BIT_1;
            value %= 2;
        }
        if value / 1 >= 1 {
            result.0 = BIT_1;
        }
        result
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
    fn byte_from_to_u8() {
        let byte = Byte::from_u8(255);
        assert_eq!(format!("{:?}", byte), "11111111");
        assert_eq!(byte.to_u8(), 255);

        let byte = Byte::from_u8(131);
        assert_eq!(format!("{:?}", byte), "11000001");
        assert_eq!(byte.to_u8(), 131);

        let byte = Byte::from_u8(1);
        assert_eq!(format!("{:?}", byte), "10000000");
        assert_eq!(byte.to_u8(), 1);
    }

    #[test]
    fn byte_get_set_bit() {
        let mut byte = Byte::from_u8(255);
        assert_eq!(byte.get(0), BIT_1);
        byte.set(0, BIT_0);
        assert_eq!(byte.get(0), BIT_0);
    }

    #[test]
    fn byte_shift() {
        let mut byte = Byte::from_u8(255);
        byte.lshift(2);
        assert_eq!(format!("{:?}", byte), "11111100");
        byte.rshift(3);
        assert_eq!(format!("{:?}", byte), "00011111");
    }
}
