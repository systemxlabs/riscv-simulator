use crate::binary::{Bit, BIT_0, BIT_1};

pub struct NotGate;

impl NotGate {
    pub fn exec(input: Bit) -> Bit {
        if input.get() {
            BIT_0
        } else {
            BIT_1
        }
    }
}
