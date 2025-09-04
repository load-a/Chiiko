use crate::register::Register;
use crate::operand::OperandError;
use crate::numeral_parser::NumeralParser;

#[derive(PartialEq, Clone, Debug)]
pub enum Operand<'a> {
    NoOperand,
    Number(u16),
    RegisterOp { register: Register<'a>, direct: bool },
    Address { id: String, location: u16, direct: bool },
    JumpAddress { id: String, location: u16 },
    Identifier(String),
    Macro(MacroType),
    Element {id: String, address: u16, value: u8},
    Error(String),
}

#[derive(PartialEq, Clone, Debug)]
enum MacroType {
    StringLiteral(String),
    ArrayElement,
}

impl<'a> Operand<'a> {
    pub fn register_from_name(name: &str, direct: bool) -> Self {
        if Register::is_register_name(&name) {
            Self::RegisterOp {
                register: Register::from_name(name).unwrap(),
                direct: direct
            }
        } else {
            Self::Error(format!("Invalid Register Name: {}", name))
        }
    }

    pub fn address_from_str(address: &str, direct: bool) -> Self {
        if let Some(number) = NumeralParser::parse_str(address) {
            Self::Address {
                id: String::new(),
                location: number as u16,
                direct: direct,
            }
        } else {
            Self::Address {
                id: address.to_string(),
                location: 0,
                direct: direct,
            }
        }
    }

    pub fn value(&self) -> Result<u16, OperandError> {
        match self {
            Self::NoOperand => Ok(0),
            Self::Number(value) => Ok(*value),
            Self::RegisterOp { register, .. } => Ok(register.code as u16),
            Self::Address { location, .. } | Self::JumpAddress { location, .. } => Ok(*location),
            _ => Err(OperandError::CannotExtractValue(format!("{:?}", self)))
        }
    }
}
