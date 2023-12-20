use crate::alu::multi_adder::MultiAdder;
use crate::alu::Alu;
use crate::bus::Bus;
use crate::cu::cu::Cu;
use crate::info::{Word, BIT_0, WORD_FOUR};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Pipeline {
    fn fetch(&mut self);
    fn decode(&mut self);
    fn execute(&mut self);
    fn access_memory(&mut self);
    fn write_back(&mut self);
}

pub struct Cpu {
    alu: Alu,
    cu: Cu,
    pc: Word,
    adder: MultiAdder,
    inst_reg: Word,
    // TODO imm generator
    // TODO branch comparator
    bus: Rc<RefCell<Bus>>,
}

impl Cpu {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Self {
            alu: Alu {},
            cu: Cu::new(),
            pc: Word::new(),
            adder: MultiAdder {},
            inst_reg: Word::new(),
            bus,
        }
    }
}

impl Pipeline for Cpu {
    fn fetch(&mut self) {
        // fetch instruction from main memory through bus
        let addr = self.pc;
        let inst = self.bus.borrow().load_instruction(addr);
        self.inst_reg = inst;
        // update pc = pc + 4
        self.pc = MultiAdder::exec(addr, WORD_FOUR, BIT_0).1;
    }

    fn decode(&mut self) {
        todo!()
    }

    fn execute(&mut self) {
        todo!()
    }

    fn access_memory(&mut self) {
        todo!()
    }

    fn write_back(&mut self) {
        todo!()
    }
}
