use std::iter::Peekable;
use crate::assembler::lexer::token::{Token, TokenVariant};
use crate::assembler::parser::{ast_node::AstNode, ast_node::ProgramNode, ParserError};

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
            Some(self.parse_data_division()?)
        } else { 
            None 
        };

        let logic = self.parse_logic_division()?;

        let subroutines = if self.peek_is_directive("SUBROUTINES") {
            Some(self.parse_subroutine_division()?)
        } else { 
            None 
        };

        Ok(AstNode::Program(ProgramNode {data, logic, subroutines}))
    }
}
