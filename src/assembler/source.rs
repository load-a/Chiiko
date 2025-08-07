use std::fs;
use std::env;
use std::fmt;

use crate::assembler::{assembly_error::AssemblyError, lexer::Lexer, token::Token};

#[derive(Default, Debug, PartialEq)]
pub struct Source {
    pub raw: String,
    pub lines: Vec<String>,
}

impl Source {
    #[cfg(not(test))]
    pub fn from_args() -> Result<Self, AssemblyError> {
        let filename = env::args().nth(1).ok_or(AssemblyError::MissingFile)?;
        Self::from_file(filename)
    }

    pub fn from_file(filename: &str) -> Result<Self, AssemblyError> {
        let raw = fs::read_to_string(filename)
            .map_err(|error| AssemblyError::CannotReadFile(filename.to_string()))?;
        let lines: Vec<String> = Self::split_lines(&raw);

        Ok(Self { raw, lines })
    }

    #[cfg(test)]
    pub fn from_str(raw_input: &str) -> Result<Self, AssemblyError> {
        let lines = Self::split_lines(&raw_input);

        Ok(Self {
            raw: raw_input.to_string(),
            lines: lines,
        })
    }

    fn split_lines(file: &str) -> Vec<String> {
        file.to_uppercase()
        .lines()
        .map(|l| l.to_string())
        .collect()
    }

    pub fn print_lines(&self) {
        for line in &self.lines {
            println!("{}", line)
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Raw Source: \n{}\n\nLines: \n{:?}", self.raw, self.lines)
    }
}
