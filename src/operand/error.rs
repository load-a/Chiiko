use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum OperandError {
    #[error("Cannot extract Value from: {0:?}")]
    CannotExtractValue(String),

    #[error("Invalid Operand Register: {0}")]
    InvalidRegister(String),
}
