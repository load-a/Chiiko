use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum AluError {
    #[error("Failed to fetch instruction")]
    CannotFetchInstruction,
    
    #[error("Cannot divide by zero")]
    DivisionByZero,
    
    #[error("{0} must take a Register Pair and the Implied Accumulator as operands")]
    LongModeError(String),

    #[error("Incorrect Mode: {0}")]
    ModeError(String)
}
