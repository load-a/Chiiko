use thiserror::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    CannotRecordEmptyLine,
    NoCharacterToConsume,
    UnfinishedStringLiteral,
}
