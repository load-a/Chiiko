use thiserror::Error;
use std::fmt;

use crate::emulator::components::{chip::ChipError, cpu::CpuError};

#[derive(Debug, Error)]
pub enum EmulatorError {
    #[error("Cannot Find: {0}")]
    CannotFind(String),

    #[error("Cannot Send: {0}")]
    CannotSend(String),

    #[error("Import too large for: {0}")]
    ImportOverload(String),

    #[error("Invalid write: {0}")]
    InvalidWrite(String),

    #[error("Cannot read: {0}")]
    InvalidRead(String),

    #[error("Invalid Source: {0}")]
    InvalidSource(String),

    #[error("Invalid destination: {0}")]
    InvalidDestination(String),

    #[error("Cannot fetch: {0}")]
    CannotFetch(String),

    #[error("Cannot resolve: {0}")]
    CannotResolve(String),

    #[error(transparent)]
    Chip(#[from] ChipError),

    #[error(transparent)]
    Cpu(#[from] CpuError),
}
