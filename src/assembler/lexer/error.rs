use thiserror::Error;
use std::fmt;
use crate::assembler::source::SourceError;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Cursor has surpassed Source lines.")]
    SourceLineOverflow,

    #[error("Cursor has surpassed Source characters.")]
    SourceCharacterOverflow,

    #[error("A String Literal has not been terminated.")]
    UnfinishedStringLiteral,

    #[error("Line counter has exceeded system integer limit.")]
    LineOverflow,

    #[error("Column counter has exceeded system integer limit.")]
    ColumnOverflow,

    #[error("Tried to consume word; found None.")]
    NoWord,

    #[error("Tried to consume line; found None.")]
    NoLine,

    #[error("Unclosed Byte Array")]
    UnclosedByteArray,

    #[error("Unclosed Mode Signature")]
    UnclosedModeSignature,

    // #[error(transparent)]
    // Source(#[from] SourceError),
}
