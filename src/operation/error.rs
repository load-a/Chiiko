use std::fmt;

#[derive(Debug)]
pub enum OperationError {
    IllegalMnemonic(String),
    IllegalOpcode(u8),
}

impl std::error::Error for OperationError {}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationError::IllegalMnemonic(id) => write!(f, "Illegal Mnemonic: {}", id),
            OperationError::IllegalOpcode(code) => write!(f, "Illegal Opcode: {:#04X}", code),
        }
    }
}
