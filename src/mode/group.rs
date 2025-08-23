#[derive(Clone, Debug, PartialEq)]
pub enum Group {
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
    Error,
}
