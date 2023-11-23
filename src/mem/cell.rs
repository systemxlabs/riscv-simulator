use crate::binary::Bit;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell(Bit);

impl Cell {
    pub fn new() -> Self {
        Self(Bit::new())
    }

    pub fn get(&self) -> Bit {
        self.0
    }

    pub fn set(&mut self, bit: Bit) {
        self.0 = bit;
    }
}
