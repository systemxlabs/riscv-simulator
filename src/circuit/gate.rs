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

pub struct OrGate;

impl OrGate {
    pub fn exec(input1: Bit, input2: Bit) -> Bit {
        if input1.get() || input2.get() {
            BIT_1
        } else {
            BIT_0
        }
    }
}

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
    use crate::circuit::gate::XorGate;

    #[test]
    fn xor() {
        assert_eq!(XorGate::exec(BIT_0, BIT_0), BIT_0);
        assert_eq!(XorGate::exec(BIT_1, BIT_1), BIT_0);
        assert_eq!(XorGate::exec(BIT_0, BIT_1), BIT_1);
        assert_eq!(XorGate::exec(BIT_1, BIT_0), BIT_1);
    }
}