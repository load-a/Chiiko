use std::fs;
use std::env;
use std::fmt;

use crate::assembler::source::SourceError;

const FILE_POSITION: usize = 1;

#[derive(Default, Debug, PartialEq)]
pub struct Source {
    pub(crate) path: String,
    pub(crate) raw: String,
    pub(crate) position: usize,
    pub(crate) line: String,
}

impl Source {
    #[cfg(test)]
    pub fn from_str(raw_input: &str) -> Result<Self, SourceError> {
        Ok(Self { 
            path: "TEST FILE".to_string(),
            raw: raw_input.to_string(), 
            position: 0,
            line: String::new() })
    }

    #[cfg(not(test))]
    pub fn from_args() -> Result<Self, SourceError> {
        let filename = env::args().nth(FILE_POSITION).ok_or(SourceError::MissingFile)?;
        Self::from_file(&filename)
    }

    pub fn from_file(filename: &str) -> Result<Self, SourceError> {
        let raw = fs::read_to_string(filename)
            .map_err(|error| SourceError::CannotRead(format!("{} \n{:?}", filename, error)))?;

        Ok(Self { 
            path: filename.to_string(),
            raw: raw, 
            position: 0, 
            line: String::new() 
        })
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Raw Source: \n{}", self.raw)
    }
}
