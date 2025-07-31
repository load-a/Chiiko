#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    None,
    Value(u8),
    Register(u8),
    IndirectRegister(u8),
    ZeroPageAddress(u8),
    IndirectZeroPageAddress(u8),
    MemoryAddress(u16),
    IndirectMemoryAddress(u16),
    JumpAddress(u16),
    Error,
}
