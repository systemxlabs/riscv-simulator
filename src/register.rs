#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RegisterKind {
    /// hardwired zero
    Zero,
    /// return address
    RA,
    /// stack pointer
    SP,
    /// global pointer
    GP,
    /// thread pointer
    TP,
    /// temporary
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    /// function argument
    A0,  // return value
    A1,  // return value
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    /// saved register
    S0,  // frame pointer
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
}
