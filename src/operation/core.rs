use crate::operation::group::{
    Group, ArithmeticVariant, LogicVariant, BranchVariant, SubroutineVariant, 
    StackVariant, MemoryVariant, InputOutputVariant, SystemVariant,
};
use crate::operation::OperationError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Operation { 
    pub mnemonics: &'static [&'static str],
    pub group: Group,
    pub opcode: u8,
    pub default_mode: u8,
}

impl Operation {
    pub fn from_mnemonic(mnemonic: &str) -> Result<Self, OperationError> {
        OPERATIONS
            .iter()
            .find(|inst| inst.mnemonics.contains(&mnemonic))
            .copied()
            .ok_or_else(|| OperationError::IllegalMnemonic(mnemonic.to_string()))
    }

    pub fn from_byte(byte: u8) -> Result<Self, OperationError> {
        let base_opcode = byte & 0x7F;
        let mut operation = OPERATIONS
            .iter()
            .find(|inst| inst.opcode == base_opcode)
            .copied()
            .ok_or(OperationError::IllegalOpcode(byte))?;

        if byte & 0x80 != 0 {
            operation.opcode |= 0x80;
        }

        Ok(operation)
    }

    pub fn lookup_group_from_byte(byte: u8) -> Result<Group, OperationError> {
        let base_code = byte & 0x7F;
        OPERATIONS
            .iter()
            .find(|inst| inst.opcode == base_code)
            .map(|inst| inst.group.clone())
            .ok_or_else(|| OperationError::IllegalOpcode(byte))
    }

    pub fn has_default_mode(&self) -> bool {
        self.opcode >> 7 == 0
    }

    pub fn is_macro(code: &str) -> bool {
        MACRO_MNEMONICS.contains(&code)
    }

    pub fn is_directive(code: &str) -> bool {
        DIRECTIVES.contains(&code)
    }
}

static MACRO_MNEMONICS: &[&str] = &[
    "STRING", "ARRAY", "VAR", "NAME", "LINK"
];

static DIRECTIVES: &[&str] = &[
    "DATA", "LOGIC", "SUBROUTINES"
];

static OPERATIONS: &[Operation] = &[
    Operation { 
        mnemonics: &["ADD"],  
        group: Group::Arithmetic(ArithmeticVariant::Add),       
        opcode: 0x00,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["SUB"],  
        group: Group::Arithmetic(ArithmeticVariant::Subtract),  
        opcode: 0x01,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["MUL","MULT"], 
        group: Group::Arithmetic(ArithmeticVariant::Multiply), 
        opcode: 0x02,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["DIV"],  
        group: Group::Arithmetic(ArithmeticVariant::Divide),    
        opcode: 0x03,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["MOD","REM"], 
        group: Group::Arithmetic(ArithmeticVariant::Remainder), 
        opcode: 0x04,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["INC"],  
        group: Group::Arithmetic(ArithmeticVariant::Increment), 
        opcode: 0x05,
        default_mode: 0x9A,
    },
    Operation { 
        mnemonics: &["DEC"],  
        group: Group::Arithmetic(ArithmeticVariant::Decrement), 
        opcode: 0x06,
        default_mode: 0x9A,
    },
    Operation { 
        mnemonics: &["RAND", "RNG"], 
        group: Group::Arithmetic(ArithmeticVariant::Random),    
        opcode: 0x07,
        default_mode: 0x9B,
    },
    Operation { 
        mnemonics: &["SUM"],  
        group: Group::Arithmetic(ArithmeticVariant::Sum),       
        opcode: 0x08,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["DIF","DIFF"], 
        group: Group::Arithmetic(ArithmeticVariant::Difference), 
        opcode: 0x09,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["PRO","PROD"], 
        group: Group::Arithmetic(ArithmeticVariant::Product),    
        opcode: 0x0A,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["QUO","QUOT"], 
        group: Group::Arithmetic(ArithmeticVariant::Quotient),   
        opcode: 0x0B,
        default_mode: 0x29,
    },

    Operation { 
        mnemonics: &["AND"], 
        group: Group::Logic(LogicVariant::LogicalAnd), 
        opcode: 0x10,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["OR"],  
        group: Group::Logic(LogicVariant::InclusiveOr), 
        opcode: 0x11,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["XOR"], 
        group: Group::Logic(LogicVariant::ExclusiveOr), 
        opcode: 0x12,
        default_mode: 0x29,
    },
    Operation { 
        mnemonics: &["NOT", "FLIP"], 
        group: Group::Logic(LogicVariant::LogicalNot),  
        opcode: 0x13,
        default_mode: 0x9B,
    },
    Operation { 
        mnemonics: &["LEFT", "DBL"], 
        group: Group::Logic(LogicVariant::LeftShift),  
        opcode: 0x14,
        default_mode: 0x9A,
    },
    Operation { 
        mnemonics: &["RGHT", "HALF"], 
        group: Group::Logic(LogicVariant::RightShift), 
        opcode: 0x15,
        default_mode: 0x9A,
    },
    Operation { 
        mnemonics: &["WEST", "LRTT", "FRWD"], 
        group: Group::Logic(LogicVariant::LeftRotate),  
        opcode: 0x16,
        default_mode: 0x9A,
    },
    Operation { 
        mnemonics: &["EAST", "RRTT", "BACK"], 
        group: Group::Logic(LogicVariant::RightRotate), 
        opcode: 0x17,
        default_mode: 0x9A,
    },

    Operation { 
        mnemonics: &["COMP", "CMPR"], 
        group: Group::Branch(BranchVariant::Compare),  
        opcode: 0x20, 
        default_mode: 0x22,
    },
    Operation { 
        mnemonics: &["POS", "GRTR"], 
        group: Group::Branch(BranchVariant::Positive),   
        opcode: 0x21, 
        default_mode: 0x10,
    },
    Operation { 
        mnemonics: &["ZERO", "EQUL"], 
        group: Group::Branch(BranchVariant::Zero),      
        opcode: 0x22, 
        default_mode: 0x10,
    },
    Operation { 
        mnemonics: &["NEG", "LESS"], 
        group: Group::Branch(BranchVariant::Negative),   
        opcode: 0x23, 
        default_mode: 0x10,
    },

    Operation { 
        mnemonics: &["CALL"], 
        group: Group::Subroutine(SubroutineVariant::Call),  
        opcode: 0x30, 
        default_mode: 0x80,
    },
    Operation { 
        mnemonics: &["RTRN"], 
        group: Group::Subroutine(SubroutineVariant::Return), 
        opcode: 0x31, 
        default_mode: 0x00,
    },
    Operation { 
        mnemonics: &["JUMP", "GOTO"], 
        group: Group::Subroutine(SubroutineVariant::Jump),   
        opcode: 0x32, 
        default_mode: 0x80,
    },
    Operation { 
        mnemonics: &["JGT"],  
        group: Group::Subroutine(SubroutineVariant::JumpGreater), 
        opcode: 0x33, 
        default_mode: 0x82,
    },
    Operation { 
        mnemonics: &["JGE"],  
        group: Group::Subroutine(SubroutineVariant::JumpGreaterEqual), 
        opcode: 0x34, 
        default_mode: 0x82,
    },
    Operation { 
        mnemonics: &["JEQ"],  
        group: Group::Subroutine(SubroutineVariant::JumpEqual), 
        opcode: 0x35, 
        default_mode: 0x82,
    },
    Operation { 
        mnemonics: &["JLE"],  
        group: Group::Subroutine(SubroutineVariant::JumpLessEqual), 
        opcode: 0x36, 
        default_mode: 0x82,
    },
    Operation { 
        mnemonics: &["JLT"],  
        group: Group::Subroutine(SubroutineVariant::JumpLess), 
        opcode: 0x37, 
        default_mode: 0x82,
    },
    Operation { 
        mnemonics: &["JNE", "JNOT"], 
        group: Group::Subroutine(SubroutineVariant::JumpNotEqual), 
        opcode: 0x38, 
        default_mode: 0x82,
    },

    Operation { 
        mnemonics: &["PUSH"], 
        group: Group::Stack(StackVariant::Push),     
        opcode: 0x40,
        default_mode: 0x90,
    },
    Operation { 
        mnemonics: &["POP"],  
        group: Group::Stack(StackVariant::Pop),      
        opcode: 0x41,
        default_mode: 0x90,
    },
    Operation { 
        mnemonics: &["DUMP"], 
        group: Group::Stack(StackVariant::Dump),     
        opcode: 0x42,
        default_mode: 0x00,
    },
    Operation { 
        mnemonics: &["RSTR"], 
        group: Group::Stack(StackVariant::Restore),  
        opcode: 0x43,
        default_mode: 0x00,
    },

    Operation { 
        mnemonics: &["MOVE", "MOV"], 
        group: Group::Memory(MemoryVariant::Move),   
        opcode: 0x50,
        default_mode: 0x22,
    },
    Operation { 
        mnemonics: &["LOAD", "LD"], 
        group: Group::Memory(MemoryVariant::Load),   
        opcode: 0x51,
        default_mode: 0x12,
    },
    Operation { 
        mnemonics: &["SAVE", "STR"], 
        group: Group::Memory(MemoryVariant::Save),   
        opcode: 0x52,
        default_mode: 0x24,
    },
    Operation { 
        mnemonics: &["SWAP"], 
        group: Group::Memory(MemoryVariant::Swap),   
        opcode: 0x53,
        default_mode: 0x22,
    },

    Operation { 
        mnemonics: &["IN", "GET"],   
        group: Group::InputOutput(InputOutputVariant::StringInput), 
        opcode: 0x60,
        default_mode: 0x40,
    },
    Operation { 
        mnemonics: &["NIN"],  
        group: Group::InputOutput(InputOutputVariant::NumericInput), 
        opcode: 0x61,
        default_mode: 0x40,
    },
    Operation { 
        mnemonics: &["PRNT", "OUT"], 
        group: Group::InputOutput(InputOutputVariant::PrintString), 
        opcode: 0x62,
        default_mode: 0x40,
    },
    Operation { 
        mnemonics: &["TLLY", "NOUT"], 
        group: Group::InputOutput(InputOutputVariant::PrintNumber), 
        opcode: 0x63,
        default_mode: 0x40,
    },

    Operation { 
        mnemonics: &["HALT", "END"], 
        group: Group::System(SystemVariant::Halt),   
        opcode: 0x70,
        default_mode: 0x00,
    },
    Operation { 
        mnemonics: &["WAIT", "NOP"], 
        group: Group::System(SystemVariant::Wait),   
        opcode: 0x71,
        default_mode: 0x00,
    },
];
