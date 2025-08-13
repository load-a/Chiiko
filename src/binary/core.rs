use std::fs;
use std::env;
use std::fmt;

#[derive(Default, Debug, PartialEq)]
pub struct Binary {
    pub raw: String,
    pub bytes: Vec<u8>,
}

impl Binary {
    #[cfg(not(test))]
    pub fn from_args() -> Result<Self, String> {
        let filename = env::args().nth(1).ok_or("Missing Input File name")?;
        Self::from_file(&filename)
    }

    pub fn from_file(filename: &str) -> Result<Self, String> {
        let raw = fs::read_to_string(filename)
            .map_err(|error| format!("Failed to read file: {} {}", filename, error))?;
        let bytes: Vec<u8> = Self::parse_bytes(&raw);

        Ok(Self { raw, bytes })
    }

    #[cfg(test)]
    pub fn from_str(raw_input: &str) -> Result<Self, String> {
        let bytes = Self::parse_bytes(&raw_input);

        Ok(Self {
            raw: raw_input.to_string(),
            bytes: bytes,
        })
    }

    fn parse_bytes(file: &str) -> Vec<u8> {
        file.split_whitespace()
        .map(|byte| {
            u8::from_str_radix(byte, 2)
            .map_err(|error| format!("Invalid byte '{}': {}", byte, error))
        }
            .unwrap())
        .collect()
    }

    pub fn print_bytes(&self) {
        for byte in &self.bytes {
            println!("{}", byte.to_string())
        }
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Raw Bytes: {}\nBytes: {:?}", self.raw, self.bytes)
    }
}
