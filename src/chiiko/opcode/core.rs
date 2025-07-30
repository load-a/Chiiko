use crate::chiiko::opcode::{
    Group, Group::*, ArithmeticVariant, LogicVariant, BranchVariant, SubroutineVariant, 
    StackVariant, MemoryVariant, InputOutputVariant, SystemVariant,
};

#[derive(Debug, PartialEq)]
pub struct Opcode {
    pub group: Group,
    pub mode: bool,
    pub option: bool,
}

impl Opcode {
    pub fn decode(byte: u8) -> Result<Self, &'static str> {
        let mode = byte >> 7 == 1;
        let group_number = (byte >> 4) & 7;
        let option = (byte >> 3) & 1 == 1;
        let variant_number = byte & 7;

        Ok(Self {
            group: Self::parse_group(group_number, variant_number)?,
            mode: mode,
            option: option,
        })
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
                _ => return Err("Illegal ARITHMETIC Opcode variant"),
            })),
            1 => Ok(Logic(match variant {
                0 => LogicVariant::LogicalAnd,
                1 => LogicVariant::LogicalNot,
                2 => LogicVariant::InclusiveOr,
                3 => LogicVariant::ExclusiveOr,
                4 => LogicVariant::LeftShift,
                5 => LogicVariant::RightShift,
                _ => return Err("Illegal LOGIC Opcode variant"),
            })),
            2 => Ok(Branch(match variant {
                0 => BranchVariant::Compare,
                1 => BranchVariant::Positive,
                2 => BranchVariant::Negative,
                3 => BranchVariant::Zero,
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
                4 => InputOutputVariant::QueryKeyboard,
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
                // ADD, SUB, MULT, DIV, REM: [II] Value -> Accumulator
                ArithmeticVariant::Add | ArithmeticVariant::Subtract | ArithmeticVariant::Multiply | 
                ArithmeticVariant::Divide | ArithmeticVariant::Remainder => 0x19,
                // INC, DEC, RAND: [IV] Accumulator
                ArithmeticVariant::Increment | ArithmeticVariant::Decrement | 
                ArithmeticVariant::Random => 0x90,
            }
            Logic(variant) => match variant {
                // AND, NOT, OR, XOR: [II] Value -> Accumulator
                LogicVariant::LogicalAnd | LogicVariant::LogicalNot | LogicVariant::InclusiveOr | 
                LogicVariant::ExclusiveOr => 0x19,
                // LEFT, RIGHT: [IV] Accumulator
                LogicVariant::RightShift | LogicVariant::LeftShift => 0x90,
            },
            Branch(variant) => match variant {
                // COMP: [IV] Register, Register
                BranchVariant::Compare => 0x22,
                // POS, NEG, ZERO: [I] Value
                BranchVariant::Positive | BranchVariant::Negative | BranchVariant::Zero => 0x10,
            },
            Subroutine(variant) => match variant {
                // CALL: [V] ROM Address
                SubroutineVariant::Call => 0x80,
                // RTRN: [IX] none
                SubroutineVariant::Return => 0x00,
                // JUMP: [V] -> ROM Address
                SubroutineVariant::Jump => 0x80,
                // JGT, JGE, JEQ, JLE, JLT: [VII] ROM Address, Value
                SubroutineVariant::JumpGreater | SubroutineVariant::JumpGreaterEqual | 
                SubroutineVariant::JumpEqual | SubroutineVariant::JumpLessEqual | 
                SubroutineVariant::JumpLess => 0x81,
            },
            Stack(variant) => match variant {
                // PUSH: [I] Register
                StackVariant::Push => 0x20,
                // POP: [V] -> Register
                StackVariant::Pop => 0x20,
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
                InputOutputVariant::PrintString | InputOutputVariant::PrintNumber => 0x20,
                // KEY: [II] Value
                InputOutputVariant::QueryKeyboard => 0x10,
            },
            System(variant) => match variant {
                // HALT, WAIT: [IX] none
                SystemVariant::Halt | SystemVariant::Wait => 0x00,
            }
        }
    }
}