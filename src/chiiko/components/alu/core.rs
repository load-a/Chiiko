use std::io;
use rand::Rng;
use crate::chiiko::components::{
    cpu::Cpu, chip::Chip, instruction::Instruction, cpu_operand::CpuOperand, cpu_operand::CpuOperand::JumpAddress,
    cpu_operand::CpuOperand::Register,
};
use crate::operation::group::{
    Group, ArithmeticVariant, LogicVariant, BranchVariant, SubroutineVariant, 
    StackVariant, MemoryVariant, InputOutputVariant, SystemVariant,
};

const ZERO_STATUS: u8 = 0b00000001;
const NEGATIVE_STATUS: u8 = 0b00000010;
const POSITIVE_STATUS: u8 = 0b00000000;
const DEFAULT_CHARACTER_LIMIT: u8 = 0xFF;
const NULL_CHARACTER: u8 = 0;

pub trait Alu {
    fn execute(&mut self) -> Result<(), &'static str>;
    fn evaluate_arithmetic(
    &mut self, 
    variant: &ArithmeticVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_16bit_arithmetic(
    &mut self, 
    variant: &ArithmeticVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_logic(
    &mut self, 
    variant: &LogicVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_branch(
    &mut self, 
    variant: &BranchVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_subroutine(
    &mut self, 
    variant: &SubroutineVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_stack(
    &mut self, 
    variant: &StackVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_memory(
    &mut self, 
    variant: &MemoryVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_io(
    &mut self, 
    variant: &InputOutputVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
    fn evaluate_system(
    &mut self, 
    variant: &SystemVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str>;
}

impl Alu for Cpu {
    fn execute(&mut self) -> Result<(), &'static str> {
        let instruction = self.instruction;

        match &instruction.operation.group {
            Group::Arithmetic(variant) => {
                if matches!(variant, 
                ArithmeticVariant::Sum | ArithmeticVariant::Difference | 
                ArithmeticVariant::Product | ArithmeticVariant::Quotient
                ) {
                    self.evaluate_16bit_arithmetic(&variant, &instruction)
                } else {
                    self.evaluate_arithmetic(&variant, &instruction)
                }
            },
            Group::Logic(variant) => self.evaluate_logic(&variant, &instruction),
            Group::Branch(variant) => self.evaluate_branch(&variant, &instruction),
            Group::Subroutine(variant) => self.evaluate_subroutine(&variant, &instruction),
            Group::Stack(variant) => self.evaluate_stack(&variant, &instruction),
            Group::System(variant) => self.evaluate_system(&variant, &instruction),
            _ => Err("Invalid Instruction")
        }
    }

    fn evaluate_arithmetic(
    &mut self, 
    variant: &ArithmeticVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        self.clear_flags();
        
        let left = self.find(instruction.left_operand)?;
        let right = self.find(instruction.right_operand)?;

        let (result, overflow) = match variant {
            ArithmeticVariant::Add | ArithmeticVariant::Increment => left.overflowing_add(right),
            ArithmeticVariant::Subtract | ArithmeticVariant::Decrement => left.overflowing_sub(right),
            ArithmeticVariant::Multiply => left.overflowing_mul(right),
            ArithmeticVariant::Divide => if right == 0 {
                return Err("Division by zero")
            } else { 
                left.overflowing_div(right)
            },
            ArithmeticVariant::Remainder => if right == 0 {
                return Err("Modulo by zero")
            } else { 
                left.overflowing_rem(right)
            },
            ArithmeticVariant::Random => rand::rng().random::<u8>().overflowing_rem(right),
            _ => return Err("Invalid Single Word Arithmetic")
        };

        if overflow { self.set_carry() }
        self.set_zero_or_negative(result);

        if matches!(
        variant, 
        ArithmeticVariant::Increment | ArithmeticVariant::Decrement | ArithmeticVariant::Random
        ) {
            self.send(instruction.left_operand, result)
        } else {
            self.send(instruction.right_operand, result)
        }
    }

    fn evaluate_16bit_arithmetic(
    &mut self, 
    variant: &ArithmeticVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        self.clear_flags();

        if !instruction.left_operand.is_register_pair() {
            return Err("Invalid right cpu_operand for 16-bit arithmetic")
        }

        let register_code = if let Register(register_code) = instruction.left_operand { 
            register_code 
        } else { 
            return Err("Invalid pair code") 
        };
        let left = self.read_register_pair(register_code)?;
        let right = self.find(instruction.right_operand)? as u16;

        let (result, overflow) = match variant {
            ArithmeticVariant::Sum => left.overflowing_add(right),
            ArithmeticVariant::Difference => left.overflowing_sub(right),
            ArithmeticVariant::Product => left.overflowing_mul(right),
            ArithmeticVariant::Quotient => {
                if right == 0 {
                    return Err("16-bit Division by zero")
                } else {
                    left.overflowing_div(right)
                }
            },
            _ => return Err("Invalid 16-bit arithmetic CpuOperand")
        };
        
        if overflow { self.set_carry() }
        if result == 0 { self.set_zero() }
        if result & 0b10000000_00000000 > 0 { self.set_negative() }

        self.write_register_pair(register_code, result)
    }

    fn evaluate_logic(
    &mut self, 
    variant: &LogicVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        self.clear_flags();

        let left = self.find(instruction.left_operand)?;
        let right = self.find(instruction.right_operand)?;

        let result = match variant {
            LogicVariant::LogicalAnd => left & right,
            LogicVariant::InclusiveOr => left | right,
            LogicVariant::ExclusiveOr | LogicVariant::LogicalNot => left ^ right,
            LogicVariant::LeftShift => {
                if (left << right.saturating_sub(1)) & 0b10000000 > 0 { self.set_carry() }
                left << right
            },
            LogicVariant::RightShift => {
                if (left >> right.saturating_sub(1)) > 0 { self.set_carry() }
                left >> right
            },
            LogicVariant::LeftRotate => left.rotate_left(right as u32),
            LogicVariant::RightRotate => left.rotate_right(right as u32),
        };

        self.set_zero_or_negative(result);

        if matches!(
        variant, 
        LogicVariant::LogicalNot | LogicVariant::LeftShift | LogicVariant::RightShift |
        LogicVariant::LeftRotate | LogicVariant::RightRotate
        ) {
            self.send(instruction.left_operand, result)
        } else {
            self.send(instruction.right_operand, result)
        }
    }

    fn evaluate_branch(
    &mut self, 
    variant: &BranchVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {        
        let left = self.find(instruction.left_operand)?;

        match variant {
            BranchVariant::Compare => {
                self.clear_flags();

                let right = self.find(instruction.right_operand)?;
                let result = left.wrapping_sub(right);

                self.set_zero_or_negative(result)
            },
            BranchVariant::Positive => {
                if self.status & 0b00000011 == POSITIVE_STATUS {
                    self.relative_jump(left)
                }
            },
            BranchVariant::Zero => {
                if self.status & 0b00000011 == ZERO_STATUS {
                    self.relative_jump(left)
                }
            },
            BranchVariant::Negative => {
                if self.status & 0b00000011 == NEGATIVE_STATUS {
                    self.relative_jump(left)
                }
            },
        }

        Ok(())
    }

    fn evaluate_subroutine(
    &mut self, 
    variant: &SubroutineVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {

        if let SubroutineVariant::Return = variant {
            let low = self.pop()?;
            let high = self.pop()?;
            self.set_pc(u16::from_be_bytes([high, low]));
            return Ok(());
        };

        if !instruction.left_operand.is_jump() {
            return Err("Invalid Jump CpuOperand")
        } 

        let address = match instruction.left_operand {
            CpuOperand::JumpAddress(value) | CpuOperand::MemoryAddress(value) => value,
            CpuOperand::Register(register_code) => self.read_register_pair(register_code)?,
            _ => return Err("Cannot get address from Subroutine CpuOperand")
        };
        
        let right = self.find(instruction.right_operand)?;

        match variant {
            SubroutineVariant::Call => {
                let [high, low] = self.program_counter.to_be_bytes();

                self.push(high)?;
                self.push(low)?;

                self.set_pc(address)
            },
            SubroutineVariant::Jump => {
                self.set_pc(address)
            },
            SubroutineVariant::JumpGreater if right > self.accumulator => self.set_pc(address),
            SubroutineVariant::JumpGreaterEqual if right >= self.accumulator => self.set_pc(address),
            SubroutineVariant::JumpEqual if right == self.accumulator => self.set_pc(address),
            SubroutineVariant::JumpLessEqual if right <= self.accumulator => self.set_pc(address),
            SubroutineVariant::JumpLess if right < self.accumulator => self.set_pc(address),
            SubroutineVariant::JumpNotEqual if right != self.accumulator => self.set_pc(address),
            _ => ()
        }

        Ok(())
    }

    fn evaluate_stack(
    &mut self, 
    variant: &StackVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        match variant {
            StackVariant::Push => {
                let value = self.find(instruction.left_operand)?;
                self.push(value)?;
            },
            StackVariant::Pop => {
                let value = self.pop()?;
                self.send(instruction.left_operand, value)?;
            },
            StackVariant::Dump => {
                self.push(self.accumulator)?;
                self.push(self.b_register)?;
                self.push(self.c_register)?;
                self.push(self.h_register)?;
                self.push(self.l_register)?;
                self.push(self.i_register)?;
                self.push(self.j_register)?;                
            },
            StackVariant::Restore => {
                self.j_register = self.pop()?;                
                self.i_register = self.pop()?;
                self.l_register = self.pop()?;
                self.h_register = self.pop()?;
                self.c_register = self.pop()?;
                self.b_register = self.pop()?;
                self.accumulator = self.pop()?;
            },
        }

        Ok(())
    }

    fn evaluate_memory(
    &mut self, 
    variant: &MemoryVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        let left = self.find(instruction.left_operand)?;
        let right = self.find(instruction.right_operand)?;

        match variant {
            MemoryVariant::Move | MemoryVariant::Load => self.send(instruction.right_operand, left)?,
            MemoryVariant::Save => {
                if instruction.right_operand.is_register() {
                    self.send(instruction.left_operand, right)?
                } else {
                    return Err("Cannot SAVE to Register");
                }
            },
            MemoryVariant::Swap => {
                if instruction.right_operand.is_register() && instruction.left_operand.is_register()
                {
                    self.send(instruction.left_operand, right)?
                } else {
                    return Err("Can only SWAP Between Registers");
                }
                self.send(instruction.right_operand, left)?;
                self.send(instruction.left_operand, right)?;
            }
        }

        Ok(())
    }

    fn evaluate_io(
    &mut self, 
    variant: &InputOutputVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        if !instruction.left_operand.is_address() {
            return Err("IO Error: Left cpu_operand must be a memory address.")
        }
        let address = self.resolve_address(&instruction.left_operand)?;
        let limit: u8 = match instruction.right_operand {
            CpuOperand::None => DEFAULT_CHARACTER_LIMIT,
            _ => self.find(instruction.right_operand)?
        };

        match variant {
            InputOutputVariant::StringInput => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).map_err(|_| "Failed to read input")?;

                for (offset, byte) in input.bytes().take(limit as usize).enumerate() {
                    self.write(address + offset as u16, byte)?;
                }

                Ok(())
            },
            InputOutputVariant::NumericInput => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).map_err(|_| "Failed to read number")?;
                let number: u8 = input.trim()
                .parse()
                .map_err(|_| "Invalid number input")?;

                self.write(address, number)
            },
            InputOutputVariant::PrintString => {
                let mut line: Vec<u8> = Vec::new();

                for offset in 0..limit {
                    let byte = self.read(address + offset as u16);
                    if byte == NULL_CHARACTER { break; }
                    line.push(byte);
                }

                if let Ok(output) = String::from_utf8(line) {
                    print!("{}", output);
                } else {
                    return Err("Invalid UTF-8 in string output");
                }

                Ok(())
            },
            InputOutputVariant::PrintNumber => {
                println!("{}", self.read(address));
                Ok(())
            },
        }
    }

    fn evaluate_system(
    &mut self, 
    variant: &SystemVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        match variant {
            SystemVariant::Halt => Ok(self.set_pc(0xFFFF)),
            SystemVariant::Wait => Ok(()),
        }
    }
}
