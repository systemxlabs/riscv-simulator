use crate::circuit::decoder::ThreeToEightDecoder;
use crate::circuit::multiplexer::EightToOneMultiplexer;
use crate::info::{Bit, BIT_0, BIT_1};
use crate::mem::cell::Cell;

pub const ARRAY_ROWS: usize = 8;
pub const ARRAY_COLS: usize = 8;

pub struct Array(Box<[[Cell; ARRAY_COLS]; ARRAY_ROWS]>);

impl Array {
    pub fn new() -> Self {
        Self(Box::new([[Cell::new(); ARRAY_COLS]; ARRAY_ROWS]))
    }

    pub fn exec(
        &mut self,
        row_addr: [Bit; 3],
        col_addr: [Bit; 3],
        write_enable: Bit,
        data: Bit,
    ) -> Bit {
        let row = match ThreeToEightDecoder::exec(row_addr[0], row_addr[1], row_addr[2]) {
            (BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => &mut self.0[0],
            (BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => &mut self.0[1],
            (BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => &mut self.0[2],
            (BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0) => &mut self.0[3],
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0) => &mut self.0[4],
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0) => &mut self.0[5],
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0) => &mut self.0[6],
            (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1) => &mut self.0[7],
            _ => unreachable!(),
        };
        let bit = EightToOneMultiplexer::exec(
            [
                row[0].get(),
                row[1].get(),
                row[2].get(),
                row[3].get(),
                row[4].get(),
                row[5].get(),
                row[6].get(),
                row[7].get(),
            ],
            col_addr,
        );
        if write_enable.get() {
            // write logic should be done in multiplexer, but it's hard to simulate.
            let cell = match ThreeToEightDecoder::exec(col_addr[0], col_addr[1], col_addr[2]) {
                (BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => &mut row[0],
                (BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => &mut row[1],
                (BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0) => &mut row[2],
                (BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0, BIT_0) => &mut row[3],
                (BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0, BIT_0) => &mut row[4],
                (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0, BIT_0) => &mut row[5],
                (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1, BIT_0) => &mut row[6],
                (BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_0, BIT_1) => &mut row[7],
                _ => unreachable!(),
            };
            cell.set(data);
            cell.get()
        } else {
            bit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dram_array() {
        let mut array = Array::new();
        // raw addr 000, col addr 000
        assert_eq!(
            array.exec([BIT_0, BIT_0, BIT_0], [BIT_0, BIT_0, BIT_0], BIT_0, BIT_0),
            BIT_0
        );
        // write bit 1
        array.exec([BIT_0, BIT_0, BIT_0], [BIT_0, BIT_0, BIT_0], BIT_1, BIT_1);
        // read bit should be 1
        assert_eq!(
            array.exec([BIT_0, BIT_0, BIT_0], [BIT_0, BIT_0, BIT_0], BIT_0, BIT_0),
            BIT_1
        );
        // write bit 0
        array.exec([BIT_0, BIT_0, BIT_0], [BIT_0, BIT_0, BIT_0], BIT_1, BIT_0);
        // read bit should be 0
        assert_eq!(
            array.exec([BIT_0, BIT_0, BIT_0], [BIT_0, BIT_0, BIT_0], BIT_0, BIT_0),
            BIT_0
        );

        // raw addr 100, col addr 100
        assert_eq!(
            array.exec([BIT_1, BIT_0, BIT_0], [BIT_1, BIT_0, BIT_0], BIT_0, BIT_0),
            BIT_0
        );
        array.exec([BIT_1, BIT_0, BIT_0], [BIT_1, BIT_0, BIT_0], BIT_1, BIT_1);
        assert_eq!(
            array.exec([BIT_1, BIT_0, BIT_0], [BIT_1, BIT_0, BIT_0], BIT_0, BIT_0),
            BIT_1
        );
        array.exec([BIT_1, BIT_0, BIT_0], [BIT_1, BIT_0, BIT_0], BIT_1, BIT_0);
        assert_eq!(
            array.exec([BIT_1, BIT_0, BIT_0], [BIT_1, BIT_0, BIT_0], BIT_0, BIT_0),
            BIT_0
        );

        // raw addr 110, col addr 110
        assert_eq!(
            array.exec([BIT_1, BIT_1, BIT_0], [BIT_1, BIT_1, BIT_0], BIT_0, BIT_0),
            BIT_0
        );
        array.exec([BIT_1, BIT_1, BIT_0], [BIT_1, BIT_1, BIT_0], BIT_1, BIT_1);
        assert_eq!(
            array.exec([BIT_1, BIT_1, BIT_0], [BIT_1, BIT_1, BIT_0], BIT_0, BIT_0),
            BIT_1
        );
        array.exec([BIT_1, BIT_1, BIT_0], [BIT_1, BIT_1, BIT_0], BIT_1, BIT_0);
        assert_eq!(
            array.exec([BIT_1, BIT_1, BIT_0], [BIT_1, BIT_1, BIT_0], BIT_0, BIT_0),
            BIT_0
        );

        // raw addr 111, col addr 111
        assert_eq!(
            array.exec([BIT_1, BIT_1, BIT_1], [BIT_1, BIT_1, BIT_1], BIT_0, BIT_0),
            BIT_0
        );
        array.exec([BIT_1, BIT_1, BIT_1], [BIT_1, BIT_1, BIT_1], BIT_1, BIT_1);
        assert_eq!(
            array.exec([BIT_1, BIT_1, BIT_1], [BIT_1, BIT_1, BIT_1], BIT_0, BIT_0),
            BIT_1
        );
        array.exec([BIT_1, BIT_1, BIT_1], [BIT_1, BIT_1, BIT_1], BIT_1, BIT_0);
        assert_eq!(
            array.exec([BIT_1, BIT_1, BIT_1], [BIT_1, BIT_1, BIT_1], BIT_0, BIT_0),
            BIT_0
        );
    }
}
