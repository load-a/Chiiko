use std::iter::Peekable;
use crate::assembler::lexer::token::{Token, TokenVariant};
use crate::assembler::parser::{ast_node::AstNode, ParserError};

#[derive(Debug)]
pub struct Parser<I>
where 
    I: Iterator<Item = Token>,
{
    pub(crate) tokens: Peekable<I>
}

impl<I> Parser<I> 
where 
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        let data = if self.peek_is_directive("DATA") {
            Some(self.parse_data()?)
        } else { 
            None 
        };

        let logic = Some(self.parse_logic()?);

        let subroutines = if self.peek_is_directive("SUBROUTINES") {
            Some(self.parse_subroutines()?)
        } else { 
            None 
        };

        Ok(AstNode::Program{data, logic, subroutines})
    }
}
