use crate::binary::Bit;
use crate::gate::and::AndGate;
use crate::gate::xor::XorGate;

pub struct HalfAdder;

impl HalfAdder {
    pub fn exec(input1: Bit, input2: Bit) -> (Bit, Bit) {
        let sum = XorGate::exec(input1, input2);
        let carry = AndGate::exec(input1, input2);
        (carry, sum)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::{BIT_0, BIT_1};
    use crate::gate::half_adder::HalfAdder;

    #[test]
    fn half_adder() {
        assert_eq!(HalfAdder::exec(BIT_0, BIT_0), (BIT_0, BIT_0));
        assert_eq!(HalfAdder::exec(BIT_0, BIT_1), (BIT_0, BIT_1));
        assert_eq!(HalfAdder::exec(BIT_1, BIT_0), (BIT_0, BIT_1));
        assert_eq!(HalfAdder::exec(BIT_1, BIT_1), (BIT_1, BIT_0));
    }
}
