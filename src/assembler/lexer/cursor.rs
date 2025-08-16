pub struct Cursor<'a> {
    source: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, position: 0, line: 1, column: 1 }
    }

    pub fn consume_while<F>(&mut self, mut f: F) -> &'a str 
    where
        F: FnMut(char) -> bool
    {
        let start = self.position;

        while let Some(character) = self.peek() {
            if !f(character) { break; }
            self.advance();
        }

        &self.source[start..self.position]
    }

    pub fn advance(&mut self) -> Option<char> {
        if let Some(character) = self.peek() {
            let char_length = character.len_utf8();

            self.position += char_length;

            if character == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            Some(character)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    pub fn byte_position(&self) -> usize {
        self.position
    }

    pub fn line_and_column(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}