#[derive(Debug)]
pub enum ModeKey {
    NoOperand,
    Value,
    Register,
    IndirectRegister,
    ZeroPage,
    IndirectZeroPage,
    DirectAddress,
    IndirectAddress,
    JumpAddress,
    Accumulator,
    Low,
    High,
    Error(String),
}
