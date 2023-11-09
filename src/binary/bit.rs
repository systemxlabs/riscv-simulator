pub const BIT_0: Bit = Bit::from(false);
pub const BIT_1: Bit = Bit::from(true);

/// true = 1, false = 0
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit(bool);

impl Bit {
    pub const fn new() -> Self {
        Self(false)
    }

    pub const fn from(value: bool) -> Self {
        Self(value)
    }

    pub fn get(&self) -> bool {
        self.0
    }

    pub fn set1(&mut self) {
        self.0 = true;
    }

    pub fn set0(&mut self) {
        self.0 = false;
    }

    pub fn reverse(&mut self) {
        self.0 = !self.0;
    }
}

impl std::fmt::Debug for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { 1 } else { 0 },)
    }
}
