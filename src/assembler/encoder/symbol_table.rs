use std::collections::HashMap;
use crate::assembler::parser::{Parser, ast_node::ASTNode, ast_node::MacroNode,
    assembler_operand::AssemblerOperand
};

pub struct SymbolTable {
    pub table: HashMap<String, Symbol>,
}

#[derive(Debug)]
pub enum Symbol {
    // Address(u8),
    Variable {address: u16, value: u8},
    Address(u16),
    Directive,
    Counter {start: u16, end: u16}
}

impl SymbolTable {
    pub fn from_ast(ast_tree: &Vec<ASTNode>) -> Self {
        let mut table: HashMap<String, Symbol> = HashMap::new();

        for node in ast_tree {
            match node {
                ASTNode::Macro(MacroNode::VariableData {address, label}) => {
                    let number = Parser::normalize_number(&address.string().unwrap())
                        .unwrap_or_else(|_| panic!("Bad address: {:?}", address));
                    table.insert(
                        label.string().unwrap(), 
                        Symbol::Variable {
                            address: number as u16,
                            value: 0,
                        }
                    );
                },
                ASTNode::Macro(MacroNode::ArrayData {address, elements}) => {
                    let address = Parser::normalize_number(&address.string().unwrap())
                        .unwrap_or_else(|_| panic!("Bad address: {:?}", address));

                    for (offset, element) in elements.iter().enumerate() {
                        if let AssemblerOperand::NamedElement { name, value } = element {
                            table.insert(
                                name.to_string(), 
                                Symbol::Variable {
                                    address: (address + offset) as u16, 
                                    value: *value as u8
                                }
                            );
                        } else if let AssemblerOperand::Identifier(label) = element {
                            table.insert(
                                label.to_string(),
                                Symbol::Variable {
                                    address: (address + offset) as u16, 
                                    value: 0
                                }
                            );
                        }
                    }
                },
                ASTNode::Instruction {operands, ..} => {
                    for operand in operands {
                        if let AssemblerOperand::StartCount(id) = operand {
                            table.insert(
                                format!("&START_COUNT<{}>", id),
                                Symbol::Counter { start: 0, end: 0 }
                            );
                        } if let AssemblerOperand::DirectAddress(label) | 
                            AssemblerOperand::IndirectAddress(label) = operand {
                            if Parser::normalize_number(&label).is_ok() || label.len() < 3 {
                                continue;
                            }

                            if !table.contains_key(label)  {
                                table.insert(
                                    label.to_string(),
                                    Symbol::Variable {
                                        address: 0, 
                                        value: 0
                                    }
                                );
                            }
                        }
                    }
                },
                ASTNode::Label(string) => { table.insert(string.to_string(), Symbol::Address(0)); },
                _ => ()
            }
        }

        Self { table: table }
    }
}
