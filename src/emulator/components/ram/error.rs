use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum RamError {
    #[error("Read out of bounds: RAM address {0:#04X}")]
    ReadOutOfBounds(u16),

    #[error("Write out of bounds: RAM address {0:#04X}")]
    WriteOutOfBounds(u16),
}
