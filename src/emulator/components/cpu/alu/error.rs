use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum AluError {
    #[error("Failed to fetch instruction")]
    CannotFetchInstruction,
    #[error("Cannot divide by zero")]
    DivisionByZero,
    #[error("Long Operation {0} cannot change mode")]
    LongModeError(String)
}
