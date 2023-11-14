use crate::binary::{Bit, Byte};
use crate::memory::MainMemory;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;

pub fn load_program(main_mem: Rc<RefCell<MainMemory>>) {
    let program_path =
        std::env::var("PROGRAM_PATH").expect("Please set env variable PROGRAM_PATH first");
    println!("Loading program from {}", program_path);
    let fs = std::fs::File::open(program_path).expect("Failed to open program file");

    // load program byte by byte
    let mut reader = std::io::BufReader::new(fs);
    let mut buf = [0; 1];
    let mut addr = 0;
    while let Ok(_) = reader.read_exact(&mut buf) {
        let mut byte = Byte::new();
        for i in 0..8 {
            byte.set(i, Bit::from(buf[0] & (1 << i) != 0));
        }
        main_mem.borrow_mut().write_byte(addr, byte);
        addr += 1;
    }
    println!("Program loaded into memory");
}

#[cfg(test)]
mod test {
    use crate::memory::MainMemory;
    use crate::program::load_program;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_load_program() {
        std::env::set_var("PROGRAM_PATH", "asm/add-addi.bin");
        let main_mem = Rc::new(RefCell::new(MainMemory::new()));
        load_program(main_mem.clone());
        assert_eq!(
            format!("{:?}", main_mem.borrow().read_word(0)),
            "11001001011100000000101000000000"
        );
        assert_eq!(
            format!("{:?}", main_mem.borrow().read_word(4)),
            "11001000111100000000101001000000"
        );
        assert_eq!(
            format!("{:?}", main_mem.borrow().read_word(8)),
            "11001101111100001111101110000000"
        );
        assert_eq!(
            format!("{:?}", main_mem.borrow().read_word(12)),
            "00000000000000000000000000000000"
        );
    }
}
