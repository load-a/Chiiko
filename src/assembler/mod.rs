mod core;
pub mod lexer;
mod token;
pub mod assembly_error;
mod parser;
mod encoder;
mod source;

#[cfg(test)]
mod test;

// pub use core::Source;
pub use crate::assembler::lexer::Lexer;