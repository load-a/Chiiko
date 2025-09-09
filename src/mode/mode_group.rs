#[derive(Clone, Copy, Debug, PartialEq)]
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
    AnyOperand, // AnyOperand and Error can only be used by the assembler.
    Error,      //
}
