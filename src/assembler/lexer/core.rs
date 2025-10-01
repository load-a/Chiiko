use crate::numeral_parser::numeral_parser;
use crate::mode::Mode;
use crate::assembler::source::Source;
use crate::assembler::lexer::{token::Token, token::TokenVariant, token::TokenVariant::*, LexerError};

#[derive(PartialEq)]
enum LexerState {
    Normal,
    StringLiteral,
    ArrayLiteral,
    ModeSignature,
}

pub struct Lexer {
    source: Source,
    mode: Vec<LexerState>,
    line: usize,
    column: usize,
    exerpt: String,
}

impl Lexer {
    pub fn new(source: Source) -> Self {
        Self {
            source: source,
            mode: Vec::with_capacity(4),
            line: 1,
            column: 1,
            exerpt: String::new(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        let buffer = String::new();

        self.mode.push(LexerState::Normal);
        self.record_current_line();

        while let Some(character) = self.source.peek() {
            match self.mode.last() {
                Some(LexerState::Normal) => {
                    if character.is_whitespace() || character == ',' {
                        self.process_whitespace();
                        continue; 
                    } else {
                        tokens.push(self.lex_normal(character))
                    }
                }
                Some(LexerState::ModeSignature) => {
                    if character.is_whitespace() {
                        self.process_whitespace();
                        continue; 
                    } else {
                        tokens.push(self.lex_mode(character))
                    }
                }
                _ => ()
            }
        }

        tokens.push(Token::end_of_file(self.position()));

        Ok(tokens)
    }

    fn lex_mode(&mut self, character: char) -> Token {
        let position = self.position();

        if character == ',' {
            self.advance_character();
            return Token::comma(position)
        } else if character == ')' {
            self.mode.pop();
            self.advance_character();
            return Token::new(TokenVariant::CloseParen, position, ")")
        }

        let word = self.extract_word().to_string();

        if Mode::is_mode_key(word.to_uppercase().as_str()) {
            Token::new(TokenVariant::ModeKey, position, &format!("{}", word))
        } else {
            self.token_error(position, word.as_str(), "Invalid Mode Signature")
        }
    }

    fn token_error(&self, position: (usize, usize), id: &str, message: &str) -> Token {
        Token::error(
            position, 
            format!("{}:\nExerpt: {}", message, self.exerpt), 
            id
        )
    }

    fn lex_normal(&mut self, character: char) -> Token {
        let position = self.position();

        if character == ';' {
            self.advance_character();
            let id = self.source.consume_line().unwrap();
            self.column += id.len(); // EndOfFile token will still need this to be accurate

            return Token::new(
                TokenVariant::Comment,
                position, 
                id.trim()
            )
        }

        match character {
            ':' | '#' | '$' | '@' | '?' => self.lex_prefixed_token(position),
            '0'..='9' => self.lex_number(position),
            '(' => {
                self.advance_character();
                self.mode.push(LexerState::ModeSignature);
                Token::new(TokenVariant::OpenParen, position, "(")
            }
            _ => {
                let id = self.extract_word().to_string();

                if self.source.peek() == Some(':') {
                    self.source.consume();

                    Token::new(
                        TokenVariant::JumpHeader,
                        position, 
                        &id
                    )
                } else {
                    Token::new(
                        TokenVariant::Identifier,
                        position, 
                        &id
                    )
                }
            }
        }
    }

    // Cannot be used in Tuple Lexing
    fn lex_prefixed_token(&mut self, position: (usize, usize)) -> Token {
        let prefix = self.source.consume();
        self.advance_column();

        let variant = match prefix {
            Some(':') => TokenVariant::JumpLabel,
            Some('#') => TokenVariant::Directive,
            Some('$') => TokenVariant::DirectAddress,
            Some('@') => TokenVariant::IndirectAddress,
            Some('?') => TokenVariant::LazyAddress,
            _ => {
                TokenVariant::Error(format!("Prohibited Error: No prefix detected"))
            }
        };

        let id = self.extract_word();

        Token::new(
            variant,
            position, 
            id
        )
    }

    fn lex_number(&mut self, position: (usize, usize)) -> Token {
        let id = self.extract_word();

        if numeral_parser::is_numeric(id) {
            Token::new(TokenVariant::Number, position, id)
        } else {
            Token::new(TokenVariant::Identifier, position, id)
        }
    }

    fn extract_word(&mut self) -> &str {
        let id = self.source.consume_word().unwrap();
        self.column += id.len();

        id
    }

    fn process_whitespace(&mut self) {
        if self.source.consume() == Some('\n') {
            self.process_newline()
        } else {
            self.advance_column()
        }
    }

    fn process_newline(&mut self) {
        self.line += 1;
        self.reset_column();
        self.record_current_line();
    }

    fn reset_column(&mut self) {
        self.column = 1;
    }

    fn record_current_line(&mut self) -> Result<(), LexerError> {
        if let Some(exerpt) = self.source.peek_line() {
            self.exerpt = exerpt.to_string();
            Ok(())
        } else {
            Err(LexerError::CannotRecordEmptyLine) // This may not be necessary
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    fn advance_character(&mut self) -> Result<(), LexerError> {
        if self.source.consume().is_some() {
            self.advance_column();
            Ok(())
        } else {
            Err(LexerError::NoCharacterToConsume) // This may not be necessary
        }
    }

    fn advance_column(&mut self) {
        self.column += 1;
    }

    // pub fn lex(&mut self) -> Vec<Token> {
    //     let mut tokens = Vec::new();
    //     let buffer = String::new();

    //     self.mode.push(LexerState::Normal);

    //     while let Some(character) = self.source.peek() {
    //         let token = match self.mode.last() {
    //             Some(LexerState::Normal) => {
    //                 if character.is_whitespace() {
    //                     self.source.advance();
    //                     if character == '\n' { Token::Newline } else { continue; }
    //                 } else if character.is_alphabetic() || character == '_' {
    //                     let slice = self.source.consume_while(|c| c.is_alphanumeric() || c == '_');

    //                     if self.source.peek() == Some(':') { 
    //                         self.source.advance();
    //                         Token::LabelHeader(slice)
    //                     } else {
    //                         Token::Identifier(slice)
    //                     }
    //                 } else if character.is_ascii_digit() {
    //                     if character == '0' {
    //                         match self.source.peek_ahead(1) {
    //                             Some('x') | Some('X') => {
    //                                 if !self.source.peek_ahead(2).unwrap().is_ascii_hexdigit() {
    //                                     Token::Error {
    //                                         message: "Incorrect number format".to_string(),
    //                                         line_and_line: self.source.line_and_line(),
    //                                         id: self.source.consume_while(|c| c != '\n'),
    //                                     }
    //                                 } else {
    //                                     self.source.advance();
    //                                     self.source.advance();
    //                                     Token::HexNumber(self.source.consume_while(
    //                                         |c| c.is_ascii_hexdigit()
    //                                     ))
    //                                 }
    //                             },
    //                             Some('o') | Some('O') => {
    //                                 if !matches!(self.source.peek_ahead(2).unwrap(), '0'..='7') {
    //                                     Token::Error {
    //                                         message: "Incorrect number format".to_string(),
    //                                         line_and_line: self.source.line_and_line(),
    //                                         id: self.source.consume_while(|c| c != '\n'),
    //                                     }
    //                                 } else {
    //                                     self.source.advance();
    //                                     self.source.advance();
    //                                     Token::OctalNumber(self.source.consume_while(
    //                                         |c| matches!(c, '0'..='7')
    //                                     ))
    //                                 }
    //                             },
    //                             Some('b') | Some('B') => {
    //                                 if !(self.source.peek_ahead(2) == Some('1') || 
    //                                         self.source.peek_ahead(2) == Some('0'))
    //                                 {
    //                                     Token::Error {
    //                                         message: "Incorrect number format".to_string(),
    //                                         line_and_line: self.source.line_and_line(),
    //                                         id: self.source.consume_while(|c| c != '\n'),
    //                                     }
    //                                 } else {
    //                                     self.source.advance();
    //                                     self.source.advance();
    //                                     Token::BinaryNumber(self.source.consume_while(|c| 
    //                                         c == '1' || c == '0'
    //                                     ))
    //                                 }
    //                             },
    //                             _ => Token::DecimalNumber(self.source.consume_while(
    //                                 |c| c.is_ascii_digit()
    //                             )),
    //                         }
    //                     } else {
    //                         Token::DecimalNumber(self.source.consume_while(|c| c.is_numeric()))
    //                     }
    //                 } else {
    //                     match character {
    //                         ';' => {
    //                             self.source.advance();
    //                             let slice = self.source.consume_while(|c| c != '\n');
    //                             Token::Comment(slice)
    //                         },
    //                         ':' => {
    //                             self.source.advance();
    //                             let slice = self.source.consume_while(|c| 
    //                                 c.is_alphanumeric() || c == '_'
    //                             );
    //                             Token::JumpLabel(slice)
    //                         },
    //                         '#' => {
    //                             self.source.advance();
    //                             let slice = self.source.consume_while(|c| c.is_alphanumeric());
    //                             Token::Directive(slice)
    //                         },
    //                         '$' => {
    //                             self.source.advance();
    //                             let slice = self.source.consume_while(|c| c.is_alphanumeric());
    //                             Token::DirectAddress(slice)
    //                         },
    //                         '@' => {
    //                             self.source.advance();
    //                             let slice = self.source.consume_while(|c| c.is_alphanumeric());
    //                             Token::IndirectAddress(slice)
    //                         },
    //                         ',' => {
    //                             self.source.advance();
    //                             Token::Comma
    //                         },
    //                         '[' => {
    //                             self.mode.push(LexerState::ArrayLiteral);
    //                             self.source.advance();
    //                             Token::OpenBracket
    //                         },
    //                         '{' => {
    //                             self.source.advance();
    //                             Token::OpenBrace
    //                         },
    //                         '}' => {
    //                             self.source.advance();
    //                             Token::CloseBrace
    //                         },
    //                         '(' => {
    //                             self.mode.push(LexerState::TupleLiteral);
    //                             self.source.advance();
    //                             Token::OpenParen
    //                         },
    //                         ')' => {
    //                             self.mode.pop();
    //                             self.source.advance();
    //                             Token::CloseParen
    //                         },
    //                         '"' => {
    //                             self.mode.push(LexerState::StringLiteral);
    //                             self.source.advance();
    //                             Token::Quote
    //                         },
    //                         _ => {
    //                             Token::Error { 
    //                                 message: format!("Unknown Token: {}", character), 
    //                                 line_and_line: self.source.line_and_line(), 
    //                                 id: self.source.consume_while(|c| c != '\n') 
    //                             }
    //                         },
    //                     }
    //                 }
    //             },
    //             Some(LexerState::StringLiteral) => {
    //                 if character == '"' {
    //                     self.mode.pop();
    //                     self.source.advance();
    //                     Token::Quote
    //                 } else {
    //                     let slice = self.source.consume_while(|c| c != '"');
    //                     Token::String(slice)
    //                 }
    //             },
    //             Some(LexerState::ArrayLiteral) => {
    //                 if character == '\n' {
    //                     self.source.advance();
    //                 } 

    //                 if character.is_whitespace() {
    //                     self.source.advance();
    //                     continue;
    //                 }
    //                 match character {
    //                     ',' => {
    //                         self.source.advance();
    //                         Token::Comma
    //                     },
    //                     ']' => {
    //                         self.source.advance();
    //                         self.mode.pop();
    //                         Token::CloseBracket
    //                     },
    //                     _ => Token::Element(
    //                     self.source.consume_while(|c| !matches!(c, ',' | ']' | '\n'))
    //                     )
    //                 }
    //             },
    //             Some(LexerState::TupleLiteral) => {
    //                 if character.is_whitespace() {
    //                     self.source.advance();
    //                     continue;
    //                 }
    //                 match character {
    //                     ',' => {
    //                         self.source.advance();
    //                         Token::Comma
    //                     },
    //                     ')' => {
    //                         self.source.advance();
    //                         self.mode.pop();
    //                         Token::CloseParen
    //                     },
    //                     _ => Token::ModeKey(
    //                     self.source.consume_while(|c| c != ',' && c != ')')
    //                     )
    //                 }
    //             },
    //             None => Token::EndOfFile,
    //         };

    //         tokens.push(token);
    //     }

    //     tokens.push(Token::EndOfFile);
    //     tokens
    // }

    // fn slice(&self, start: usize, end: usize) -> &'a str {
    //     self.source
    //     .get(start..end)
    //     .unwrap_or_else(|| {
    //         panic!(
    //         "Lexer Error: tried to slice invalid UTF-8 boundaries ({}..{})", start, end
    //         )
    //     })
    // }
}
