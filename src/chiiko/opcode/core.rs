use crate::chiiko::opcode::{
    Group, Group::*, ArithmeticVariant, LogicVariant, BranchVariant, SubroutineVariant, 
    StackVariant, MemoryVariant, InputOutputVariant, SystemVariant,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Opcode {
    pub group: Group,
    pub mode: bool,
    pub byte: u8,
}

impl Opcode {
    pub fn decode(byte: u8) -> Self {
        let mode = byte >> 7 == 1;
        let group_number = (byte >> 4) & 7;
        let variant_number = byte & 15;

        Self {
            group: Self::parse_group(group_number, variant_number).unwrap(),
            mode: mode,
            byte: byte,
        }
    }

    fn parse_group(group: u8, variant: u8) -> Result<Group, &'static str> {
        match group {
            0 => Ok(Arithmetic(match variant {
                0 => ArithmeticVariant::Add,
                1 => ArithmeticVariant::Subtract,
                2 => ArithmeticVariant::Multiply,
                3 => ArithmeticVariant::Divide,
                4 => ArithmeticVariant::Remainder,
                5 => ArithmeticVariant::Increment,
                6 => ArithmeticVariant::Decrement,
                7 => ArithmeticVariant::Random,
                8 => ArithmeticVariant::Sum,
                9 => ArithmeticVariant::Difference,
                10 => ArithmeticVariant::Product,
                11 => ArithmeticVariant::Quotient,
                _ => return Err("Illegal ARITHMETIC Opcode variant"),
            })),
            1 => Ok(Logic(match variant {
                0 => LogicVariant::LogicalAnd,
                1 => LogicVariant::InclusiveOr,
                2 => LogicVariant::ExclusiveOr,
                3 => LogicVariant::LogicalNot,
                4 => LogicVariant::LeftShift,
                5 => LogicVariant::RightShift,
                6 => LogicVariant::LeftRotate,
                7 => LogicVariant::RightRotate,
                _ => return Err("Illegal LOGIC Opcode variant"),
            })),
            2 => Ok(Branch(match variant {
                0 => BranchVariant::Compare,
                1 => BranchVariant::Positive,
                2 => BranchVariant::Zero,
                3 => BranchVariant::Negative,
                _ => return Err("Illegal BRANCH Opcode variant"),
            })),
            3 => Ok(Subroutine(match variant {
                0 => SubroutineVariant::Call,
                1 => SubroutineVariant::Return,
                2 => SubroutineVariant::Jump,
                3 => SubroutineVariant::JumpGreater,
                4 => SubroutineVariant::JumpGreaterEqual,
                5 => SubroutineVariant::JumpEqual,
                6 => SubroutineVariant::JumpLessEqual,
                7 => SubroutineVariant::JumpLess,
                8 => SubroutineVariant::JumpNotEqual,
                _ => return Err("Illegal JUMP Opcode variant"),
            })),
            4 => Ok(Stack(match variant {
                0 => StackVariant::Push,
                1 => StackVariant::Pop,
                2 => StackVariant::Dump,
                3 => StackVariant::Restore,
                _ => return Err("Illegal STACK Opcode variant"),
            })),
            5 => Ok(Memory(match variant {
                0 => MemoryVariant::Move,
                1 => MemoryVariant::Load,
                2 => MemoryVariant::Save,
                3 => MemoryVariant::Swap,
                _ => return Err("Illegal MEMORY Opcode variant"),
            })),
            6 => Ok(InputOutput(match variant {
                0 => InputOutputVariant::StringInput,
                1 => InputOutputVariant::NumericInput,
                2 => InputOutputVariant::PrintString,
                3 => InputOutputVariant::PrintNumber,
                _ => return Err("Illegal IO Opcode variant"),
            })),
            7 => Ok(System(match variant {
                0 => SystemVariant::Halt,
                1 => SystemVariant::Wait,
                _ => return Err("Illegal SYSTEM Opcode variant"),
            })),
            _ => Err("Illegal group number"),
        }
    }

    pub fn default_grammar(&self) -> u8 {
        match &self.group {
            Arithmetic(variant) => match variant {
                // ADD, SUB, MULT, DIV, REM: [II] Register -> Accumulator
                ArithmeticVariant::Add | ArithmeticVariant::Subtract | ArithmeticVariant::Multiply | 
                ArithmeticVariant::Divide | ArithmeticVariant::Remainder => 0x29,
                // INC, DEC: [IV] Accumulator, 1
                ArithmeticVariant::Increment | ArithmeticVariant::Decrement => 0x9A, 
                // RAND: [IV] Accumulator, 255
                ArithmeticVariant::Random => 0x9B,
                // SUM, DIFF, PROD, QUO: [VII] register <- Accumulator (left must be a Register Pair)
                ArithmeticVariant::Sum | ArithmeticVariant::Difference | 
                ArithmeticVariant::Product | ArithmeticVariant::Quotient => 0x29,
            }
            Logic(variant) => match variant {
                // AND, OR, XOR: [II] Register -> Accumulator
                LogicVariant::LogicalAnd | LogicVariant::InclusiveOr | 
                LogicVariant::ExclusiveOr => 0x29,
                // NOT: [V] Accumulator
                LogicVariant::LogicalNot => 0x9B,
                // LEFT, RIGHT, WEST, EAST: [IV] Accumulator, 1
                LogicVariant::LeftShift | LogicVariant::RightShift |
                LogicVariant::LeftRotate | LogicVariant::RightRotate => 0x9A,
            },
            Branch(variant) => match variant {
                // COMP: [IV] Register, Register
                BranchVariant::Compare => 0x22,
                // POS, NEG, ZERO: [I] Value
                BranchVariant::Positive | BranchVariant::Zero | BranchVariant::Negative => 0x10,
            },
            Subroutine(variant) => match variant {
                // CALL, JUMP: [V] ROM Address
                SubroutineVariant::Call | SubroutineVariant::Jump => 0x80,
                // RTRN: [IX] none
                SubroutineVariant::Return => 0x00,
                // JGT, JGE, JEQ, JLE, JLT, JNE: [VII] ROM Address, Value
                SubroutineVariant::JumpGreater | SubroutineVariant::JumpGreaterEqual | 
                SubroutineVariant::JumpEqual | SubroutineVariant::JumpLessEqual | 
                SubroutineVariant::JumpLess | SubroutineVariant::JumpNotEqual => 0x82,
            },
            Stack(variant) => match variant {
                // PUSH: [I] Accumulator
                StackVariant::Push => 0x90,
                // POP: [V] -> Accumulator
                StackVariant::Pop => 0x90,
                // DUMP, RSTR: [IX] none
                StackVariant::Dump | StackVariant::Restore => 0x00,
            },
            Memory(variant) => match variant {
                // MOVE: [III] Register -> Register
                MemoryVariant::Move => 0x22,
                // LOAD: [III] Value -> Register
                MemoryVariant::Load => 0x12,
                // SAVE: [III] Register -> Zero-page Address
                MemoryVariant::Save => 0x24,
                // SWAP: [VIII] Register <-> Register
                MemoryVariant::Swap => 0x22,
            },
            InputOutput(variant) => match variant {
                // IN, NIN, PRNT, TLLY: [VI] Zero-page Address
                InputOutputVariant::StringInput | InputOutputVariant::NumericInput | 
                InputOutputVariant::PrintString | InputOutputVariant::PrintNumber => 0x40,
            },
            System(variant) => match variant {
                // HALT, WAIT: [IX] none
                SystemVariant::Halt | SystemVariant::Wait => 0x00,
            }
        }
    }
}