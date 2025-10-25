use crate::mode::Mode;
use crate::assembler::parser::{Parser, ParserError};
use crate::assembler::parser::ast_node::{
    AstNode, ProgramNode, DataDivisionNode, LogicDivisionNode, SubroutineDivisionNode, 
    SubroutineNode, InstructionNode, 
};
use crate::assembler::lexer::token::{Token, TokenVariant};

impl<I> Parser<I> 
where 
    I: Iterator<Item = Token>,
{
    pub(crate) fn parse_data_division(&mut self) -> Result<DataDivisionNode, ParserError> {
        self.expect_directive("DATA")?;
        self.consume_terminator()?;

        Ok(DataDivisionNode { directives: self.parse_instructions()? })
    }

    pub(crate) fn parse_logic_division(&mut self) -> Result<LogicDivisionNode, ParserError> {
        if self.peek_is_directive("LOGIC") {
            self.expect_directive("LOGIC")?;
        }
        self.consume_terminator()?;

        Ok(LogicDivisionNode { tokens: self.collect_tokens()? })
    }

    pub(crate) fn parse_subroutine_division(&mut self) -> Result<SubroutineDivisionNode, ParserError> {
        self.expect_directive("SUBROUTINES")?;
        self.consume_terminator()?;

        Ok(SubroutineDivisionNode{ subroutines: self.collect_tokens()? })
    }

    fn parse_instructions(&mut self) -> Result<Vec<InstructionNode>, ParserError> {
        let mut instructions = Vec::new();

        while self.peek_is_within_subroutine() {
            instructions.push(self.parse_instruction()?);
        }

        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<InstructionNode, ParserError> {
        let operation = self.parse_operation()?;
        let mode = self.parse_mode()?;
        let mut operands = Vec::new();

        while self.peek_is_within_instruction() {
            if let Some(token) = self.consume() {
                operands.push(token);
            } else {
                return Err(ParserError::UnexpectedEOF);
            }
        }

        self.consume_terminator()?;

        Ok(InstructionNode { operation: operation, mode: mode, operands: operands })
    }

    fn parse_operation(&mut self) -> Result<String, ParserError> {
        let token = self.expect(TokenVariant::Identifier)?;
        Ok(token.id)
    }

    fn parse_mode(&mut self) -> Result<Option<(Mode, Mode)>, ParserError> {
        if !self.peek_is(TokenVariant::OpenParen) {
            return Ok(None);
        }

        self.expect(TokenVariant::OpenParen)?;
        let token_1 = self.expect(TokenVariant::ModeKey)?;
        self.expect(TokenVariant::Comma)?;
        let token_2 = self.expect(TokenVariant::ModeKey)?;
        self.expect(TokenVariant::CloseParen)?;

        let mode_1 = Mode::from_key(&token_1.id)?;
        let mode_2 = Mode::from_key(&token_2.id)?;

        Ok(Some((mode_1, mode_2)))
    }

    fn collect_tokens(&mut self) -> Result<Vec<Token>, ParserError> {
        let mut nodes = Vec::new();

        while self.peek_is_within_division() {
            if let Some(token) = self.consume() {
                nodes.push(token);
            } else {
                break;
            }
        }

        Ok(nodes)
    }
}
