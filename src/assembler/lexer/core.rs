use crate::numeral_parser::numeral_parser;
use crate::mode::Mode;
use crate::assembler::source::Source;
use crate::assembler::lexer::{token::Token, token::TokenVariant, token::TokenVariant::*, LexerError};

#[derive(PartialEq)]
enum LexerState {
    Normal,
    StringLiteral,
    ByteArray,
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
                Some(LexerState::ByteArray) => {
                    if character.is_whitespace() {
                        self.process_whitespace();
                        continue; 
                    } else {
                        tokens.push(self.lex_array(character))
                    }
                }
                _ => ()
            }
        }

        tokens.push(Token::end_of_file(self.position()));

        Ok(tokens)
    }

    fn lex_array(&mut self, character: char) -> Token {
        let position = self.position();

        match character {
            ',' => {
                self.advance_character();
                Token::comma(position)
            }
            ']' => {
                self.mode.pop();
                self.advance_character();
                Token::new(TokenVariant::CloseBracket, position, "]")
            }
            '=' => {
                self.advance_character();
                Token::assignment(position)
            }
            '0'..='9' => return self.lex_number(position),
            'A'..='z' | '_' => {
                let word = self.extract_word().to_string();
                Token::new(TokenVariant::Identifier, position, &word)
            }
            _ => {
                self.advance_character();
                self.token_error(position, &(character.to_string()), "Invalid character in Array")
            }
        } 
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
            '[' => {
                self.advance_character();
                self.mode.push(LexerState::ByteArray);
                Token::new(TokenVariant::OpenBracket, position, "[")
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
            self.exerpt = format!("{}. {}", self.line, exerpt);
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
}
