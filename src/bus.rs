use crate::memory::MainMemory;
use std::cell::RefCell;
use std::rc::Rc;
use crate::binary::Word;

pub struct Bus {
    main_mem: Rc<RefCell<MainMemory>>,
}

impl Bus {
    pub fn new(main_mem: Rc<RefCell<MainMemory>>) -> Self {
        Self { main_mem }
    }

    pub fn load_instruction(&self, addr: usize) -> Word {
        self.main_mem.borrow().read_word(addr)
    }
}