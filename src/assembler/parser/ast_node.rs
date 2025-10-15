use crate::operand::Operand;
use crate::mode::mode_group::ModeGroup;

#[derive(Debug, Clone)]
pub enum AstNode {
    Element {
        name: Option<String>,
        value: Option<u8>,
    },
}

