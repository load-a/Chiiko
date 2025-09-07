use std::fmt;

#[derive(Debug)]
pub enum EmulatorError {
    CannotFind(String),
    CannotSend,
    ImportOverload(String),
    InvalidWrite(String),
    InvalidRead(String),
    InvalidSource(String)
}

impl std::error::Error for EmulatorError {}

impl fmt::Display for EmulatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmulatorError::CannotFind(source) => write!(f, "Cannot Find: {}", source),
            EmulatorError::CannotSend => write!(f, "Cannot Send ()"),
            EmulatorError::InvalidWrite(destination) => {
                write!(f, "Invalid write: {}", destination)
            },
            EmulatorError::InvalidRead(address) => write!(f, "Cannot read: {}", address),
            EmulatorError::ImportOverload(target) => write!(f, "Import too large for: {}", target),
            EmulatorError::InvalidSource(source) => write!(f, "Invalid Source: {}", source),
        }
    }
}
