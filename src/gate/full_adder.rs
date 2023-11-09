use crate::binary::Bit;
use crate::gate::half_adder::HalfAdder;
use crate::gate::or::OrGate;

pub struct FullAdder;

impl FullAdder {
    pub fn exec(input1: Bit, input2: Bit, carry_in: Bit) -> (Bit, Bit) {
        let (carry1, sum1) = HalfAdder::exec(input1, input2);
        let (carry2, sum2) = HalfAdder::exec(sum1, carry_in);
        let carry = OrGate::exec(carry1, carry2);
        (carry, sum2)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::{BIT_0, BIT_1};
    use crate::gate::full_adder::FullAdder;

    #[test]
    fn full_adder() {
        assert_eq!(FullAdder::exec(BIT_0, BIT_0, BIT_0), (BIT_0, BIT_0));
        assert_eq!(FullAdder::exec(BIT_0, BIT_1, BIT_0), (BIT_0, BIT_1));
        assert_eq!(FullAdder::exec(BIT_1, BIT_0, BIT_0), (BIT_0, BIT_1));
        assert_eq!(FullAdder::exec(BIT_1, BIT_1, BIT_0), (BIT_1, BIT_0));
        assert_eq!(FullAdder::exec(BIT_0, BIT_0, BIT_1), (BIT_0, BIT_1));
        assert_eq!(FullAdder::exec(BIT_0, BIT_1, BIT_1), (BIT_1, BIT_0));
        assert_eq!(FullAdder::exec(BIT_1, BIT_0, BIT_1), (BIT_1, BIT_0));
        assert_eq!(FullAdder::exec(BIT_1, BIT_1, BIT_1), (BIT_1, BIT_1));
    }
}
