use crate::bus::Bus;
use crate::cpu::{Cpu, Execution};
use crate::memory::MainMemory;
use crate::program::load_program;
use std::cell::RefCell;
use std::rc::Rc;

mod alu;
mod binary;
mod bus;
mod cpu;
mod cu;
mod gate;
mod instruction;
mod memory;
mod program;
mod register;

fn main() {
    let main_mem = Rc::new(RefCell::new(MainMemory::new()));
    let bus = Rc::new(RefCell::new(Bus::new(main_mem.clone())));
    let cpu = Cpu::new(bus);

    // load program instructions into memory
    load_program(main_mem);

    // execute program
    loop {
        cpu.fetch();
        cpu.decode();
        cpu.execute();
        cpu.access_memory();
        cpu.write_back();
    }
}
