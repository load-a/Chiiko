use std::fmt;
use crate::operand::Operand;

#[derive(Debug)]
pub enum OperandError {
    CannotExtractValue(String)
}

impl std::error::Error for OperandError {}

impl fmt::Display for OperandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperandError::CannotExtractValue(operand) => 
                write!(f, "Cannot extract Value from: {:?}", operand)
        }
    }
}
