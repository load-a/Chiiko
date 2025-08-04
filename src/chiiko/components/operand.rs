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
    pub fn is_address(&self) -> bool {
        matches!(
        self, 
        Operand::IndirectRegister(_) | Operand::ZeroPageAddress(_) | 
        Operand::IndirectZeroPageAddress(_) | Operand::MemoryAddress(_) | 
        Operand::IndirectMemoryAddress(_))
    }

    pub fn is_register(&self) -> bool {
        matches!(self, Operand::Register(_))
    }

    pub fn is_jump(&self) -> bool {
        match self {
            Operand::JumpAddress(_) | Operand::MemoryAddress(_) => true,
            Operand::Register(register_code) => match register_code {
                9..=11 => true,
                _ => false
            },
            _ => false
        }
    }
}
