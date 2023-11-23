use crate::alu::{Alu, Operation};
use crate::info::word::WORD_ONE;
use crate::info::{Bit, Byte, Word, BIT_1, BYTE_BIT_SIZE};
use crate::mem::MainMemory;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

pub fn load_program(main_mem: Rc<RefCell<MainMemory>>) {
    let program_path =
        std::env::var("PROGRAM_PATH").expect("Please set env variable PROGRAM_PATH first");
    println!("Loading program from {}", program_path);
    let fs = std::fs::File::open(program_path).expect("Failed to open program file");

    // load program should be done by OS, but we don't have one,
    // so we use alu to increment address and manually load program
    let alu = Alu {};

    // load program byte by byte
    let mut reader = std::io::BufReader::new(fs);
    let mut buf = [0; 1];
    let mut addr = Word::new();
    while let Ok(_) = reader.read_exact(&mut buf) {
        let mut byte = Byte::new();
        for i in 0..BYTE_BIT_SIZE {
            byte.set(i, Bit::from(buf[0] & (1 << i) != 0));
        }
        main_mem.borrow_mut().exec_byte(addr, BIT_1, byte);

        // increment addr
        addr = alu.exec(Operation::ADD, addr, WORD_ONE);
    }
    println!("Program loaded into memory");
}

#[cfg(test)]
mod test {
    use crate::info::word::EMPTY_WORD;
    use crate::info::{Word, BIT_0};
    use crate::mem::MainMemory;
    use crate::program::load_program;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_load_program() {
        std::env::set_var("PROGRAM_PATH", "asm/add-addi.bin");
        let main_mem = Rc::new(RefCell::new(MainMemory::new()));
        load_program(main_mem.clone());

        // addr 0
        assert_eq!(
            main_mem
                .borrow_mut()
                .exec_word(
                    Word::from_str("00000000000000000000000000000000"),
                    BIT_0,
                    EMPTY_WORD
                )
                .display_in_big_endian(),
            "00000000010100000000111010010011"
        );
        // addr 4
        assert_eq!(
            main_mem
                .borrow_mut()
                .exec_word(
                    Word::from_str("00100000000000000000000000000000"),
                    BIT_0,
                    EMPTY_WORD
                )
                .display_in_big_endian(),
            "00000010010100000000111100010011"
        );
        // addr 8
        assert_eq!(
            main_mem
                .borrow_mut()
                .exec_word(
                    Word::from_str("00010000000000000000000000000000"),
                    BIT_0,
                    EMPTY_WORD
                )
                .display_in_big_endian(),
            "00000001110111110000111110110011"
        );
        // addr 12
        assert_eq!(
            main_mem
                .borrow_mut()
                .exec_word(
                    Word::from_str("00110000000000000000000000000000"),
                    BIT_0,
                    EMPTY_WORD
                )
                .display_in_big_endian(),
            "00000000000000000000000000000000"
        );
    }
}
