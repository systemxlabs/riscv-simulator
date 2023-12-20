pub mod bit;
pub mod byte;
pub mod word;

pub use bit::*;
pub use byte::*;
pub use word::*;

pub fn parse_bits(s: &str) -> Vec<Bit> {
    let mut bits = Vec::new();
    for c in s.chars() {
        bits.push(Bit::from_str(c.to_string().as_str()))
    }
    bits
}
