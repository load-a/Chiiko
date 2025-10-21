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

        Ok(DataDivisionNode { directives: self.parse_instructions()? })
    }

    pub(crate) fn parse_logic_division(&mut self) -> Result<LogicDivisionNode, ParserError> {
        if self.peek_is_directive("LOGIC") {
            self.expect_directive("LOGIC")?;
        }

        Ok(LogicDivisionNode { tokens: self.parse_division()? })
    }

    pub(crate) fn parse_subroutine_division(&mut self) -> Result<SubroutineDivisionNode, ParserError> {
        self.expect_directive("SUBROUTINES")?;
        self.consume_terminator()?;

        Ok(SubroutineDivisionNode{ subroutines: self.parse_division()? })
    }

    // pub(crate) fn parse_subroutines(&mut self) -> Result<Vec<SubroutineNode>, ParserError> {
    //     let mut subroutines = Vec::new();

    //     while self.peek_is_within_division() {
    //         subroutines.push(self.parse_subroutine()?);
    //     }

    //     Ok(subroutines)
    // }

    // fn parse_subroutine(&mut self) -> Result<SubroutineNode, ParserError> {
    //     let header = self.expect(TokenVariant::SubroutineHeader)?.id;
    //     self.consume_terminator()?;

    //     Ok(SubroutineNode { header: header, instructions: self.parse_instructions()? })
    // }

    fn parse_instructions(&mut self) -> Result<Vec<InstructionNode>, ParserError> {
        let mut instructions = Vec::new();

        while self.peek_is_within_subroutine() {
            let instruction = self.parse_instruction()?;

            if instruction.is_empty() {
                println!(">> {:?}", instruction.is_empty());

                continue;
            } else {
                instructions.push(instruction);
            }
        }

        Ok(instructions)
    }

    fn parse_instruction(&mut self) -> Result<InstructionNode, ParserError> {
        let mut nodes = Vec::new();

        while self.peek_is_within_instruction() {

            if let Some(token) = self.consume() {
                nodes.push(token);
            } else {
                return Err(ParserError::UnexpectedEOF);
            }
        }

        self.consume_terminator()?;

        Ok(InstructionNode { tokens: nodes })
    }

    fn parse_division(&mut self) -> Result<Vec<Token>, ParserError> {
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
