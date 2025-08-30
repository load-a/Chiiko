use crate::assembler::assembly_error::AssemblyError;
use crate::assembler::parser::{Parser, ast_node::ASTNode, ast_node::MacroNode,
    assembler_operand::AssemblerOperand
};
use crate::operation::Operation;
use crate::mode::{Mode, mode_group::ModeGroup};

pub mod SyntaxChecker {
    use super::*;

    pub fn check(source: Vec<ASTNode>) -> Result<(), AssemblyError> {
        for node in source {
            match node {
                ASTNode::Macro(macro_node) => check_macro(macro_node),
                ASTNode::Directive(header) => {
                    if !Operation::is_directive(&header) {
                        panic!("Invalid Directive: {}", header)
                    }
                },
                ASTNode::Instruction {ref mnemonic, ref mode, ref operands} => {
                    if operands.len() > 2 {
                        panic!("Syntax Error: Too many operands {:?} {:?}", mnemonic, operands)
                    }

                    let operation = Operation::from_mnemonic(&mnemonic);
                    let default_mode = Mode::groups_from_byte(operation.default_mode);
                    let set_mode = if mode.is_some() {
                        mode.clone().unwrap()
                    } else {
                        Mode::default_tuple()
                    };
                    let inferred_mode = infer_mode(&operands);

                    // Diagnostic info
                    // println!(
                    // "{:?} \n\tdef: {:?} \n\tset: {:?} \n\tinf: {:?}", 
                    // operation, default_mode, set_mode, inferred_mode
                    // );

                    if !Mode::are_compatible(default_mode.clone(), inferred_mode.clone()) {
                        if !Mode::are_compatible(set_mode.clone(), inferred_mode.clone()) {
                            panic!(
                            "Mode Conflict: {:?} \n\tdef: {:?} \n\tset: {:?} \n\tinf: {:?}", 
                            node.clone(), default_mode, set_mode, inferred_mode
                            )
                        }
                    }

                    for operand in operands {
                        match operand {
                            AssemblerOperand::Register(id) => {
                                if !AssemblerOperand::is_valid_register(id) {
                                    panic!("Invalid Register Code: {}", id)
                                }
                            }
                            _ => ()
                        }
                    }
                },
                ASTNode::Error(message) => panic!("{}", message),
                _ => ()
            }
        }

        Ok(())
    }

    pub fn infer_mode(operands: &Vec<AssemblerOperand>) -> (ModeGroup, ModeGroup) {
        if operands.is_empty() {
            Mode::default_tuple()
        } else if operands.len() == 1 {
            (determine_operand_mode(&operands[0]), ModeGroup::Default)
        } else {
            (determine_operand_mode(&operands[0]), determine_operand_mode(&operands[1]))
        }
    }

    fn determine_operand_mode(operand: &AssemblerOperand) -> ModeGroup {
        match operand {
            AssemblerOperand::Number(_) | 
            AssemblerOperand::StartCount(_) => ModeGroup::Value,
            AssemblerOperand::Register(_) => ModeGroup::Register,
            AssemblerOperand::DirectAddress(id) => {
                if let Ok(number) = Parser::normalize_number(&id) {
                    if number > 0xFF { 
                        ModeGroup::DirectAddress 
                    } else {
                        ModeGroup::ZeroPage
                    }
                } else if id.len() < 3 {
                    ModeGroup::Register
                } else {
                    ModeGroup::DirectAddress
                }
            },
            AssemblerOperand::IndirectAddress(id) => {
                if let Ok(number) = Parser::normalize_number(&id) {
                    if number > 0xFF { 
                        ModeGroup::IndirectAddress 
                    } else {
                        ModeGroup::IndirectZeroPage
                    }
                } else if id.len() < 3 {
                    ModeGroup::IndirectRegister
                } else {
                    ModeGroup::IndirectAddress
                }
            },
            AssemblerOperand::JumpAddress(_) => ModeGroup::JumpAddress,
                AssemblerOperand::String(_) | AssemblerOperand::Error(_) |
                AssemblerOperand::Placeholder(_) | AssemblerOperand::EndCount |
                AssemblerOperand::NamedElement {..} => ModeGroup::Error,
            AssemblerOperand::Identifier(_) => ModeGroup::Register,
            _ => ModeGroup::Error,
        }
    }

    fn check_macro(node: MacroNode) {
        match node {
            MacroNode::LinkData(operand) => (),
            MacroNode::StringData { address, value } => {
                if !address.is_destination() {
                    panic!("Macro Error: Invalid STRING destination: {:?}", address);
                }

                if let AssemblerOperand::String(string) = value {
                    ()
                } else {
                    panic!("Macro Error: Invalid STRING source: {:?}", value);
                }
            }
            MacroNode::MacroError(message) => panic!("{}", message),
            _ => ()
        }
    }
}
