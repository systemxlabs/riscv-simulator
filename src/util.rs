use crate::info::{Bit, Word};

pub fn chip_addr(addr: &Word) -> [Bit; 2] {
    [addr.bit(0), addr.bit(1)]
}
pub fn bank_addr(addr: &Word) -> [Bit; 3] {
    [addr.bit(2), addr.bit(3), addr.bit(4)]
}
pub fn row_addr(addr: &Word) -> [Bit; 3] {
    [addr.bit(5), addr.bit(6), addr.bit(7)]
}
pub fn col_addr(addr: &Word) -> [Bit; 3] {
    [addr.bit(8), addr.bit(9), addr.bit(10)]
}
