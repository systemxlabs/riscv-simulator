use crate::binary::{Bit, Byte, Word, BIT_0, BIT_1};
use crate::circuit::decoder::TwoToFourDecoder;
use crate::mem::chip::Chip;

pub const CHIP_COUNT: usize = 4;

pub struct Dram(Box<[Chip; CHIP_COUNT]>);

impl Dram {
    pub fn new() -> Self {
        Self(Box::new([
            Chip::new(),
            Chip::new(),
            Chip::new(),
            Chip::new(),
        ]))
    }

    pub fn exec_byte(&mut self, addr: Word, write_enable: Bit, data: Byte) -> Byte {
        let chip_addr = [addr.bit(0), addr.bit(1)];
        match TwoToFourDecoder::exec(chip_addr[0], chip_addr[1]) {
            (BIT_1, BIT_0, BIT_0, BIT_0) => self.0[0].exec(addr, write_enable, data),
            (BIT_0, BIT_1, BIT_0, BIT_0) => self.0[1].exec(addr, write_enable, data),
            (BIT_0, BIT_0, BIT_1, BIT_0) => self.0[2].exec(addr, write_enable, data),
            (BIT_0, BIT_0, BIT_0, BIT_1) => self.0[3].exec(addr, write_enable, data),
            _ => unreachable!(),
        }
    }

    pub fn exec_half_word(&mut self, addr: Word, write_enable: Bit, data: [Byte; 2]) -> [Byte; 2] {
        let chip_addr = [addr.bit(0), addr.bit(1)];
        let chip_index = match TwoToFourDecoder::exec(chip_addr[0], chip_addr[1]) {
            (BIT_1, BIT_0, BIT_0, BIT_0) => 0,
            (BIT_0, BIT_1, BIT_0, BIT_0) => 1,
            (BIT_0, BIT_0, BIT_1, BIT_0) => 2,
            (BIT_0, BIT_0, BIT_0, BIT_1) => 3,
            _ => unreachable!(),
        };
        let byte0 = self.0[chip_index].exec(addr, write_enable, data[0]);
        let byte1 = self.0[(chip_index + 1) % CHIP_COUNT].exec(addr, write_enable, data[1]);
        [byte0, byte1]
    }

    pub fn exec_word(&mut self, addr: Word, write_enable: Bit, data: Word) -> Word {
        let chip_addr = [addr.bit(0), addr.bit(1)];
        let chip_index = match TwoToFourDecoder::exec(chip_addr[0], chip_addr[1]) {
            (BIT_1, BIT_0, BIT_0, BIT_0) => 0,
            (BIT_0, BIT_1, BIT_0, BIT_0) => 1,
            (BIT_0, BIT_0, BIT_1, BIT_0) => 2,
            (BIT_0, BIT_0, BIT_0, BIT_1) => 3,
            _ => unreachable!(),
        };
        let byte0 = self.0[chip_index].exec(addr, write_enable, data.byte(0));
        let byte1 = self.0[(chip_index + 1) % CHIP_COUNT].exec(addr, write_enable, data.byte(1));
        let byte2 = self.0[(chip_index + 2) % CHIP_COUNT].exec(addr, write_enable, data.byte(2));
        let byte3 = self.0[(chip_index + 3) % CHIP_COUNT].exec(addr, write_enable, data.byte(3));
        Word::from(byte0, byte1, byte2, byte3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::byte::EMPTY_BYTE;
    use crate::binary::word::EMPTY_WORD;
    use crate::binary::{BIT_0, BIT_1};

    #[test]
    fn dram_byte() {
        let mut dram = Dram::new();

        let addr = Word::from_str("00000000000000000000000000000000");
        let expected = Byte::from_str("11111111");
        dram.exec_byte(addr, BIT_1, expected);
        let byte = dram.exec_byte(addr, BIT_0, EMPTY_BYTE);
        assert_eq!(byte, expected);

        let addr = Word::from_str("01010101000000000000000000000000");
        let expected = Byte::from_str("10101010");
        dram.exec_byte(addr, BIT_1, expected);
        let byte = dram.exec_byte(addr, BIT_0, EMPTY_BYTE);
        assert_eq!(byte, expected);
    }

    #[test]
    fn dram_half_word() {
        let mut dram = Dram::new();

        let addr = Word::from_str("00000000000000000000000000000000");
        let expected = [Byte::from_str("11111111"), Byte::from_str("11111111")];
        dram.exec_half_word(addr, BIT_1, expected);
        let half_word = dram.exec_half_word(addr, BIT_0, [EMPTY_BYTE, EMPTY_BYTE]);
        assert_eq!(half_word, expected);

        let addr = Word::from_str("01010101000000000000000000000000");
        let expected = [Byte::from_str("10101010"), Byte::from_str("10101010")];
        dram.exec_half_word(addr, BIT_1, expected);
        let half_word = dram.exec_half_word(addr, BIT_0, [EMPTY_BYTE, EMPTY_BYTE]);
        assert_eq!(half_word, expected);
    }

    #[test]
    fn dram_word() {
        let mut dram = Dram::new();

        let addr = Word::from_str("00000000000000000000000000000000");
        let expected = Word::from_str("11111111111111111111111111111111");
        dram.exec_word(addr, BIT_1, expected);
        let word = dram.exec_word(addr, BIT_0, EMPTY_WORD);
        assert_eq!(word, expected);

        let addr = Word::from_str("01010101000000000000000000000000");
        let expected = Word::from_str("11101001011011010110100101101011");
        dram.exec_word(addr, BIT_1, expected);
        let word = dram.exec_word(addr, BIT_0, EMPTY_WORD);
        assert_eq!(word, expected);
    }
}
