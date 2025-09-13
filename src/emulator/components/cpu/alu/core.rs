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
use crate::mode::Mode;

const ZERO_STATUS: u8 = 0b00000001;
const NEGATIVE_STATUS: u8 = 0b00000010;
const POSITIVE_STATUS: u8 = 0b00000000;
const DEFAULT_CHARACTER_LIMIT: u8 = 0xFF;
const NULL_CHARACTER: u8 = 0;

pub trait Alu {
    fn execute(&mut self) -> Result<(), CpuError>;
    fn evaluate_arithmetic(&mut self, variant: ArithmeticVariant) -> Result<(), CpuError>;
    fn evaluate_long_arithmetic(&mut self, variant: ArithmeticVariant) -> Result<(), CpuError>;
    fn evaluate_logic(&mut self, variant: LogicVariant) -> Result<(), CpuError>;
    fn evaluate_memory(&mut self, variant: MemoryVariant) -> Result<(), CpuError>;
}

impl Alu for Cpu {
    fn execute(&mut self) -> Result<(), CpuError> {
        // Instruction is fetched and stored as a field in CPU before this gets called
        let group = self.instruction.operation.group;

        match group {
            Group::Arithmetic(variant) => self.evaluate_arithmetic(variant),
            Group::Logic(variant) => self.evaluate_logic(variant),
            Group::Memory(variant) => self.evaluate_memory(variant),
            _ => return Err(AluError::CannotFetchInstruction)?
        }
    }

    fn evaluate_arithmetic(&mut self, variant: ArithmeticVariant) -> Result<(), CpuError> {
        if self.instruction.operation.is_long() {
            return self.evaluate_long_arithmetic(variant)
        }

        let left = self.find(&self.instruction.left_operand)?;
        let right = self.find(&self.instruction.right_operand)?;

        // println!("\n{:?}", &self.instruction);

        let (result, overflow) = match variant {
            ArithmeticVariant::Add | ArithmeticVariant::Increment => {
                left.overflowing_add(right)
            }
            ArithmeticVariant::Subtract | ArithmeticVariant::Decrement => {
                left.overflowing_sub(right)
            }
            ArithmeticVariant::Multiply => left.overflowing_mul(right),
            ArithmeticVariant::Divide => {
                if right == 0 {
                    return Err(AluError::DivisionByZero)?
                }
                left.overflowing_div(right)
            }
            ArithmeticVariant::Remainder => {
                if right == 0 {
                    return Err(AluError::DivisionByZero)?
                }
                left.overflowing_rem(right)
            }
            ArithmeticVariant::Random => rand::rng().random::<u8>().overflowing_rem(right),
            _ => return Err(AluError::DivisionByZero)?
        };
        self.update_flags(result, overflow);

        let destination = match variant {
            ArithmeticVariant::Increment | ArithmeticVariant::Decrement | 
            ArithmeticVariant::Random => &self.instruction.left_operand,
            _ => &self.instruction.right_operand
        };
        self.send(&destination.clone(), result)?; // Clone to prevent borrow errors
        Ok(())
    }

    fn evaluate_long_arithmetic(&mut self, variant: ArithmeticVariant) -> Result<(), CpuError> {
        if self.instruction.operation.default_mode != Mode::as_byte(self.instruction.mode) {
            return Err(
                AluError::LongModeError(self.instruction.operation.mnemonics[0].to_string())
            )?
        }

        let register_code = self.instruction.left_operand.value().unwrap() as u8;
        let left = self.read_register_pair(register_code)?;
        let right = self.find(&self.instruction.right_operand)? as u16;

        // println!("\n{:?}", &self.instruction);

        let result = match variant {
            ArithmeticVariant::Sum => left.wrapping_add(right),
            ArithmeticVariant::Difference => left.wrapping_sub(right),
            ArithmeticVariant::Product => left.wrapping_mul(right),
            ArithmeticVariant::Quotient => {
                if right == 0 {
                    return Err(AluError::DivisionByZero)?
                }

                let quotient = left.wrapping_div(right) as u8;
                let remainder = left.wrapping_rem(right) as u8;

                u16::from_be_bytes([quotient, remainder])
            },
            _ => todo!()
        };

        self.write_register_pair(register_code, result)?;
        return Ok(())
    }

    fn evaluate_logic(&mut self, variant: LogicVariant) -> Result<(), CpuError> {
        let left = self.find(&self.instruction.left_operand)?;
        let right = self.find(&self.instruction.right_operand)?;

        // println!("\n{:?}", &self.instruction);

        let result = match variant {
            LogicVariant::LogicalAnd => left & right,
            LogicVariant::InclusiveOr => left | right,
            LogicVariant::ExclusiveOr | LogicVariant::LogicalNot => left ^ right,
            LogicVariant::LeftShift => {
                if (left << right.saturating_sub(1)) & 0b10000000 > 0 { 
                    self.clear_flags();
                    self.set_carry();
                }
                left << right
            },
            LogicVariant::RightShift => {
                if (left >> right.saturating_sub(1)) > 0 { 
                    self.clear_flags();
                    self.set_carry();
                }
                left >> right
            },
            LogicVariant::LeftRotate => left.rotate_left(right as u32),
            LogicVariant::RightRotate => left.rotate_right(right as u32),
        };

        let destination = match variant {
            LogicVariant::LogicalNot | LogicVariant::LeftShift | LogicVariant::RightShift |
            LogicVariant::LeftRotate | LogicVariant::RightRotate => &self.instruction.left_operand,
            _ => &self.instruction.right_operand
        };

        self.send(&destination.clone(), result)?; // Clone to prevent borrow errors
        Ok(())
    }

    fn evaluate_memory(&mut self, variant: MemoryVariant) -> Result<(), CpuError> {
        let source = self.find(&self.instruction.left_operand)?;
        let target = self.instruction.right_operand.clone();

        match variant {
            MemoryVariant::Move | MemoryVariant::Load => (),
            MemoryVariant::Save => {
                if self.instruction.mode.1 == Mode::from_key("R")? {
                    return Err(
                        AluError::ModeError("SAVE requires ([source], [Not Register])".to_string())
                    )?
                }
            }
            MemoryVariant::Swap => {
                if self.instruction.operation.default_mode != Mode::as_byte(self.instruction.mode) {
                    return Err(
                        AluError::ModeError("SWAP requires (REGISTER, REGISTER)".to_string())
                    )?
                }

                let value = self.find(&self.instruction.right_operand)?;
                self.send(&self.instruction.left_operand.clone(), value)?;
            }
        };

        self.send(&target, source)?;

        Ok(())
    }
}
