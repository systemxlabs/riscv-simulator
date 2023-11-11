use crate::binary::Word;
use crate::binary::{Byte, WORD_BYTE_SIZE};

const MEMORY_SIZE: usize = 1024 * 1024; // 1MB

pub struct MainMemory {
    mem: Vec<Byte>,
}

impl MainMemory {
    pub fn new() -> Self {
        Self {
            mem: vec![Byte::new(); MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, addr: usize) -> Byte {
        self.mem[addr]
    }

    pub fn write_byte(&mut self, addr: usize, data: Byte) {
        self.mem[addr] = data;
    }

    pub fn read_word(&self, addr: usize) -> Word {
        let mut word = Word::new();
        for i in 0..WORD_BYTE_SIZE {
            word.set_byte(i, self.read_byte(addr + i))
        }
        word
    }

    pub fn write_word(&mut self, addr: usize, data: Word) {
        for i in 0..WORD_BYTE_SIZE {
            self.write_byte(addr + i, data.byte(i));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::{Byte, Word};
    use crate::memory::MainMemory;

    #[test]
    fn read_write_byte() {
        let mut mem = MainMemory::new();
        mem.write_byte(0, Byte::ALL_ONE);
        assert_eq!(mem.read_byte(0), Byte::ALL_ONE);
    }

    #[test]
    fn read_write_word() {
        let mut mem = MainMemory::new();

        let mut word = Word::new();
        word.set_byte(0, Byte::ALL_ONE);
        word.set_byte(1, Byte::ALL_ONE);
        word.set_byte(3, Byte::ALL_ONE);

        mem.write_word(1, word.clone());
        assert_eq!(mem.read_word(1), word);
    }
}
