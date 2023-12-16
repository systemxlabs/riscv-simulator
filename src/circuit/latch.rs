use crate::circuit::gate::{NandGate, NorGate, OrGate};
use crate::info::{Bit, BIT_0, BIT_1};

pub struct SRLatch {
    // input of set
    s_in: Bit,
    // input of reset
    r_in: Bit,
}

impl SRLatch {
    pub fn new() -> Self {
        SRLatch {
            s_in: BIT_0,
            r_in: BIT_0,
        }
    }

    // return (Q, Q_next)
    pub fn exec(&mut self, set: Bit, reset: Bit) -> (Bit, Bit) {
        if set == BIT_1 && reset == BIT_1 {
            panic!("Set and Reset cannot be 1 at the same time")
        }

        let q_next = NorGate::exec(set, self.s_in);
        self.r_in = q_next;
        let q = NorGate::exec(reset, self.r_in);
        self.s_in = q;
        let mut result = (q, q_next);

        loop {
            let q_next = NorGate::exec(set, self.s_in);
            self.r_in = q_next;
            let q = NorGate::exec(reset, self.r_in);
            self.s_in = q;

            if result == (q, q_next) {
                // reach steady state
                break;
            } else {
                result = (q, q_next);
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::latch::SRLatch;
    use crate::info::{BIT_0, BIT_1};

    #[test]
    fn sr_latch() {
        let mut sr_latch = SRLatch::new();
        // get
        assert_eq!(sr_latch.exec(BIT_0, BIT_0), (BIT_0, BIT_1));
        assert_eq!(sr_latch.exec(BIT_0, BIT_0), (BIT_0, BIT_1));
        // set
        assert_eq!(sr_latch.exec(BIT_1, BIT_0), (BIT_1, BIT_0));
        // get
        assert_eq!(sr_latch.exec(BIT_0, BIT_0), (BIT_1, BIT_0));
        assert_eq!(sr_latch.exec(BIT_0, BIT_0), (BIT_1, BIT_0));
        // reset
        assert_eq!(sr_latch.exec(BIT_0, BIT_1), (BIT_0, BIT_1));
        // get
        assert_eq!(sr_latch.exec(BIT_0, BIT_0), (BIT_0, BIT_1));
        assert_eq!(sr_latch.exec(BIT_0, BIT_0), (BIT_0, BIT_1));
    }
}
