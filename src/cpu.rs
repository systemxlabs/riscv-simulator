use crate::alu::Alu;
use crate::binary::Word;
use crate::cu::Cu;
use crate::memory::MainMemory;

pub trait Execution {
    fn fetch(&self) -> Word;
    fn decode(&self, instruction: Word) -> Word;
    fn execute(&self, instruction: Word) -> Word;
    fn access_memory(&self, instruction: Word) -> Word;
    fn write_back(&self, instruction: Word) -> Word;
}

pub struct Cpu {
    alu: Alu,
    cu: Cu,
    mem: MainMemory,
}