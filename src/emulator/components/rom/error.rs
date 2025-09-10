use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum RomError {
    #[error("ROM address: {0:#04X}")]
    ReadOutOfBounds(u16),
    
    #[error("ROM Write attempted at address: {0:#04X}")]
    AttemptedWrite(u16),
}
