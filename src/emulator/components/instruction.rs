use crate::operation::Operation;
use crate::mode::Mode;
use crate::operand::Operand;

#[derive(PartialEq, Debug, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub mode: (Mode, Mode),
    pub left_operand: Operand,
    pub right_operand: Operand,
}

impl Default for Instruction {
    // Gives a WAIT (no-op) instruction
    fn default() -> Self {
        Self {
            operation: Operation::from_byte(0b01110001).unwrap(),
            mode: Mode::from_byte(0).unwrap(),
            left_operand: Operand::NoOperand,
            right_operand: Operand::NoOperand,
        }
    }
}

impl Instruction {
    pub fn new(operation: Operation, mode: (Mode, Mode), left: Operand, right: Operand) -> Self {
        Self {
            operation: operation,
            mode: mode,
            left_operand: left, 
            right_operand: right,
        }
    }

    pub fn bytes(&self) -> [u8; 6] {
        let left_side: [u8; 2] = self.left_operand.value().unwrap().to_be_bytes();
        let right_side: [u8; 2] = self.right_operand.value().unwrap().to_be_bytes();
        let mode = self.mode.0.nibble << 4 | self.mode.1.nibble;

        [self.operation.opcode, mode, left_side[0], left_side[1], right_side[0], right_side[1]]
    }
}
