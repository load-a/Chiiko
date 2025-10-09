use crate::assembler::lexer::{Lexer, error::LexerError};

impl Lexer {
    pub(crate) fn extract_word(&mut self) -> Result<&str, LexerError> {
        let id = self.source.consume_word().ok_or(LexerError::NoWord)?;
        self.column += id.len();

        Ok(id)
    }

    pub(crate) fn process_whitespace(&mut self) -> Result<(), LexerError> {
        if self.source.consume() == Some('\n') {
            self.process_newline()
        } else {
            self.increment_column_counter()
        }
    }

    pub(crate) fn process_newline(&mut self) -> Result<(), LexerError> {
        self.increment_line_counter()?;
        self.reset_column_counter();
        self.record_current_line()
    }

    pub(crate) fn record_current_line(&mut self) -> Result<(), LexerError> {
        if let Some(exerpt) = self.source.peek_line() {
            self.exerpt = format!("{}. {}", self.line, exerpt);
            Ok(())
        } else {
            Err(LexerError::SourceLineOverflow)
        }
    }
}
