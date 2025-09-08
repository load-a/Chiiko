use std::fmt;

#[derive(Debug)]
pub enum EmulatorError {
    CannotFind(String),
    CannotSend(String),
    ImportOverload(String),
    InvalidWrite(String),
    InvalidRead(String),
    InvalidSource(String),
    InvalidDestination(String),
    CannotFetch(String),
    CannotResolve(String),
}

impl std::error::Error for EmulatorError {}

impl fmt::Display for EmulatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmulatorError::CannotFind(source) => write!(f, "Cannot Find: {}", source),
            EmulatorError::CannotSend(destination) => write!(f, "Cannot Send: {}", destination),
            EmulatorError::InvalidWrite(destination) => {
                write!(f, "Invalid write: {}", destination)
            },
            EmulatorError::InvalidRead(address) => write!(f, "Cannot read: {}", address),
            EmulatorError::ImportOverload(target) => write!(f, "Import too large for: {}", target),
            EmulatorError::InvalidSource(source) => write!(f, "Invalid Source: {}", source),
            EmulatorError::InvalidDestination(destination) => {
                write!(f, "Invalid destination: {}", destination)
            },
            EmulatorError::CannotFetch(reason) => write!(f, "Cannot fetch: {}", reason),
            EmulatorError::CannotResolve(address) => write!(f, "Cannot resolve: {}", address),
        }
    }
}
