use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum BusError {
    #[error("Bus read to unmapped address: {0:#04X}")]
    UnmappedRead(u16),

    #[error("Bus write to unmapped address: {0:#04X}")]
    UnmappedWrite(u16),
}
