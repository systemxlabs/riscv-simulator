use crate::binary::Bit;

pub struct DFlipFlop {
    state: Bit,
}
impl DFlipFlop {
    pub fn new() -> Self {
        Self { state: Bit::ZERO }
    }

    pub fn exec(&mut self, input: Bit) -> Bit {
        let output = self.state;
        self.state = input;
        output
    }
}