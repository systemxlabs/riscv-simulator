use crate::mem::bank::Bank;

pub const BANK_COUNT: usize = 8;

pub struct Chip(Box<[Bank; BANK_COUNT]>);
