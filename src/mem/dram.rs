use crate::mem::chip::Chip;

pub const CHIP_COUNT: usize = 4;

pub struct Dram(Box<[Chip; CHIP_COUNT]>);
