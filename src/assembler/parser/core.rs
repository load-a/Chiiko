use crate::assembler::lexer::{token::Token};
use crate::assembler::parser::ast_node::AstNode;

#[derive(Default, Debug)]
pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            position: 0
        }
    }

    pub(crate) fn advance(&mut self) {
        self.position = self.position.wrapping_add(1);
    }

    // fn consume(variant: TokenVariant) -> Result<(), ParserError> {
        
    // }
}
