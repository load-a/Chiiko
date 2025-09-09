use std::fmt;

#[derive(Debug)]
pub enum ModeError {
    IllegalKey(String),
    IllegalNibble(u8),
}

impl std::error::Error for ModeError {}

impl fmt::Display for ModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModeError::IllegalKey(name) => write!(f, "Illegal Mode key: {}", name),
            ModeError::IllegalNibble(nibble) => {
                write!(f, "Illegal Mode nibble: {:#04X}", nibble)
            }
        }
    }
}
