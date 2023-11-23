use crate::circuit::gate::{AndGate, AndGate4, NotGate, OrGate, OrGate4, OrGate8};
use crate::info::Bit;

pub struct TwoToOneMultiplexer;

impl TwoToOneMultiplexer {
    pub fn exec(input: [Bit; 2], selector: Bit) -> Bit {
        let [input1, input2] = input;
        let not_selector = NotGate::exec(selector);
        OrGate::exec(
            AndGate::exec(input1, not_selector),
            AndGate::exec(input2, selector),
        )
    }
}

pub struct FourToOneMultiplexer;

impl FourToOneMultiplexer {
    pub fn exec(input: [Bit; 4], selector: [Bit; 2]) -> Bit {
        let [input1, input2, input3, input4] = input;
        let [selector1, selector2] = selector;
        let (not_selector1, not_selector2) = (NotGate::exec(selector1), NotGate::exec(selector2));
        OrGate4::exec(
            AndGate::exec(AndGate::exec(not_selector1, not_selector2), input1),
            AndGate::exec(AndGate::exec(selector1, not_selector2), input2),
            AndGate::exec(AndGate::exec(not_selector1, selector2), input3),
            AndGate::exec(AndGate::exec(selector1, selector2), input4),
        )
    }
}

pub struct EightToOneMultiplexer;

impl EightToOneMultiplexer {
    pub fn exec(input: [Bit; 8], selector: [Bit; 3]) -> Bit {
        let [input1, input2, input3, input4, input5, input6, input7, input8] = input;
        let [selector1, selector2, selector3] = selector;
        let (not_selector1, not_selector2, not_selector3) = (
            NotGate::exec(selector1),
            NotGate::exec(selector2),
            NotGate::exec(selector3),
        );
        OrGate8::exec(
            AndGate4::exec(not_selector1, not_selector2, not_selector3, input1),
            AndGate4::exec(selector1, not_selector2, not_selector3, input2),
            AndGate4::exec(not_selector1, selector2, not_selector3, input3),
            AndGate4::exec(selector1, selector2, not_selector3, input4),
            AndGate4::exec(not_selector1, not_selector2, selector3, input5),
            AndGate4::exec(selector1, not_selector2, selector3, input6),
            AndGate4::exec(not_selector1, selector2, selector3, input7),
            AndGate4::exec(selector1, selector2, selector3, input8),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::multiplexer::{
        EightToOneMultiplexer, FourToOneMultiplexer, TwoToOneMultiplexer,
    };
    use crate::info::{BIT_0, BIT_1};

    #[test]
    fn two_to_one_multiplexer() {
        assert_eq!(TwoToOneMultiplexer::exec([BIT_0, BIT_0], BIT_0), BIT_0);
        assert_eq!(TwoToOneMultiplexer::exec([BIT_0, BIT_0], BIT_1), BIT_0);
        assert_eq!(TwoToOneMultiplexer::exec([BIT_0, BIT_1], BIT_0), BIT_0);
        assert_eq!(TwoToOneMultiplexer::exec([BIT_0, BIT_1], BIT_1), BIT_1);
        assert_eq!(TwoToOneMultiplexer::exec([BIT_1, BIT_0], BIT_0), BIT_1);
        assert_eq!(TwoToOneMultiplexer::exec([BIT_1, BIT_0], BIT_1), BIT_0);
        assert_eq!(TwoToOneMultiplexer::exec([BIT_1, BIT_1], BIT_0), BIT_1);
        assert_eq!(TwoToOneMultiplexer::exec([BIT_1, BIT_1], BIT_1), BIT_1);
    }

    #[test]
    fn four_to_one_multiplexer() {
        // select index=0 bit
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_0, BIT_0, BIT_0, BIT_0], [BIT_0, BIT_0]),
            BIT_0
        );
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_1, BIT_0, BIT_0, BIT_0], [BIT_0, BIT_0]),
            BIT_1
        );
        // select index=1 bit
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_0, BIT_0, BIT_0, BIT_0], [BIT_1, BIT_0]),
            BIT_0
        );
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_0, BIT_1, BIT_0, BIT_0], [BIT_1, BIT_0]),
            BIT_1
        );
        // select index=2 bit
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_0, BIT_0, BIT_0, BIT_0], [BIT_0, BIT_1]),
            BIT_0
        );
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_0, BIT_0, BIT_1, BIT_0], [BIT_0, BIT_1]),
            BIT_1
        );
        // select index=3 bit
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_0, BIT_0, BIT_0, BIT_0], [BIT_1, BIT_1]),
            BIT_0
        );
        assert_eq!(
            FourToOneMultiplexer::exec([BIT_0, BIT_0, BIT_0, BIT_1], [BIT_1, BIT_1]),
            BIT_1
        );
    }

    #[test]
    fn eight_to_one_multiplexer() {
        // select index=0 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_0, BIT_0, BIT_0]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_0, BIT_0, BIT_0]
            ),
            BIT_1
        );
        // select index=1 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_1, BIT_0, BIT_0]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_1, BIT_0, BIT_0]
            ),
            BIT_1
        );
        // select index=2 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_0, BIT_1, BIT_0]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_0, BIT_1, BIT_0]
            ),
            BIT_1
        );
        // select index=3 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_1, BIT_1, BIT_0]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_1, BIT_1, BIT_0]
            ),
            BIT_1
        );
        // select index=4 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_0, BIT_0, BIT_1]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0],
                [BIT_0, BIT_0, BIT_1]
            ),
            BIT_1
        );
        // select index=5 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_1, BIT_0, BIT_1]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0],
                [BIT_1, BIT_0, BIT_1]
            ),
            BIT_1
        );
        // select index=6 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_0, BIT_1, BIT_1]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0],
                [BIT_0, BIT_1, BIT_1]
            ),
            BIT_1
        );
        // select index=7 bit
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0],
                [BIT_1, BIT_1, BIT_1]
            ),
            BIT_0
        );
        assert_eq!(
            EightToOneMultiplexer::exec(
                [BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1],
                [BIT_1, BIT_1, BIT_1]
            ),
            BIT_1
        );
    }
}
