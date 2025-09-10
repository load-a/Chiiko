use thiserror::Error;
use std::fmt;

use crate::emulator::components::{bus::BusError, ram::RamError, rom::RomError};

#[derive(Debug, Error)]
pub enum ChipError {
    #[error("Cannot Read: {0}")]
    CannotRead(String),

    #[error("Cannot Write: {0}")]
    CannotWrite(String),

    #[error("Cannot Tick: {0}")]
    CannotTick(String),

    #[error("Cannot Reset: {0}")]
    CannotReset(String),

    #[error(transparent)]
    Bus(#[from] BusError),

    #[error(transparent)]
    Ram(#[from] RamError),

    #[error(transparent)]
    Rom(#[from] RomError),
}
