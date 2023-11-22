use crate::mem::cell::Cell;

pub const ARRAY_ROWS: usize = 8;
pub const ARRAY_COLS: usize = 8;

pub struct Array(Box<[[Cell; ARRAY_COLS]; ARRAY_ROWS]>);
