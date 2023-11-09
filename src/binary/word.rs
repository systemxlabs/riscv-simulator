use crate::binary::Byte;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Word(Byte, Byte, Byte, Byte, Byte, Byte, Byte, Byte);

impl Word {
    pub fn new() -> Self {
        Self(
            Byte::new(),
            Byte::new(),
            Byte::new(),
            Byte::new(),
            Byte::new(),
            Byte::new(),
            Byte::new(),
            Byte::new(),
        )
    }

    pub fn set(&mut self, index: usize, value: Byte) {
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

    pub fn get(&self, index: usize) -> Byte {
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
}
