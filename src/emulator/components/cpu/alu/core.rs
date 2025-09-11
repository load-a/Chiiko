use std::io;
use rand::Rng;
use crate::emulator::components::{ chip::Chip, instruction::Instruction };
use crate::emulator::components::cpu::{ Cpu, CpuError, alu::AluError };
use crate::operation::group::{
    Group, ArithmeticVariant, LogicVariant, BranchVariant, SubroutineVariant, 
    StackVariant, MemoryVariant, InputOutputVariant, SystemVariant,
};
use crate::operand::Operand;
use crate::operation::Operation;

const ZERO_STATUS: u8 = 0b00000001;
const NEGATIVE_STATUS: u8 = 0b00000010;
const POSITIVE_STATUS: u8 = 0b00000000;
const DEFAULT_CHARACTER_LIMIT: u8 = 0xFF;
const NULL_CHARACTER: u8 = 0;

pub trait Alu {
    fn execute(&mut self) -> Result<(), CpuError>;
    fn evaluate_arithmetic(&mut self, variant: ArithmeticVariant) -> Result<(), CpuError>;
}

impl Alu for Cpu {
    fn execute(&mut self) -> Result<(), CpuError> {
        // Instruction is fetched and stored as a field in CPU before this gets called
        let group = self.instruction.operation.group;

        match group {
            Group::Arithmetic(variant) => self.evaluate_arithmetic(variant),
            _ => return Err(AluError::CannotFetchInstruction)?
        }
    }

    fn evaluate_arithmetic(&mut self, variant: ArithmeticVariant) -> Result<(), CpuError> {
        let left = self.find(&self.instruction.left_operand)?;
        let right = self.find(&self.instruction.right_operand)?;

        let (result, overflow) = match variant {
            ArithmeticVariant::Add | ArithmeticVariant::Increment => {
                left.overflowing_add(right)
            }
            _ => todo!()
        };
        self.update_flags(result, overflow);

        let destination = match variant {
            ArithmeticVariant::Increment | ArithmeticVariant::Decrement | 
            ArithmeticVariant::Random => &self.instruction.left_operand,
            _ => &self.instruction.right_operand
        };
        self.send(&destination.clone(), result)?;
        Ok(())
    }
}
