use crate::register::Register;
use crate::assembler::lexer::{
    lexer_state::LexerState, token::Token, token::TokenVariant, LexerError,
};
use crate::assembler::source::Source;
use crate::mode::Mode;
use crate::numeral_parser::numeral_parser;

pub struct Lexer {
    pub(crate) source: Source,
    pub(crate) mode: Vec<LexerState>,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) exerpt: String,
    pub(crate) string_position: (usize, usize),
}

impl Lexer {
    pub fn new(source: Source) -> Self {
        Self {
            source: source,
            mode: Vec::with_capacity(4),
            line: 1,
            column: 1,
            exerpt: String::new(),
            string_position: (0, 0),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        if self.source.peek().is_none() {
            return Ok([Token::end_of_file(self.position())].to_vec());
        }

        let mut tokens = Vec::new();
        let mut buffer = String::new();

        self.enter_mode(LexerState::Normal);
        self.record_current_line()?;

        while let Some(character) = self.source.peek() {
            match self.current_mode() {
                LexerState::Normal => {
                    if character == '\n' {
                        tokens.push(Token::terminator(self.position()));
                        self.process_whitespace();
                        continue;
                    } else if character.is_whitespace() || character == ',' {
                        self.process_whitespace()?;
                        continue;
                    } else {
                        tokens.push(self.lex_normal(character)?)
                    }
                }
                LexerState::ModeSignature => {
                    if character.is_whitespace() {
                        self.process_whitespace()?;
                        continue;
                    } else {
                        tokens.push(self.lex_signature(character)?)
                    }
                }
                LexerState::ByteArray => {
                    if character.is_whitespace() {
                        self.process_whitespace()?;
                        continue;
                    } else {
                        tokens.push(self.lex_array(character)?)
                    }
                }
                LexerState::StringLiteral => {
                    self.advance_cursor()?;

                    match character {
                        '\\' => {
                            buffer.push('\\');

                            if self.source.peek() == Some('"') {
                                self.advance_cursor()?;
                                buffer.push('"');
                            }

                            continue;
                        }
                        '"' => {
                            tokens.push(self.tokenize_string(&buffer));
                            buffer.clear();

                            self.exit_mode();
                            tokens.push(Token::new(TokenVariant::Quote, self.position(), "\""));

                            continue;
                        }
                        '\n' => {
                            self.process_newline();
                            buffer.push(character);
                            continue;
                        }
                        _ => {
                            buffer.push(character);
                        }
                    }
                }
                _ => (),
            }
        }

        if !buffer.is_empty() {
            return Err(LexerError::UnfinishedStringLiteral);
        } else if self.current_mode() == LexerState::ByteArray {
            return Err(LexerError::UnclosedByteArray);
        } else if self.current_mode() == LexerState::ModeSignature {
            return Err(LexerError::UnclosedModeSignature);
        }

        tokens.push(Token::end_of_file(self.position()));

        Ok(tokens)
    }

    fn tokenize_string(&self, string: &String) -> Token {
        Token::string(self.string_position, string)
    }

    fn lex_array(&mut self, character: char) -> Result<Token, LexerError> {
        let position = self.position();

        match character {
            ',' => {
                self.advance_cursor()?;
                Ok(Token::comma(position))
            }
            ']' => {
                self.exit_mode();
                self.advance_cursor()?;
                Ok(Token::new(TokenVariant::CloseBracket, position, "]"))
            }
            '=' => {
                self.advance_cursor()?;
                Ok(Token::assignment(position))
            }
            '?' => {
                self.advance_cursor()?;
                Ok(Token::new(TokenVariant::LazyAddress, position, "?"))
            }
            '0'..='9' => return self.lex_number(position),
            'A'..='z' | '_' => {
                let word = self.extract_word()?.to_string();
                Ok(Token::new(TokenVariant::Identifier, position, &word))
            }
            _ => {
                self.advance_cursor()?;
                Ok(self.error_token(
                    position,
                    &(character.to_string()),
                    "Invalid character in Array",
                ))
            }
        }
    }

    fn lex_signature(&mut self, character: char) -> Result<Token, LexerError> {
        let position = self.position();

        if character == ',' {
            self.advance_cursor()?;
            return Ok(Token::comma(position));
        } else if character == ')' {
            self.exit_mode();
            self.advance_cursor()?;
            return Ok(Token::new(TokenVariant::CloseParen, position, ")"));
        }

        let word = self.extract_word()?.to_string();

        if Mode::is_mode_key(word.to_uppercase().as_str()) {
            Ok(Token::new(
                TokenVariant::ModeKey,
                position,
                &format!("{}", word),
            ))
        } else {
            Ok(self.error_token(position, word.as_str(), "Invalid Mode Signature"))
        }
    }

    fn error_token(&self, position: (usize, usize), id: &str, message: &str) -> Token {
        Token::error(
            position,
            format!("{}:\nExerpt: {}", message, self.exerpt),
            id,
        )
    }

    fn lex_normal(&mut self, character: char) -> Result<Token, LexerError> {
        let position = self.position();

        if character == ';' {
            self.advance_cursor()?;
            let message = self.source.consume_line().ok_or(LexerError::NoLine)?;
            self.column += message.len(); // EndOfFile token will still need this to be accurate

            return Ok(Token::comment(position, message.trim()));
        } else if character.is_ascii_alphabetic() || character == '_' {
            return self.resolve_identifier_token(position)
        }

        let token = match character {
            ':' | '#' | '$' | '@' | '&' => self.lex_prefixed_token(position)?,
            '0'..='9' => self.lex_number(position)?,
            '.' => {
                self.advance_cursor()?;
                Token::new(TokenVariant::IndexMarker, position, ".")
            }
            '?' => {
                self.advance_cursor()?;
                Token::new(TokenVariant::LazyAddress, position, "?")
            }
            '(' => {
                self.advance_cursor()?;
                self.enter_mode(LexerState::ModeSignature);
                Token::new(TokenVariant::OpenParen, position, "(")
            }
            '[' => {
                self.advance_cursor()?;
                self.enter_mode(LexerState::ByteArray);
                Token::new(TokenVariant::OpenBracket, position, "[")
            }
            '"' => {
                self.advance_cursor()?;
                self.string_position = position;
                self.enter_mode(LexerState::StringLiteral);
                Token::new(TokenVariant::Quote, position, "\"")
            }
            '{' => {
                self.advance_cursor()?;
                Token::new(TokenVariant::OpenBrace, position, "{")
            }
            '}' => {
                self.advance_cursor()?;
                Token::new(TokenVariant::CloseBrace, position, "}")
            }
            _ => {
                self.advance_cursor()?;
                self.error_token(position, &(character.to_string()), "Invalid character")
            }
        };

        Ok(token)
    }

    fn resolve_identifier_token(&mut self, position: (usize, usize)) -> Result<Token, LexerError> {
        let id = self.extract_word()?.to_string();

        if self.source.peek() == Some(':') {
            self.advance_cursor()?;

            Ok(Token::new(TokenVariant::SubroutineHeader, position, &id))
        } else if Register::is_register_name(&id) {
            Ok(Token::new(TokenVariant::Register, position, &id))
        } else {
            Ok(Token::new(TokenVariant::Identifier, position, &id))
        }
    }

    // Cannot be used in Tuple Lexing
    fn lex_prefixed_token(&mut self, position: (usize, usize)) -> Result<Token, LexerError> {
        let prefix = self.source.consume();
        self.increment_column_counter()?;

        let variant = match prefix {
            Some(':') => TokenVariant::JumpLabel,
            Some('#') => TokenVariant::Division,
            Some('$') => TokenVariant::DirectAddress,
            Some('@') => TokenVariant::IndirectAddress,
            Some('&') => TokenVariant::ChipLabel,
            _ => TokenVariant::Error(format!("Prohibited Error: No valid prefix detected")),
        };

        let id = self.extract_word()?;

        Ok(Token::new(variant, position, id))
    }

    fn lex_number(&mut self, position: (usize, usize)) -> Result<Token, LexerError> {
        let id = self.extract_word()?;

        if numeral_parser::is_numeric(id) {
            Ok(Token::new(TokenVariant::Number, position, id))
        } else {
            Ok(Token::new(TokenVariant::Identifier, position, id))
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}
