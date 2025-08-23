use crate::chiiko::components::cpu_operand::CpuOperand;
use crate::operation::Operation;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Instruction {
    pub operation: Operation,
    pub mode: u8,
    pub left_operand: CpuOperand,
    pub right_operand: CpuOperand,
}

impl Default for Instruction {
    // Gives a WAIT (no-op) instruction
    fn default() -> Self {
        Self {
            operation: Operation::from_byte(0b01110001),
            mode: 0,
            left_operand: CpuOperand::None,
            right_operand: CpuOperand::None,
        }
    }
}

impl Instruction {
    pub fn new(operation: Operation, mode: u8, left: CpuOperand, right: CpuOperand) -> Self {
        Self {
            operation: operation,
            mode: mode,
            left_operand: left, 
            right_operand: right,
        }
    }

    pub fn bytes(&self) -> [u8; 6] {
        let left_side: Vec<u8> = match self.left_operand {
            CpuOperand::None | CpuOperand::Error => [0xFF, 0xFF].to_vec(),
            CpuOperand::Value(value) | CpuOperand::Register(value) | CpuOperand::IndirectRegister(value) |
            CpuOperand::ZeroPageAddress(value) | 
            CpuOperand::IndirectZeroPageAddress(value) => [0, value].to_vec(),
            CpuOperand::MemoryAddress(value) | CpuOperand::IndirectMemoryAddress(value) | 
            CpuOperand::JumpAddress(value) => value.to_be_bytes().to_vec(),
        };

        let right_side: Vec<u8> = match self.right_operand {
            CpuOperand::None => [0, 0].to_vec(),
            CpuOperand::Value(value) | CpuOperand::Register(value) | CpuOperand::IndirectRegister(value) |
            CpuOperand::ZeroPageAddress(value) | 
            CpuOperand::IndirectZeroPageAddress(value) => [0, value].to_vec(),
            CpuOperand::MemoryAddress(value) | CpuOperand::IndirectMemoryAddress(value) | 
            CpuOperand::JumpAddress(value) => value.to_be_bytes().to_vec(),
            CpuOperand::Error => [0xFF, 0xFF].to_vec()
        };

        [self.operation.opcode, self.mode, left_side[0], left_side[1], right_side[0], right_side[1]]
    }
}
