use crate::assembler::assembly_error::AssemblyError;

#[derive(Clone, PartialEq, Debug)]
pub enum AssemblerOperand {
    Number(u16),
    Register(String),
    DirectAddress(String),
    IndirectAddress(String),
    String(String),
    JumpAddress(String),
    Identifier(String),
    Error(String),
    Placeholder(String),
    StartCount(usize),
    EndCount,
    NamedElement {name: String, value: u8},
    NoOperand,
}

impl AssemblerOperand {
    pub fn string(&self) -> Result<String, AssemblyError> {
        match self {
            Self::Register(string) | Self::DirectAddress(string) | Self::IndirectAddress(string) |
            Self::String(string) | Self::JumpAddress(string) | Self::Identifier(string) => {
                Ok(string.to_string())
            },
            Self::NamedElement { name, .. } => {
                Ok(name.to_string())
            }, 
            _ => Err(AssemblyError::NoOperandString)
        }
    }
}
