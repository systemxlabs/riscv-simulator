use crate::binary::{Bit, BIT_0, BIT_1};

pub struct AndGate;

impl AndGate {
    pub fn exec(input1: Bit, input2: Bit) -> Bit {
        if input1.get() && input2.get() {
            BIT_1
        } else {
            BIT_0
        }
    }
}
