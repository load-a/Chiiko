use std::fmt;

use crate::register::RegisterError;
use crate::mode::ModeError;

#[derive(Debug)]
pub enum ChiikoError {
    Register(RegisterError),
    Operand,
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

impl fmt::Display for ChiikoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChiikoError::Register(err) => write!(f, "{}", err),
            ChiikoError::Operand => write!(f, "Operand Error"),
            ChiikoError::Mode(err) => write!(f, "{}", err),
            ChiikoError::Operation => write!(f, "Operation Error"),
        }
    }
}
