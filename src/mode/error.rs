use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum ModeError {
    #[error("Illegal Mode key: {0}")]
    IllegalKey(String),

    #[error("Illegal Mode nibble: {0:#04X}")]
    IllegalNibble(u8),
}
