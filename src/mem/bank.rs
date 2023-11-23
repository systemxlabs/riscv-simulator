use crate::binary::{Bit, Byte};
use crate::mem::array::Array;

pub const ARRAY_COUNT: usize = 8;

pub struct Bank(Box<[Array; ARRAY_COUNT]>);

impl Bank {
    pub fn new() -> Self {
        Self(Box::new([
            Array::new(),
            Array::new(),
            Array::new(),
            Array::new(),
            Array::new(),
            Array::new(),
            Array::new(),
            Array::new(),
        ]))
    }

    pub fn exec(
        &mut self,
        row_addr: [Bit; 3],
        col_addr: [Bit; 3],
        write_enable: Bit,
        data: Byte,
    ) -> Byte {
        let bit0 = self.0[0].exec(row_addr, col_addr, write_enable, data.bit(0));
        let bit1 = self.0[1].exec(row_addr, col_addr, write_enable, data.bit(1));
        let bit2 = self.0[2].exec(row_addr, col_addr, write_enable, data.bit(2));
        let bit3 = self.0[3].exec(row_addr, col_addr, write_enable, data.bit(3));
        let bit4 = self.0[4].exec(row_addr, col_addr, write_enable, data.bit(4));
        let bit5 = self.0[5].exec(row_addr, col_addr, write_enable, data.bit(5));
        let bit6 = self.0[6].exec(row_addr, col_addr, write_enable, data.bit(6));
        let bit7 = self.0[7].exec(row_addr, col_addr, write_enable, data.bit(7));
        Byte::from(bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7)
    }
}

#[cfg(test)]
mod tests {
    use super::Bank;
    use crate::binary::byte::EMPTY_BYTE;
    use crate::binary::{Byte, BIT_0, BIT_1};

    #[test]
    fn test_bank() {
        let mut bank = Bank::new();

        let row_addr = [BIT_0, BIT_0, BIT_0];
        let col_addr = [BIT_0, BIT_0, BIT_0];
        let expected = Byte::from_str("10000000");
        bank.exec(row_addr, col_addr, BIT_1, expected);
        let byte = bank.exec(row_addr, col_addr, BIT_0, EMPTY_BYTE);
        assert_eq!(byte, expected);

        let row_addr = [BIT_1, BIT_0, BIT_1];
        let col_addr = [BIT_1, BIT_1, BIT_0];
        let expected = Byte::from_str("10010001");
        bank.exec(row_addr, col_addr, BIT_1, expected);
        let byte = bank.exec(row_addr, col_addr, BIT_0, EMPTY_BYTE);
        assert_eq!(byte, expected);
    }
}
