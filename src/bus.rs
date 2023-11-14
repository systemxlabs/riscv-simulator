use crate::memory::MainMemory;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Bus {
    main_mem: Rc<RefCell<MainMemory>>,
}

impl Bus {
    pub fn new(main_mem: Rc<RefCell<MainMemory>>) -> Self {
        Self { main_mem }
    }
}
