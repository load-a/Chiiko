use crate::assembler::{assembly_error::AssemblyError, source::Source, token::Token};
use crate::assembler::lexer::cursor::Cursor;

enum LexerMode {
    Normal,
    StringLiteral,
    ArrayLiteral,
    TupleLiteral,
}

pub struct Lexer<'a> {
    source: &'a str,
    cursor: Cursor,
    mode: Vec<LexerMode>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source,
            cursor: Cursor::new(source),
            mode: Vec::with_capacity(4),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let buffer = String::new();

        while let Some(character) = self.cursor.peek() {
            let token = if character.is_whitespace() {
                self.cursor.advance();
                if character == '\n' && self.mode.last() != Some(&LexerMode::ArrayLiteral) { 
                    Token::Newline 
                } else { 
                    continue; 
                }
            } else if character.is_alphabetic() || character == '_' {
                let slice = self.cursor.consume_while(|c| c.is_alphanumeric() || c == '_');

                if self.cursor.peek() == ':' { 
                    self.cursor.advance();
                    Token::LabelHeader(slice)
                } else {
                    Token::Identifier(slice)
                }
            } else if character.is_numeric() {
                let slice = self.cursor.consume_while(|c| c.is_numeric());
                Token::Number(slice)
            } else {
                match character {
                    ':' => {
                        self.cursor.advance();
                        let slice = self.cursor.consume_while(|c| c.is_alphanumeric() || c == '_');
                        Token::LabelAddress(slice)
                    },
                    '#' => {
                        self.cursor.advance();
                        let slice = self.cursor.consume_while(|c| c.is_alphanumeric());
                        Token::Directive(slice)
                    },
                    ',' => {
                        self.cursor.advance();
                        Token::Comma
                    },
                    '[' => {
                        self.mode.push(LexerMode::ArrayLiteral);
                        self.cursor.advance();
                        Token::OpenBracket
                    },
                    ']' => {
                        self.mode.pop();
                        self.cursor.advance();
                        Token::CloseBracket
                    },
                    '{' => {
                        self.cursor.advance();
                        Token::OpenBrace
                    },
                    '}' => {
                        self.cursor.advance();
                        Token::CloseBrace
                    },
                    '(' => {
                        self.mode.push(LexerMode::TupleLiteral);
                        self.cursor.advance();
                        Token::OpenParen
                    },
                    ')' => {
                        self.mode.pop();
                        self.cursor.advance();
                        Token::CloseParen
                    },
                    '"' => {
                        if self.mode.last() == Some(&LexerMode::StringLiteral) {
                            self.mode.pop();
                        } else {
                            self.mode.push(LexerMode::StringLiteral);
                        }
                        self.cursor.advance();
                        Token::Quote
                    },
                    '=' => {
                        self.cursor.advance();
                        Token::AssignmentOperator
                    },
                    _ => {
                        let slice = self.cursor.consume_while(|c| c != '\n');
                        Token::Error("Unknown Token Error", self.cursor.line, cursor.column, slice)
                    },
                }
            };

            tokens.push(token);
        }

            tokens.push(Token::EndOfFile)
            tokens
    }

    fn slice(&self, start: usize, end: usize) -> &'a str {
        self.source
            .get(start..end)
            .unwrap_or_else(|| {
                panic!(
                "Lexer Error: tried to slice invalid UTF-8 boundaries ({}..{})", start, end
                )
            })
    }
}