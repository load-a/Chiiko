use std::collections::HashMap;

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
