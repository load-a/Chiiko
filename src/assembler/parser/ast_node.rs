use crate::operand::Operand;
use crate::mode::mode_group::ModeGroup;

// Temp
use crate::assembler::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program(ProgramNode),
    // Subroutine(SubroutineNode),
    // Block(BlockNode),
    // Instruction(InstructionNode),
    // Operation(OperationNode),
    // ModeSignature(ModeSignatureNode),
    // Operand(OperandNode),
    // MemoryAddress(MemoryAddressNode),
    // Index(IndexNode),
    // SimpleAddress(SimpleAddressNode),
    // Array(ArrayNode),
    // Element(ElementNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramNode {
    pub data: Option<DataDivisionNode>,
    pub logic: LogicDivisionNode,
    pub subroutines: Option<SubroutineDivisionNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataDivisionNode {
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogicDivisionNode {
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineDivisionNode {
    pub tokens: Vec<Token>,
}
