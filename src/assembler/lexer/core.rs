use crate::assembler::{assembly_error::AssemblyError, source::Source};
use crate::assembler::lexer::{cursor::Cursor, token::Token};

#[derive(PartialEq)]
enum LexerMode {
    Normal,
    StringLiteral,
    ArrayLiteral,
    TupleLiteral,
}

pub struct Lexer<'a> {
    source: &'a str,
    cursor: Cursor<'a>,
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

        self.mode.push(LexerMode::Normal);

        while let Some(character) = self.cursor.peek() {
            let token = match self.mode.last() {
                Some(LexerMode::Normal) => {
                    if character.is_whitespace() {
                        self.cursor.advance();
                        if character == '\n' { Token::Newline } else { continue; }
                    } else if character.is_alphabetic() || character == '_' {
                        let slice = self.cursor.consume_while(|c| c.is_alphanumeric() || c == '_');

                        if self.cursor.peek() == Some(':') { 
                            self.cursor.advance();
                            Token::LabelHeader(slice)
                        } else {
                            Token::Identifier(slice)
                        }
                    } else if character.is_ascii_digit() {
                        if character == '0' {
                            match self.cursor.peek_ahead(1) {
                                Some('x') | Some('X') => {
                                    if !self.cursor.peek_ahead(2).unwrap().is_ascii_hexdigit() {
                                        Token::Error {
                                            message: "Incorrect number format".to_string(),
                                            line_and_column: self.cursor.line_and_column(),
                                            snippet: self.cursor.consume_while(|c| c != '\n'),
                                        }
                                    } else {
                                        self.cursor.advance();
                                        self.cursor.advance();
                                        Token::HexNumber(self.cursor.consume_while(
                                            |c| c.is_ascii_hexdigit()
                                        ))
                                    }
                                },
                                Some('o') | Some('O') => {
                                    if !matches!(self.cursor.peek_ahead(2).unwrap(), '0'..='7') {
                                        Token::Error {
                                            message: "Incorrect number format".to_string(),
                                            line_and_column: self.cursor.line_and_column(),
                                            snippet: self.cursor.consume_while(|c| c != '\n'),
                                        }
                                    } else {
                                        self.cursor.advance();
                                        self.cursor.advance();
                                        Token::OctalNumber(self.cursor.consume_while(
                                            |c| matches!(c, '0'..='7')
                                        ))
                                    }
                                },
                                Some('b') | Some('B') => {
                                    if !(self.cursor.peek_ahead(2) == Some('1') || 
                                            self.cursor.peek_ahead(2) == Some('0'))
                                    {
                                        Token::Error {
                                            message: "Incorrect number format".to_string(),
                                            line_and_column: self.cursor.line_and_column(),
                                            snippet: self.cursor.consume_while(|c| c != '\n'),
                                        }
                                    } else {
                                        self.cursor.advance();
                                        self.cursor.advance();
                                        Token::BinaryNumber(self.cursor.consume_while(|c| 
                                            c == '1' || c == '0'
                                        ))
                                    }
                                },
                                _ => Token::DecimalNumber(self.cursor.consume_while(
                                    |c| c.is_ascii_digit()
                                )),
                            }
                        } else {
                            Token::DecimalNumber(self.cursor.consume_while(|c| c.is_numeric()))
                        }
                    } else {
                        match character {
                            ';' => {
                                self.cursor.advance();
                                let slice = self.cursor.consume_while(|c| c != '\n');
                                Token::Comment(slice)
                            },
                            ':' => {
                                self.cursor.advance();
                                let slice = self.cursor.consume_while(|c| 
                                    c.is_alphanumeric() || c == '_'
                                );
                                Token::JumpLabel(slice)
                            },
                            '#' => {
                                self.cursor.advance();
                                let slice = self.cursor.consume_while(|c| c.is_alphanumeric());
                                Token::Directive(slice)
                            },
                            '$' => {
                                self.cursor.advance();
                                let slice = self.cursor.consume_while(|c| c.is_alphanumeric());
                                Token::DirectAddress(slice)
                            },
                            '@' => {
                                self.cursor.advance();
                                let slice = self.cursor.consume_while(|c| c.is_alphanumeric());
                                Token::IndirectAddress(slice)
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
                                self.mode.push(LexerMode::StringLiteral);
                                self.cursor.advance();
                                Token::Quote
                            },
                            _ => {
                                Token::Error { 
                                    message: "Unknown Token Error".to_string(), 
                                    line_and_column: self.cursor.line_and_column(), 
                                    snippet: self.cursor.consume_while(|c| c != '\n') 
                                }
                            },
                        }
                    }
                },
                Some(LexerMode::StringLiteral) => {
                    if character == '"' {
                        self.mode.pop();
                        self.cursor.advance();
                        Token::Quote
                    } else {
                        let slice = self.cursor.consume_while(|c| c != '"');
                        Token::String(slice)
                    }
                },
                Some(LexerMode::ArrayLiteral) => {
                    if character == '\n' {
                        self.cursor.advance();
                    } 

                    if character.is_whitespace() {
                        self.cursor.advance();
                        continue;
                    }
                    match character {
                        ',' => {
                            self.cursor.advance();
                            Token::Comma
                        },
                        ']' => {
                            self.cursor.advance();
                            self.mode.pop();
                            Token::CloseBracket
                        },
                        _ => Token::Element(
                        self.cursor.consume_while(|c| !matches!(c, ',' | ']' | '\n'))
                        )
                    }
                },
                Some(LexerMode::TupleLiteral) => {
                    if character.is_whitespace() {
                        self.cursor.advance();
                        continue;
                    }
                    match character {
                        ',' => {
                            self.cursor.advance();
                            Token::Comma
                        },
                        ')' => {
                            self.cursor.advance();
                            self.mode.pop();
                            Token::CloseParen
                        },
                        _ => Token::Element(
                        self.cursor.consume_while(|c| c != ',' && c != ')')
                        )
                    }
                },
                None => Token::EndOfFile,
            };

            tokens.push(token);
        }

        tokens.push(Token::EndOfFile);
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
