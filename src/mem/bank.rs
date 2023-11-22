use crate::mem::array::Array;

pub const ARRAY_COUNT: usize = 8;

pub struct Bank(Box<[Array; ARRAY_COUNT]>);
