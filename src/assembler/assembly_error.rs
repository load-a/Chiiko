use std::fmt;

pub enum AssemblyError {
    UnmatchedBracket,
    CannotReadFile(String),
    MissingFile,
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyError::CannotReadFile(filename) => 
                write!(f, "Cannot read file: {:?}", filename),
            AssemblyError::MissingFile => write!(f, "Missing input file name"),
            _ => write!(f, "Invalid Assembly Error")
        }
    }
}