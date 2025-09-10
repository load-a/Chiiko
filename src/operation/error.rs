use thiserror::Error;
use std::fmt;

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Illegal Mnemonic: {0}")]
    IllegalMnemonic(String),

    #[error("Illegal Opcode: {0:#04X}")]
    IllegalOpcode(u8),
}
