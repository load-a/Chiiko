use crate::chiiko::components::operand::Operand;
use crate::chiiko::opcode::Opcode;

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub mode: u8,
    pub left_operand: Operand,
    pub right_operand: Operand,
}

impl Default for Instruction {
    // Gives a WAIT (no-op) instruction
    fn default() -> Self {
        Self {
            opcode: Opcode::decode(0b01110001),
            mode: 0,
            left_operand: Operand::None,
            right_operand: Operand::None,
        }
    }
}

impl Instruction {
    pub fn new(opcode: Opcode, mode: u8, left: Operand, right: Operand) -> Self {
        Self {
            opcode: opcode,
            mode: mode,
            left_operand: left, 
            right_operand: right,
        }
    }

    pub fn bytes(&self) -> [u8; 6] {
        let left_side: Vec<u8> = match self.left_operand {
            Operand::None | Operand::Error => [0xFF, 0xFF].to_vec(),
            Operand::Value(value) | Operand::Register(value) | Operand::IndirectRegister(value) |
            Operand::ZeroPageAddress(value) | 
            Operand::IndirectZeroPageAddress(value) => [0, value].to_vec(),
            Operand::MemoryAddress(value) | Operand::IndirectMemoryAddress(value) | 
            Operand::JumpAddress(value) => value.to_be_bytes().to_vec(),
        };

        let right_side: Vec<u8> = match self.right_operand {
            Operand::None => [0, 0].to_vec(),
            Operand::Value(value) | Operand::Register(value) | Operand::IndirectRegister(value) |
            Operand::ZeroPageAddress(value) | 
            Operand::IndirectZeroPageAddress(value) => [0, value].to_vec(),
            Operand::MemoryAddress(value) | Operand::IndirectMemoryAddress(value) | 
            Operand::JumpAddress(value) => value.to_be_bytes().to_vec(),
            Operand::Error => [0xFF, 0xFF].to_vec()
        };

        [self.opcode.byte, self.mode, left_side[0], left_side[1], right_side[0], right_side[1]]
    }
}