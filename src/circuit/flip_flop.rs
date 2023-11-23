use crate::info::{Bit, BIT_0};

pub struct DFlipFlop {
    state: Bit,
}
impl DFlipFlop {
    pub fn new() -> Self {
        Self { state: BIT_0 }
    }

    pub fn exec(&mut self, input: Bit) -> Bit {
        let output = self.state;
        self.state = input;
        output
    }
}
