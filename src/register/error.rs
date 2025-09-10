use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("Illegal Register name: {0}")]
    IllegalName(String),

    #[error("Illegal Register code: {0:#04X}")]
    IllegalCode(u8),
}
