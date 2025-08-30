use crate::assembler::parser::{Parser, ast_node::ASTNode, ast_node::MacroNode,
    assembler_operand::AssemblerOperand
};
use crate::operation::Operation;
use crate::assembler::encoder::syntax_checker::SyntaxChecker;

enum Instruction {
    Normal { operation: u8, mode: u8, operands: Vec<u8> },
    Macro(&'static str)
}

pub struct InstructionGenerator {
    source: Vec<ASTNode>,
    instructions: Vec<Instruction>,
}

impl InstructionGenerator {
    pub fn new(source: Vec<ASTNode>) -> Self {

        SyntaxChecker::check(source.clone());

        Self {
            source: source, 
            instructions: Vec::new()
        }
    }

    pub fn generate(&mut self) {
        for node in &self.source {
            match node {
                ASTNode::Instruction { mnemonic, mode, operands } => {
                    let operation = Operation::from_mnemonic(&mnemonic);
                    let mode = SyntaxChecker::infer_mode(operands);
                    let operands = operands;

                    println!("{:?}\n{:?}\n{:?}", operation, mode, operands);
                },
                _ => ()
            }
        }
    }
}
