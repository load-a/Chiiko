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

impl Operand {
    fn is_address(&self) -> bool {
        matches!(
        self, 
        Operand::IndirectRegister(_) | Operand::ZeroPageAddress(_) | 
        Operand::IndirectZeroPageAddress(_) | Operand::MemoryAddress(_) | 
        Operand::IndirectMemoryAddress(_))
    }

    fn is_register(&self) -> bool {
        matches!(self, Operand::Register(_))
    }

    fn is_jump(&self) -> bool {
        matches!(self, Operand::JumpAddress(_))
    }
}
