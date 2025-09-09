use crate::numeral_parser::numeral_parser;
use crate::operand::OperandError;
use crate::register::Register;

#[derive(PartialEq, Clone, Debug)]
pub enum Operand {
    NoOperand,
    Number(u16),
    RegisterOp {
        register: Register,
        direct: bool,
    },
    Address {
        id: Option<String>,
        location: Option<u16>,
        direct: bool,
    },
    JumpAddress {
        id: Option<String>,
        location: Option<u16>,
    },
}

impl Operand {
    pub fn register_from_name(name: &str, direct: bool) -> Result<Self, OperandError> {
        if Register::is_register_name(&name) {
            Ok(Self::RegisterOp {
                register: Register::from_name(name).unwrap(),
                direct: direct,
            })
        } else {
            Err(OperandError::InvalidRegister(name.to_string()))
        }
    }

    // Only use after identifiers have been validated
    pub fn address_from_str(address: &str, direct: bool) -> Self {
        if let Some(number) = numeral_parser::parse_str(address) {
            Self::Address {
                id: None,
                location: Some(number as u16),
                direct: direct,
            }
        } else {
            Self::Address {
                id: Some(address.to_string()),
                location: None,
                direct: direct,
            }
        }
    }

    pub fn value(&self) -> Result<u16, OperandError> {
        match self {
            Self::NoOperand => Ok(0),
            Self::Number(value) => Ok(*value),
            Self::RegisterOp { register, .. } => Ok(register.code as u16),
            Self::Address { location, .. } | Self::JumpAddress { location, .. } => {
                if location.is_some() {
                    Ok(location.unwrap())
                } else {
                    Err(OperandError::CannotExtractValue(format!("{:?}", self)))
                }
            }
            _ => Err(OperandError::CannotExtractValue(format!("{:?}", self))),
        }
    }
}
