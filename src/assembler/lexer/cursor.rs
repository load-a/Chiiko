use crate::assembler::lexer::{error::LexerError, Lexer};

impl Lexer {
    pub(crate) fn reset_column_counter(&mut self) {
        self.column = 1;
    }

    pub(crate) fn advance_cursor(&mut self) -> Result<(), LexerError> {
        if self.source.consume().is_some() {
            self.increment_column_counter()
        } else {
            Err(LexerError::SourceCharacterOverflow)
        }
    }

    pub(crate) fn increment_column_counter(&mut self) -> Result<(), LexerError> {
        let (result, overflow) = self.column.overflowing_add(1);

        if overflow {
            Err(LexerError::ColumnOverflow)
        } else {
            self.column = result;
            Ok(())
        }
    }

    pub(crate) fn increment_line_counter(&mut self) -> Result<(), LexerError> {
        let (result, overflow) = self.line.overflowing_add(1);

        if overflow {
            Err(LexerError::LineOverflow)
        } else {
            self.line = result;
            Ok(())
        }
    }
}
