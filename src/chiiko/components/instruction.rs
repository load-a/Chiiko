use crate::operation::Operation;
use crate::mode::Mode;
use crate::operand::Operand;

#[derive(PartialEq, Debug, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub mode: u8,
    pub left_operand: Operand,
    pub right_operand: Operand,
}

impl Default for Instruction {
    // Gives a WAIT (no-op) instruction
    fn default() -> Self {
        Self {
            operation: Operation::from_byte(0b01110001),
            mode: 0,
            left_operand: Operand::NoOperand,
            right_operand: Operand::NoOperand,
        }
    }
}

impl Instruction {
    pub fn new(operation: Operation, mode: u8, left: Operand, right: Operand) -> Self {
        Self {
            operation: operation,
            mode: mode,
            left_operand: left, 
            right_operand: right,
        }
    }

    pub fn bytes(&self) -> [u8; 6] {
        let left_side: Vec<u8> = self.left_operand.as_u16()::to_be_bytes();
        let right_side: Vec<u8> = self.right_operand.as_u16()::to_be_bytes();

        [self.operation.opcode, self.mode, left_side[0], left_side[1], right_side[0], right_side[1]]
    }
}
