use crate::byte::Byte;

pub type Word = [Byte; 8];

// Note that this can not be set too large, otherwise the program
// will fail to new MainMemory due to invalid memory reference.
const MEMORY_SIZE: usize = 1024; // 1KB

pub struct MainMemory {
    mem: [Byte; MEMORY_SIZE],
}

impl MainMemory {
    pub fn new() -> Self {
        Self {
            mem: [Byte::new(); MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, addr: usize) -> Byte {
        self.mem[addr]
    }

    pub fn write_byte(&mut self, addr: usize, data: Byte) {
        self.mem[addr] = data;
    }

    pub fn read_word(&self, addr: usize) -> Word {
        let mut word = [Byte::new(); 8];
        for i in 0..8 {
            word[i] = self.read_byte(addr + i);
        }
        word
    }

    pub fn write_word(&mut self, addr: usize, data: Word) {
        for i in 0..8 {
            self.write_byte(addr + i, data[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::byte::Byte;
    use crate::memory::MainMemory;

    #[test]
    fn read_write_byte() {
        let mut mem = MainMemory::new();
        mem.write_byte(0, Byte::from_u8(255));
        assert_eq!(mem.read_byte(0), Byte::from_u8(255));
    }

    #[test]
    fn read_write_word() {
        let mut mem = MainMemory::new();
        mem.write_word(1, [Byte::from_u8(255); 8]);
        assert_eq!(mem.read_word(1), [Byte::from_u8(255); 8]);
    }
}
