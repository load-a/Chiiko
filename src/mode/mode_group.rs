#[derive(Clone, Debug, PartialEq)]
pub enum ModeGroup {
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
