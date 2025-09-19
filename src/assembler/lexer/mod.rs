mod core;
mod error;
mod cursor;
pub mod token;

#[cfg(test)]
mod test;

pub use core::Lexer;
pub use error::LexerError;
