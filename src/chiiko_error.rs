use std::fmt;

use crate::register::RegisterError;
use crate::operation::OperationError;
use crate::mode::ModeError;
use crate::operand::OperandError;
use crate::emulator::EmulatorError;

#[derive(Debug)]
pub enum ChiikoError {
    Operation(OperationError),
    Mode(ModeError),
    Operand(OperandError),
    Register(RegisterError),
    Emulator(EmulatorError),
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

impl From<OperationError> for ChiikoError {
    fn from(error: OperationError) -> Self {
        ChiikoError::Operation(error)
    }
}

impl From<EmulatorError> for ChiikoError {
    fn from(error: EmulatorError) -> Self {
        ChiikoError::Emulator(error)
    }
}

impl fmt::Display for ChiikoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChiikoError::Register(err) => write!(f, "{}", err),
            ChiikoError::Operand(err) => write!(f, "{}", err),
            ChiikoError::Mode(err) => write!(f, "{}", err),
            ChiikoError::Operation(err) => write!(f, "{}", err),
            ChiikoError::Emulator(err) => write!(f, "{}", err),
        }
    }
}
