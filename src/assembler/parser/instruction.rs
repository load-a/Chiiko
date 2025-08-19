use crate::assembler::parser::operand::Operand;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Operand>,
}
