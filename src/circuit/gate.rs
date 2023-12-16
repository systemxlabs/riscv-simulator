use crate::info::{Bit, BIT_0, BIT_1};

pub struct AndGate;

impl AndGate {
    pub fn exec(input0: Bit, input1: Bit) -> Bit {
        if input0.get() && input1.get() {
            BIT_1
        } else {
            BIT_0
        }
    }
}

pub struct AndGate4;

impl AndGate4 {
    pub fn exec(input0: Bit, input1: Bit, input2: Bit, input3: Bit) -> Bit {
        AndGate::exec(AndGate::exec(input0, input1), AndGate::exec(input2, input3))
    }
}

pub struct OrGate;

impl OrGate {
    pub fn exec(input0: Bit, input1: Bit) -> Bit {
        if input0.get() || input1.get() {
            BIT_1
        } else {
            BIT_0
        }
    }
}

pub struct OrGate4;

impl OrGate4 {
    pub fn exec(input0: Bit, input1: Bit, input2: Bit, input3: Bit) -> Bit {
        OrGate::exec(OrGate::exec(input0, input1), OrGate::exec(input2, input3))
    }
}

pub struct OrGate8;

impl OrGate8 {
    pub fn exec(
        input0: Bit,
        input1: Bit,
        input2: Bit,
        input3: Bit,
        input4: Bit,
        input5: Bit,
        input6: Bit,
        input7: Bit,
    ) -> Bit {
        OrGate::exec(
            OrGate4::exec(input0, input1, input2, input3),
            OrGate4::exec(input4, input5, input6, input7),
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
    pub fn exec(input0: Bit, input1: Bit) -> Bit {
        let output1 = OrGate::exec(input0, input1);
        let output2 = NotGate::exec(AndGate::exec(input0, input1));
        AndGate::exec(output1, output2)
    }
}

pub struct NandGate;

impl NandGate {
    pub fn exec(input0: Bit, input1: Bit) -> Bit {
        NotGate::exec(AndGate::exec(input0, input1))
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::gate::{NandGate, XorGate};
    use crate::info::{BIT_0, BIT_1};

    #[test]
    fn xor() {
        assert_eq!(XorGate::exec(BIT_0, BIT_0), BIT_0);
        assert_eq!(XorGate::exec(BIT_1, BIT_1), BIT_0);
        assert_eq!(XorGate::exec(BIT_0, BIT_1), BIT_1);
        assert_eq!(XorGate::exec(BIT_1, BIT_0), BIT_1);
    }

    #[test]
    fn nand() {
        assert_eq!(NandGate::exec(BIT_0, BIT_0), BIT_1);
        assert_eq!(NandGate::exec(BIT_1, BIT_1), BIT_0);
        assert_eq!(NandGate::exec(BIT_0, BIT_1), BIT_1);
        assert_eq!(NandGate::exec(BIT_1, BIT_0), BIT_1);
    }
}
