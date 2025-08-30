use crate::register::Register;

#[derive(PartialEq, Debug)]
pub enum Operand {
    NoOperand,
    Number { id: String, value: u16 },
    RegisterOp(Register), // Convert into the actual Register later
    Address {id: String, location: u16, direct: bool },
    JumpAddress { id: String, location: u16 },
    Identifier(String),
    StringLiteral(String),
    Meta(MetaType),
    Error(String),
}

#[derive(PartialEq, Debug)]
enum MetaType {
    Element {id: String, value: u8}
}

impl Operand {
    pub fn new_number(id: &str, value: u16) -> Self {
        Self::Number {
            id: id.to_string(),
            value: value
        }
    }

    pub fn from_number(value: u16) -> Self {
        Self::Number {
            id: String::new(),
            value: value
        }
    }

    pub fn new_register(name: &str) -> Self {
        if Register::is_register_name(name) {
            Self::RegisterOp(Register::from_name(name))
        } else {
            Self::Error(format!("Invalid Register Name: {}", name))
        }
    }

    pub fn from_address(address: &str, direct: bool) -> Self {
        if let Some(number) = Self::parse_number(address) {
            Self::Address {
                id: String::new(),
                location: number as u16,
                direct: direct,
            }
        } else {
            Self::Error(format!("Invalid Address: {}", address))
        }
    }

    pub fn new_address(id: &str, address: &str, direct: bool) -> Self {
        if let Some(number) = Self::parse_number(address) {
            Self::Address {
                id: id.to_string(),
                location: number as u16,
                direct: direct,
            }
        } else {
            Self::Error(format!("Invalid Address for {}: {}", id, address))
        }
    }

    pub fn new_jump(label: &str) -> Self {
        Self::JumpAddress {
            id: label.to_string(),
            location: 0x8000, // Placeholder
        }
    }

    pub fn new_identifier(id: &str) -> Self {
        Self::Identifier(id.to_string())
    }

    pub fn new_error(message: &str) -> Self {
        Self::Error(message.to_string())
    }

    pub fn parse_number(slice: &str) -> Option<usize> {
        if Self::is_numeric(slice) {
            return None
        }

        Some(match &slice[0..=1] {
                    "0X" => usize::from_str_radix(&slice[2..], 16).unwrap(),
                    "0O" => usize::from_str_radix(&slice[2..], 8).unwrap(),
                    "0B" => usize::from_str_radix(&slice[2..], 2).unwrap(),
                    _ => usize::from_str_radix(&slice, 10).unwrap(),
                })
    }

    pub fn is_numeric(slice: &str) -> bool {
        if let Some(rest) = slice.strip_prefix("0X") {
            usize::from_str_radix(rest, 16).is_ok()
        } else if let Some(rest) = slice.strip_prefix("0O") {
            usize::from_str_radix(rest, 8).is_ok()
        } else if let Some(rest) = slice.strip_prefix("0B") {
            usize::from_str_radix(rest, 2).is_ok()
        } else {
            usize::from_str_radix(slice, 10).is_ok()
        }
    }
}

// Notes to Self
// - Identifiers in code never represent a static value, but rather a static address. The identifier 
//      Operand only represents unmarked identifiers like mnemonics, variable declarations, etc.
//      (In the case of variables, they may start as Identifier but become Address.)
// - Transform branch blocks into a higher-level node. Encode the stuff inside the block, count then 
//      bytes and then replace the placeholder with that count.
// - Error should contain diagnostic information, but that's probably tied to how it's created
// - String literals will be kept within Macro nodes until expanded, so they're probably not needed
// - Zero Page status can be determined when needed (Does this affect mode checks?)
// - Named and Unnamed elements will be treated the same
