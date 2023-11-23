use crate::info::{Bit, BIT_0, BIT_1};

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

pub struct AndGate4;

impl AndGate4 {
    pub fn exec(input1: Bit, input2: Bit, input3: Bit, input4: Bit) -> Bit {
        AndGate::exec(AndGate::exec(input1, input2), AndGate::exec(input3, input4))
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

pub struct OrGate4;

impl OrGate4 {
    pub fn exec(input1: Bit, input2: Bit, input3: Bit, input4: Bit) -> Bit {
        OrGate::exec(OrGate::exec(input1, input2), OrGate::exec(input3, input4))
    }
}

pub struct OrGate8;

impl OrGate8 {
    pub fn exec(
        input1: Bit,
        input2: Bit,
        input3: Bit,
        input4: Bit,
        input5: Bit,
        input6: Bit,
        input7: Bit,
        input8: Bit,
    ) -> Bit {
        OrGate::exec(
            OrGate4::exec(input1, input2, input3, input4),
            OrGate4::exec(input5, input6, input7, input8),
        )
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
    use crate::circuit::gate::XorGate;
    use crate::info::{BIT_0, BIT_1};

    #[test]
    fn xor() {
        assert_eq!(XorGate::exec(BIT_0, BIT_0), BIT_0);
        assert_eq!(XorGate::exec(BIT_1, BIT_1), BIT_0);
        assert_eq!(XorGate::exec(BIT_0, BIT_1), BIT_1);
        assert_eq!(XorGate::exec(BIT_1, BIT_0), BIT_1);
    }
}
