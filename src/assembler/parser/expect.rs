use crate::assembler::parser::{Parser, ParserError};
use crate::assembler::lexer::token::{Token, TokenVariant};

impl<I> Parser<I> 
where 
    I: Iterator<Item = Token>,
{
    pub(crate) fn expect_directive(&mut self, expected: &str) -> Result<(), ParserError> {
        let token = self.expect(TokenVariant::Division)?;
        if token.id == expected.to_string() {
            Ok(())
        } else {
            Err(Self::unexpected_id(expected, token))
        }
    }

    pub(crate) fn expect(&mut self, expected: TokenVariant) -> Result<Token, ParserError> {
        match self.consume() {
            Some(token) if token.variant == expected => Ok(token),
            Some(token) => Err(Self::unexpected_token(expected, token)),
            None => Err(ParserError::UnexpectedEOF)
        }
    }

    pub(crate) fn unexpected_token(expected: TokenVariant, found: Token) -> ParserError {
        match found.variant {
            TokenVariant::Error(string) => ParserError::ErrorToken(string),
            _ => {
                ParserError::UnexpectedToken {
                    expected: format!("{:?}", expected),
                    found: found.log(),
                }
            }
        }
    }

    pub(crate) fn unexpected_id(expected: &str, found: Token) -> ParserError {
        ParserError::UnexpectedID {
            expected: format!("{:?}", expected),
            found: found.log(),
        }
    }

    pub(crate) fn consume_terminator(&mut self) -> Result<(), ParserError> {
        if self.expect(TokenVariant::Terminator).is_ok() || self.is_eof() {
            Ok(())
        } else {
            Err(Self::unexpected_token(
                TokenVariant::Terminator, self.consume().expect("This should not happen.")
            ))
        }
    }

    pub(crate) fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
