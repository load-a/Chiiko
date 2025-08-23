#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CpuOperand {
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

impl CpuOperand {
    pub fn is_address(&self) -> bool {
        matches!(
        self, 
        CpuOperand::IndirectRegister(_) | CpuOperand::ZeroPageAddress(_) | 
        CpuOperand::IndirectZeroPageAddress(_) | CpuOperand::MemoryAddress(_) | 
        CpuOperand::IndirectMemoryAddress(_))
    }

    pub fn is_register(&self) -> bool {
        matches!(self, CpuOperand::Register(_))
    }

    pub fn is_register_pair(&self) -> bool {
        if let CpuOperand::Register(code) = self {
            *code > 8 && *code < 12
        } else { false }
    }

    pub fn is_jump(&self) -> bool {
        match self {
            CpuOperand::JumpAddress(_) | CpuOperand::MemoryAddress(_) => true,
            CpuOperand::Register(register_code) => match register_code {
                9..=11 => true,
                _ => false
            },
            _ => false
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, CpuOperand::None)
    }
}
