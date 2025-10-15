use crate::assembler::parser::{Parser, ParserError};
use crate::assembler::lexer::token::{Token, TokenVariant};

impl<I> Parser<I> 
where 
    I: Iterator<Item = Token>,
{
    pub(crate) fn parse_data(&mut self) -> Result<Vec<Token>, ParserError> {
        self.expect_directive("DATA")?;

        self.parse_division()
    }

    pub(crate) fn parse_logic(&mut self) -> Result<Vec<Token>, ParserError> {
        if self.peek_is_directive("LOGIC") {
            self.expect_directive("LOGIC")?;
        }

        self.parse_division()
    }

    pub(crate) fn parse_subroutines(&mut self) -> Result<Vec<Token>, ParserError> {
        self.expect_directive("SUBROUTINES")?;

        self.parse_division()
    }

    pub(crate) fn parse_division(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut nodes = Vec::new();

        while !(self.is_eof() || self.peek_is(TokenVariant::Directive)) {
            if let Some(token) = self.consume() {
                nodes.push(token);
            } else {
                break;
            }
        }

        Ok(nodes)
    }
}
