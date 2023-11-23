use crate::info::word::EMPTY_WORD;
use crate::info::{Word, BIT_0};
use crate::mem::MainMemory;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Bus {
    main_mem: Rc<RefCell<MainMemory>>,
}

impl Bus {
    pub fn new(main_mem: Rc<RefCell<MainMemory>>) -> Self {
        Self { main_mem }
    }

    pub fn load_instruction(&self, addr: Word) -> Word {
        self.main_mem
            .borrow_mut()
            .exec_word(addr, BIT_0, EMPTY_WORD)
    }
}
