use thiserror::Error;
use std::fmt;

use crate::emulator::EmulatorError;
use crate::mode::ModeError;
use crate::operand::OperandError;
use crate::operation::OperationError;
use crate::register::RegisterError;

#[derive(Debug, Error)]
pub enum ChiikoError {
    #[error(transparent)]
    Operation(#[from] OperationError),
    
    #[error(transparent)]
    Mode(#[from] ModeError),
    
    #[error(transparent)]
    Operand(#[from] OperandError),
    
    #[error(transparent)]
    Register(#[from] RegisterError),
    
    #[error(transparent)]
    Emulator(#[from] EmulatorError),
}
