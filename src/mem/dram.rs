use crate::binary::{Bit, Word};
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

    pub fn exec(&mut self, addr: Word, write_enable: Bit, data: Word) -> Word {
        let byte0 = self.0[0].exec(addr, write_enable, data.byte(0));
        let byte1 = self.0[1].exec(addr, write_enable, data.byte(1));
        let byte2 = self.0[2].exec(addr, write_enable, data.byte(2));
        let byte3 = self.0[3].exec(addr, write_enable, data.byte(3));
        Word::from(byte0, byte1, byte2, byte3)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::{BIT_0, BIT_1};
    use crate::binary::word::EMPTY_WORD;
    use super::*;

    #[test]
    fn test_dram() {
        let mut dram = Dram::new();

        let addr = Word::from_str("00000000000000000000000000000000");
        let expected = Word::from_str("11111111111111111111111111111111");
        dram.exec(addr, BIT_1, expected);
        let word = dram.exec(addr, BIT_0, EMPTY_WORD);
        assert_eq!(word, expected);

        let addr = Word::from_str("01010101000000000000000000000000");
        let expected = Word::from_str("11101001011011010110100101101011");
        dram.exec(addr, BIT_1, expected);
        let word = dram.exec(addr, BIT_0, EMPTY_WORD);
        assert_eq!(word, expected);
    }
}