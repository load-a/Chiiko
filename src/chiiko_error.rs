use std::fmt;

use crate::register::RegisterError;
use crate::mode::ModeError;
use crate::operand::OperandError;

#[derive(Debug)]
pub enum ChiikoError {
    Register(RegisterError),
    Operand(OperandError),
    Mode(ModeError),
    Operation,
}

impl std::error::Error for ChiikoError {}

impl From<RegisterError> for ChiikoError {
    fn from(error: RegisterError) -> Self {
        ChiikoError::Register(error)
    }
}

impl From<ModeError> for ChiikoError {
    fn from(error: ModeError) -> Self {
        ChiikoError::Mode(error)
    }
}

impl From<OperandError> for ChiikoError {
    fn from(error: OperandError) -> Self {
        ChiikoError::Operand(error)
    }
}

impl fmt::Display for ChiikoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChiikoError::Register(err) => write!(f, "{}", err),
            ChiikoError::Operand(err) => write!(f, "{}", err),
            ChiikoError::Mode(err) => write!(f, "{}", err),
            ChiikoError::Operation => write!(f, "Operation Error"),
        }
    }
}
