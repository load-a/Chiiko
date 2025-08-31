use std::collections::HashMap;
use crate::assembler::parser::{Parser, ast_node::ASTNode, ast_node::MacroNode};
use crate::operand::Operand;

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
                    table.insert(
                        label, 
                        Symbol::Variable {
                            address: address, // Why doesn't it see its own fields?
                            value: 0,
                        }
                    );
                },
                ASTNode::Macro(MacroNode::ArrayData {address, elements}) => {
                    for (offset, element) in elements.iter().enumerate() {
                        if let Operand::Element { id, value, ..} = element {
                            table.insert(
                                id, 
                                Symbol::Variable {
                                    address: (address + offset) as u16, 
                                    value: *value as u8
                                }
                            );
                        // } else if let Operand::Identifier(label) = element {
                        //     table.insert(
                        //         label.to_string(),
                        //         Symbol::Variable {
                        //             address: (address + offset) as u16, 
                        //             value: 0
                        //         }
                        //     );
                        }
                    }
                },
                ASTNode::Instruction {operands, ..} => {
                    for operand in operands {
                        if let Operand::StartCount(id) = operand {
                            table.insert(
                                format!("&START_COUNT<{}>", id),
                                Symbol::Counter { start: 0, end: 0 }
                            );
                        } if let Operand::Address { id, location, direct } = operand {
                            if Parser::normalize_number(&id).is_ok() || id.len() < 3 {
                                continue;
                            }

                            if !table.contains_key(id)  {
                                table.insert(
                                    id.to_string(),
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
