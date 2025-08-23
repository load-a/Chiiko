use crate::assembler::parser::{assembler_operand::AssemblerOperand, opcode::Opcode, mode_key::ModeKey};

#[derive(Debug)]
pub enum ASTNode {
    Instruction {
        mnemonic: String,
        mode: Option<(ModeKey, ModeKey)>,
        operands: Vec<AssemblerOperand>,
    },
    Macro(MacroNode),
    Directive(String),
    Label(String),
    Error(String)
}

#[derive(Debug)]
pub enum MacroNode {
    ArrayData {
        address: AssemblerOperand,
        elements: Vec<AssemblerOperand>
    },
    StringData {
        address: AssemblerOperand,
        value: AssemblerOperand,
    },
    EndCount {
        address: u16,
    },
    VariableData {
        address: AssemblerOperand,
        label: AssemblerOperand
    }
}
