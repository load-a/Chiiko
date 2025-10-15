use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Cursor has exceeded system integer limit.")]
    PositionOverflow,

    #[error("Expected type: {0}; Found {1}.", expected, found)]
    UnexpectedToken {expected: String, found: String},

    #[error("Expected ID: {0}; Found {1}.", expected, found)]
    UnexpectedID {expected: String, found: String},

    #[error("Expected tokens, found End of File.")]
    UnexpectedEOF,
}
