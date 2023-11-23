use crate::binary::{Bit, Byte, Word, BIT_0, BIT_1};
use crate::circuit::decoder::ThreeToEightDecoder;
use crate::mem::bank::Bank;

pub const BANK_COUNT: usize = 8;

pub struct Chip(Box<[Bank; BANK_COUNT]>);

impl Chip {
    pub fn new() -> Self {
        Self(Box::new([
            Bank::new(),
            Bank::new(),
            Bank::new(),
            Bank::new(),
            Bank::new(),
            Bank::new(),
            Bank::new(),
            Bank::new(),
        ]))
    }

    pub fn exec(&mut self, addr: Word, write_enable: Bit, data: Byte) -> Byte {
        let row_addr = [addr.bit(0), addr.bit(1), addr.bit(2)];
        let col_addr = [addr.bit(3), addr.bit(4), addr.bit(5)];
        let bank_addr = [addr.bit(6), addr.bit(7), addr.bit(8)];

        let byte = match ThreeToEightDecoder::exec(bank_addr[0], bank_addr[1], bank_addr[2]) {
            (BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => {
                self.0[0].exec(row_addr, col_addr, write_enable, data)
            }
            (BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => {
                self.0[1].exec(row_addr, col_addr, write_enable, data)
            }
            (BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => {
                self.0[2].exec(row_addr, col_addr, write_enable, data)
            }
            (BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0) => {
                self.0[3].exec(row_addr, col_addr, write_enable, data)
            }
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0) => {
                self.0[4].exec(row_addr, col_addr, write_enable, data)
            }
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0) => {
                self.0[5].exec(row_addr, col_addr, write_enable, data)
            }
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0) => {
                self.0[6].exec(row_addr, col_addr, write_enable, data)
            }
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1) => {
                self.0[7].exec(row_addr, col_addr, write_enable, data)
            }
            _ => unreachable!(),
        };

        byte
    }
}

#[cfg(test)]
mod tests {
    use super::Chip;
    use crate::binary::byte::EMPTY_BYTE;
    use crate::binary::{Bit, Byte, Word, BIT_0, BIT_1};

    #[test]
    fn dram_chip() {
        let mut chip = Chip::new();
        let addr = Word::from_str("00000000000000000000000000000000");
        let expected = Byte::from_str("11111111");
        chip.exec(addr, BIT_1, expected);
        let byte = chip.exec(addr, BIT_0, EMPTY_BYTE);
        assert_eq!(byte, expected);

        let addr = Word::from_str("01010101000000000000000000000000");
        let expected = Byte::from_str("10101010");
        chip.exec(addr, BIT_1, expected);
        let byte = chip.exec(addr, BIT_0, EMPTY_BYTE);
        assert_eq!(byte, expected);
    }
}
