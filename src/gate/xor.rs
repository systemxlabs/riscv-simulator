use crate::binary::Bit;
use crate::gate::and::AndGate;
use crate::gate::not::NotGate;
use crate::gate::or::OrGate;

pub struct XorGate;

impl XorGate {
    pub fn exec(input1: Bit, input2: Bit) -> Bit {
        let output1 = OrGate::exec(input1, input2);
        let output2 = NotGate::exec(AndGate::exec(input1, input2));
        AndGate::exec(output1, output2)
    }
}

#[cfg(test)]
mod tests {
    use crate::binary::{BIT_0, BIT_1};
    use crate::gate::xor::XorGate;

    #[test]
    fn xor() {
        assert_eq!(XorGate::exec(BIT_0, BIT_0), BIT_0);
        assert_eq!(XorGate::exec(BIT_1, BIT_1), BIT_0);
        assert_eq!(XorGate::exec(BIT_0, BIT_1), BIT_1);
        assert_eq!(XorGate::exec(BIT_1, BIT_0), BIT_1);
    }
}
