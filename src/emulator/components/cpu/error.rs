use thiserror::Error;
use std::fmt;

use crate::emulator::components::chip::ChipError;

#[derive(Debug, Error)]
pub enum CpuError {
    #[error(transparent)]
    Chip(#[from] ChipError),

    #[error("Invalid read at CPU address: {0:#04X}")]
    InvalidRead(u16),

    #[error("Invalid write at CPU address: {0:#04X}")]
    InvalidWrite(u16),

    #[error("Invalid register code: {0}")]
    InvalidRegister(u8),

    #[error("Invalid register pair code: {0}")]
    InvalidRegisterPair(u8),

    #[error("Invalid single register code: {0}")]
    InvalidSingleRegister(u8),

    #[error("Cannot find source: {0}")]
    CannotFind(String),

    #[error("Cannot send to destination: {0}")]
    CannotSend(String),

    #[error("Cannot Fetch: {0}")]
    CannotFetch(String),

    #[error("End of Program ROM")]
    EndOfProgram,
}
