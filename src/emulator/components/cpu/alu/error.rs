use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum AluError {
    #[error("Failed to fetch instruction: {0}")]
    CannotFetchInstruction(String),
    
    #[error("Cannot divide by zero")]
    DivisionByZero,
    
    #[error("{0} must take a Register Pair and the Implied Accumulator as operands")]
    LongModeError(String),

    #[error("Incorrect Mode: {0}")]
    ModeError(String),

    #[error("Operand must be an address")]
    MissingAddress,

    #[error("Input was not numeric")]
    NonNumericInput,

    #[error("Invalid UTF-8 at address: {0}")]
    CannotReadString(u16),
}
