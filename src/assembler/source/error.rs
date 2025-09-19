use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Cannot find file: {0}")]
    MissingFile(String),

    #[error("Cannot read file: {0}")]
    CannotRead(String),
}
