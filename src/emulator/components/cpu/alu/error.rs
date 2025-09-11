use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum AluError {
    #[error("Failed to fetch instruction")]
    CannotFetchInstruction
}
