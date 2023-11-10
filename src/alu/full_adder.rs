use crate::alu::half_adder::HalfAdder;
use crate::binary::Bit;
use crate::gate::or::OrGate;

/// One bit adder
pub struct FullAdder;

impl FullAdder {
    /// input two bits + carry, output carry + sum
    pub fn exec(input1: Bit, input2: Bit, carry_in: Bit) -> (Bit, Bit) {
        let (carry1, sum1) = HalfAdder::exec(input1, input2);
        let (carry2, sum2) = HalfAdder::exec(sum1, carry_in);
        let carry = OrGate::exec(carry1, carry2);
        (carry, sum2)
    }
}

#[cfg(test)]
mod tests {
    use crate::alu::full_adder::FullAdder;
    use crate::binary::{BIT_0, BIT_1};

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
