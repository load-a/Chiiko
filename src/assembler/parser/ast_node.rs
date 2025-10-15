use crate::operand::Operand;
use crate::mode::mode_group::ModeGroup;

// Temp
use crate::assembler::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program {
        data: Option<Vec<Token>>,
        logic: Option<Vec<Token>>,
        subroutines: Option<Vec<Token>>,
    },
}

