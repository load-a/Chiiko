use crate::assembler::source::{Source, SourceError};

impl Source {
    pub(crate) fn peek_line(&self) -> &str {
        self.peek_while(|c| c != '\n')
    }

    pub(crate) fn peek_while<F>(&self, mut f: F) -> &str 
    where 
        F: FnMut(char) -> bool 
    {
        let start = self.position;
        let mut end = start;

        while let Some(character) = self.raw[end..].chars().next() {
            if !f(character) { break; }
            end += character.len_utf8();
        }

        &self.raw[start..end]
    }

    pub(crate) fn peek_ahead(&self, mut offset: usize) -> Option<char> {
        let mut position = self.position;
        let mut chars = self.raw[position..].chars();

        while offset > 0 {
            let character = chars.next()?;
            position += character.len_utf8();
            offset -= 1;
        }

        chars.next()
    }

    pub(crate) fn consume_line(&mut self) -> &str {
        self.consume_while(|c| c != '\n')
        // Does not consume the trailing newline character
    }

    pub(crate) fn consume_while<F>(&mut self, mut f: F) -> &str 
    where 
        F: FnMut(char) -> bool 
    {
        let start = self.position;

        while let Some(character) = self.peek() {
            if !f(character) { break; }
            self.consume();
        }

        &self.raw[start..self.position]
    }

    pub(crate) fn consume(&mut self) -> Option<char> {
        if let Some(character) = self.peek() {
            self.position += character.len_utf8();
            Some(character)
        } else {
            None
        }
    }

    pub(crate) fn peek(&self) -> Option<char> {
        self.raw[self.position..].chars().next()
    }

    pub(crate) fn end_of_file(&self) -> bool {
        self.position == self.raw.len()
    }
}
