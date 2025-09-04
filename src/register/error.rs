use std::fmt;

#[derive(Debug)]
pub enum RegisterError {
    IllegalName(String),
    IllegalCode(u8),
}

impl std::error::Error for RegisterError {}

impl fmt::Display for RegisterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegisterError::IllegalName(name) => write!(f, "Illegal Register name: {}", name),
            RegisterError::IllegalCode(code) => {
                write!(f, "Illegal Register code: {:#04X}", code)
            },
        }
    }
}
