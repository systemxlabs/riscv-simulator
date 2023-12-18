use crate::alu::Alu;
use crate::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;
use crate::cu::cu::Cu;

pub trait Execution {
    fn fetch(&self);
    fn decode(&self);
    fn execute(&self);
    fn access_memory(&self);
    fn write_back(&self);
}

pub struct Cpu {
    alu: Alu,
    cu: Cu,
    bus: Rc<RefCell<Bus>>,
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            alu: Alu {},
            cu: Cu::new(),
            bus,
        }
    }
}

impl Execution for Cpu {
    fn fetch(&self) {
        todo!()
    }

    fn decode(&self) {
        todo!()
    }

    fn execute(&self) {
        todo!()
    }

    fn access_memory(&self) {
        todo!()
    }

    fn write_back(&self) {
        todo!()
    }
}
