use rand::Rng;
use crate::chiiko::components::{
    cpu::Cpu, instruction::Instruction, operand::Operand, operand::Operand::JumpAddress,
};
use crate::chiiko::opcode::{
    ArithmeticVariant, LogicVariant, BranchVariant, SubroutineVariant, StackVariant, MemoryVariant,
    InputOutputVariant, SystemVariant,
};
use crate::chiiko::opcode::Group::{Arithmetic, Logic, Branch, Subroutine, Stack, Memory, InputOutput, System};

const ZERO_STATUS: u8 = 0b00000001;
const NEGATIVE_STATUS: u8 = 0b00000010;
const POSITIVE_STATUS: u8 = 0b00000000;

pub trait Alu {
    fn execute(&mut self, instruction: Instruction) -> Result<(), &'static str>;
    fn evaluate_arithmetic(
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
    // fn evaluate_io(
    // &mut self, 
    // variant: &InputOutputVariant, 
    // instruction: &Instruction
    // ) -> Result<(), &'static str>;
}

impl Alu for Cpu {
    fn execute(&mut self, instruction: Instruction) -> Result<(), &'static str> {
        match &instruction.opcode.group {
            Arithmetic(variant) => self.evaluate_arithmetic(&variant, &instruction),
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
        let mut right = self.find(instruction.right_operand)?;

        if matches!(variant, ArithmeticVariant::Increment | ArithmeticVariant::Decrement) 
        && right == 0 {
            right = 1;
        }

        let (result, overflow) = match variant {
            ArithmeticVariant::Add => left.overflowing_add(right),
            ArithmeticVariant::Subtract => left.overflowing_sub(right),
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
            ArithmeticVariant::Increment => left.overflowing_add(right),
            ArithmeticVariant::Decrement => left.overflowing_sub(right),
            ArithmeticVariant::Random => (rand::rng().random(), false), // Rust has changed `rand::thread_rng().gen()` into this
        };

        if overflow { self.set_carry() }
        self.set_zero_or_negative(result);

        self.send(instruction.right_operand, result)?;
        Ok(())
    }

    fn evaluate_logic(
    &mut self, 
    variant: &LogicVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        self.clear_flags();

        let left = self.find(instruction.left_operand)?;
        let mut right = self.find(instruction.right_operand)?;

        if matches!(variant, LogicVariant::LeftShift | LogicVariant::RightShift) 
        && right == 0 {
            right = 1;
        }

        let result = match variant {
            LogicVariant::LogicalAnd => left & right,
            LogicVariant::LogicalNot => left ^ 0xFF,
            LogicVariant::InclusiveOr => left | right,
            LogicVariant::ExclusiveOr => left ^ right,
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

        self.send(instruction.right_operand, result)?;
        Ok(())
    }

    fn evaluate_branch(
    &mut self, 
    variant: &BranchVariant, 
    instruction: &Instruction
    ) -> Result<(), &'static str> {
        self.clear_flags();
        
        let left = self.find(instruction.left_operand)?;

        match variant {
            BranchVariant::Compare => {
                let right = self.find(instruction.right_operand)?;
                let result = left.wrapping_sub(right);

                self.set_zero_or_negative(result);
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
        let mut address = 0;

        if !matches!(variant, SubroutineVariant::Return) {
            if let Operand::JumpAddress(value) = instruction.left_operand { 
                address = value
            } else {
                return Err("Invalid Jump Operand")
            }
        };
        
        let right = self.find(instruction.right_operand)?;

        match variant {
            SubroutineVariant::Call => {
                let [high, low] = self.program_counter.to_be_bytes();

                self.push(high)?;
                self.push(low)?;

                self.set_pc(address)
            },
            SubroutineVariant::Return => {
                let low = self.pop()?;
                let high = self.pop()?;
                self.set_pc(u16::from_be_bytes([high, low]));
            },
            SubroutineVariant::Jump => {
                self.set_pc(address)
            },
            SubroutineVariant::JumpGreater => {
                if right > self.accumulator { self.set_pc(address) }
            },
            SubroutineVariant::JumpGreaterEqual => {
                if right >= self.accumulator { self.set_pc(address) }
            },
            SubroutineVariant::JumpEqual => {
                if right == self.accumulator { self.set_pc(address) }
            },
            SubroutineVariant::JumpLessEqual => {
                if right <= self.accumulator { self.set_pc(address) }
            },
            SubroutineVariant::JumpLess => {
                if right < self.accumulator { self.set_pc(address) }
            },
            SubroutineVariant::JumpNotEqual => {
                if right != self.accumulator { self.set_pc(address) }
            },
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
                if matches!(instruction.right_operand, 
                    Operand::Register(_) | Operand::IndirectRegister(_)) 
                {
                    return Err("Cannot SAVE to Register");
                } else {
                    self.send(instruction.left_operand, right)?
                }
            },
            MemoryVariant::Swap => {
                if !(matches!(instruction.right_operand, Operand::Register(_)) 
                    && matches!(instruction.left_operand, Operand::Register(_))) 
                {
                    return Err("Can only SWAP Between Registers");
                } else {
                    self.send(instruction.left_operand, right)?
                }
                self.send(instruction.right_operand, left)?;
                self.send(instruction.left_operand, right)?;
            }
        }

        Ok(())
    }

    // fn evaluate_io(
    // &mut self, 
    // variant: &InputOutputVariant, 
    // instruction: &Instruction
    // ) -> Result<(), &'static str> {
    //     match variant {
    //         InputOutputVariant::StringInput => {
                
    //         }
    //     }
    // }
}