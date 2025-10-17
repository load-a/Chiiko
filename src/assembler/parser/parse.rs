use crate::assembler::parser::{Parser, ParserError};
use crate::assembler::parser::ast_node::{
    AstNode, ProgramNode, DataDivisionNode, LogicDivisionNode, SubroutineDivisionNode
};
use crate::assembler::lexer::token::{Token, TokenVariant};

impl<I> Parser<I> 
where 
    I: Iterator<Item = Token>,
{
    pub(crate) fn parse_data_division(&mut self) -> Result<DataDivisionNode, ParserError> {
        self.expect_directive("DATA")?;

        Ok(DataDivisionNode { tokens: self.parse_division()? })
    }

    pub(crate) fn parse_logic_division(&mut self) -> Result<LogicDivisionNode, ParserError> {
        if self.peek_is_directive("LOGIC") {
            self.expect_directive("LOGIC")?;
        }

        Ok(LogicDivisionNode { tokens: self.parse_division()? })
    }

    pub(crate) fn parse_subroutine_division(&mut self) -> Result<SubroutineDivisionNode, ParserError> {
        self.expect_directive("SUBROUTINES")?;

        Ok(SubroutineDivisionNode{ tokens: self.parse_subroutines()? })
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

    pub(crate) fn parse_subroutines(&mut self) -> Result<Vec<Token>, ParserError> {
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
