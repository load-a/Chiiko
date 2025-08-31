use crate::operand::Operand;
use crate::mode::mode_group::ModeGroup;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Instruction {
        mnemonic: String,
        mode: Option<(ModeGroup, ModeGroup)>,
        operands: Vec<Operand>,
    },
    Macro(MacroNode),
    Directive(String),
    Label(String),
    Error(String)
}

#[derive(Debug, Clone)]
pub enum MacroNode {
    ArrayData {
        address: Operand,
        elements: Vec<Operand>
    },
    StringData {
        address: Operand,
        value: Operand,
    },
    EndCount {
        id: usize,
    },
    VariableData {
        address: Operand,
        label: Operand
    },
    LinkData(String),
    MacroError(String),
}
