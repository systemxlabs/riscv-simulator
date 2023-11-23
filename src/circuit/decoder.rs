use crate::circuit::gate::AndGate;
use crate::circuit::gate::NotGate;
use crate::info::Bit;

pub struct OneToTwoDecoder;

impl OneToTwoDecoder {
    pub fn exec(input: Bit) -> (Bit, Bit) {
        (NotGate::exec(input), input)
    }
}

pub struct TwoToFourDecoder;

impl TwoToFourDecoder {
    pub fn exec(input1: Bit, input2: Bit) -> (Bit, Bit, Bit, Bit) {
        let (not_input1, not_input2) = (NotGate::exec(input1), NotGate::exec(input2));
        let output1 = AndGate::exec(not_input1, not_input2);
        let output2 = AndGate::exec(input1, not_input2);
        let output3 = AndGate::exec(not_input1, input2);
        let output4 = AndGate::exec(input1, input2);
        (output1, output2, output3, output4)
    }
}

pub struct ThreeToEightDecoder;

impl ThreeToEightDecoder {
    pub fn exec(input1: Bit, input2: Bit, input3: Bit) -> (Bit, Bit, Bit, Bit, Bit, Bit, Bit, Bit) {
        let (not_input1, not_input2, not_input3) = (
            NotGate::exec(input1),
            NotGate::exec(input2),
            NotGate::exec(input3),
        );
        let output1 = AndGate::exec(AndGate::exec(not_input1, not_input2), not_input3);
        let output2 = AndGate::exec(AndGate::exec(input1, not_input2), not_input3);
        let output3 = AndGate::exec(AndGate::exec(not_input1, input2), not_input3);
        let output4 = AndGate::exec(AndGate::exec(input1, input2), not_input3);
        let output5 = AndGate::exec(AndGate::exec(not_input1, not_input2), input3);
        let output6 = AndGate::exec(AndGate::exec(input1, not_input2), input3);
        let output7 = AndGate::exec(AndGate::exec(not_input1, input2), input3);
        let output8 = AndGate::exec(AndGate::exec(input1, input2), input3);
        (
            output1, output2, output3, output4, output5, output6, output7, output8,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::decoder::{OneToTwoDecoder, ThreeToEightDecoder, TwoToFourDecoder};
    use crate::info::{BIT_0, BIT_1};

    #[test]
    fn one_to_two_decoder() {
        assert_eq!(OneToTwoDecoder::exec(BIT_0), (BIT_1, BIT_0));
        assert_eq!(OneToTwoDecoder::exec(BIT_1), (BIT_0, BIT_1));
    }

    #[test]
    fn two_to_four_decoder() {
        assert_eq!(
            TwoToFourDecoder::exec(BIT_0, BIT_0),
            (BIT_1, BIT_0, BIT_0, BIT_0)
        );
        assert_eq!(
            TwoToFourDecoder::exec(BIT_1, BIT_0),
            (BIT_0, BIT_1, BIT_0, BIT_0)
        );
        assert_eq!(
            TwoToFourDecoder::exec(BIT_0, BIT_1),
            (BIT_0, BIT_0, BIT_1, BIT_0)
        );
        assert_eq!(
            TwoToFourDecoder::exec(BIT_1, BIT_1),
            (BIT_0, BIT_0, BIT_0, BIT_1)
        );
    }

    #[test]
    fn three_to_eight_decoder() {
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_0, BIT_0, BIT_0),
            (BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0)
        );
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_1, BIT_0, BIT_0),
            (BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0)
        );
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_0, BIT_1, BIT_0),
            (BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0)
        );
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_1, BIT_1, BIT_0),
            (BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0)
        );
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_0, BIT_0, BIT_1),
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0)
        );
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_1, BIT_0, BIT_1),
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0)
        );
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_0, BIT_1, BIT_1),
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0)
        );
        assert_eq!(
            ThreeToEightDecoder::exec(BIT_1, BIT_1, BIT_1),
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1)
        );
    }
}
