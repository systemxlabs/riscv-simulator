use crate::bus::Bus;
use crate::cpu::{Cpu, Execution};
use crate::mem::MainMemory;
use crate::program::load_program;
use std::cell::RefCell;
use std::rc::Rc;

mod alu;
mod bus;
mod circuit;
mod cpu;
mod cu;
mod info;
mod instruction;
mod mem;
mod program;
mod register;
mod util;

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
