use crate::assembler::parser::assembler_operand::AssemblerOperand;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<AssemblerOperand>,
}
