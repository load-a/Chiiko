use crate::assembler::parser::{Parser, ParserError};
use crate::assembler::lexer::token::{Token, TokenVariant};

impl<I> Parser<I> 
where 
    I: Iterator<Item = Token>,
{
    pub(crate) fn peek_return(&mut self) -> bool {
        if let Some(token) = self.peek() {
            token.variant == TokenVariant::Identifier &&
            token.id == "RTRN".to_string()
        } else {
            false
        }
    }

    pub(crate) fn peek_is_directive(&mut self, id: &str) -> bool {
        if let Some(token) = self.peek() {
            token.variant == TokenVariant::Division &&
            token.id == id.to_string()
        } else {
            false
        }
    }

    pub(crate) fn peek_is_within_division(&mut self) -> bool {
        !(self.is_eof() || self.peek_is(TokenVariant::Division))
    }

    pub(crate) fn peek_is_within_subroutine(&mut self) -> bool {
        self.peek_is_within_division() &&
        !self.peek_is(TokenVariant::SubroutineHeader)
    }

    pub(crate) fn peek_is_within_instruction(&mut self) -> bool {
        self.peek_is_within_subroutine() &&
        !self.peek_is(TokenVariant::Terminator)
    }

    pub(crate) fn peek_is(&mut self, variant: TokenVariant) -> bool {
        if let Some(token) = self.peek() {
            token.variant == variant
        } else {
            false
        }
    }

    pub(crate) fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub(crate) fn is_eof(&mut self) -> bool {
        self.peek().is_none()
    }
}
