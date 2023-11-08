/// A byte is a group of 8 bits.
/// true = 1, false = 0
/// Little endian.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Byte(bool, bool, bool, bool, bool, bool, bool, bool);

impl Byte {
    pub fn new() -> Self {
        Self(false, false, false, false, false, false, false, false)
    }

    pub fn set(&mut self, index: usize, value: bool) {
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

    pub fn get(&self, index: usize) -> bool {
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
        if self.0 {
            result += 1;
        }
        if self.1 {
            result += 2;
        }
        if self.2 {
            result += 4;
        }
        if self.3 {
            result += 8;
        }
        if self.4 {
            result += 16;
        }
        if self.5 {
            result += 32;
        }
        if self.6 {
            result += 64;
        }
        if self.7 {
            result += 128;
        }
        result
    }

    pub fn from_u8(mut value: u8) -> Self {
        let mut result = Self::new();
        if value / 128 >= 1 {
            result.7 = true;
            value %= 128;
        }
        if value / 64 >= 1 {
            result.6 = true;
            value %= 64;
        }
        if value / 32 >= 1 {
            result.5 = true;
            value %= 32;
        }
        if value / 16 >= 1 {
            result.4 = true;
            value %= 16;
        }
        if value / 8 >= 1 {
            result.3 = true;
            value %= 8;
        }
        if value / 4 >= 1 {
            result.2 = true;
            value %= 4;
        }
        if value / 2 >= 1 {
            result.1 = true;
            value %= 2;
        }
        if value / 1 >= 1 {
            result.0 = true;
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
            self.7 = false;
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
            self.0 = false;
        }
    }
}

impl std::fmt::Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            if self.0 { 1 } else { 0 },
            if self.1 { 1 } else { 0 },
            if self.2 { 1 } else { 0 },
            if self.3 { 1 } else { 0 },
            if self.4 { 1 } else { 0 },
            if self.5 { 1 } else { 0 },
            if self.6 { 1 } else { 0 },
            if self.7 { 1 } else { 0 },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::byte::Byte;

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
        assert!(byte.get(0));
        byte.set(0, false);
        assert!(!byte.get(0));
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
