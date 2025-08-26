use crate::assembler::parser::{assembler_operand::AssemblerOperand, opcode::Opcode};
use crate::mode::mode_group::ModeGroup;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Instruction {
        mnemonic: String,
        mode: Option<(ModeGroup, ModeGroup)>,
        operands: Vec<AssemblerOperand>,
    },
    Macro(MacroNode),
    Directive(String),
    Label(String),
    Error(String)
}

#[derive(Debug, Clone)]
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
        id: usize,
    },
    VariableData {
        address: AssemblerOperand,
        label: AssemblerOperand
    },
    LinkData(String),
    MacroError(String),
}
