use crate::register::error::RegisterError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Register {
    pub id: &'static str,
    pub code: u8,
}

impl Register {
    pub fn from_name(name: &str) -> Result<Self, RegisterError> {
        REGISTERS
            .iter()
            .find(|reg| name.trim().eq_ignore_ascii_case(reg.id))
            .copied()
            .ok_or_else(|| RegisterError::IllegalName(name.to_string()))
    }

    pub fn from_code(code: u8) -> Result<Self, RegisterError> {
        REGISTERS
            .iter()
            .find(|reg| code == reg.code)
            .copied()
            .ok_or_else(|| RegisterError::IllegalCode(code))
    }

    pub fn is_register_pair(&self) -> bool {
        PAIR_NAMES.contains(&self.id)
    }

    pub fn is_register_name(name: &str) -> bool {
        Self::from_name(name).is_ok()
    }
}

static PAIR_NAMES: &[&str] = &[
    "BC", "HL", "IJ",
];

static REGISTERS: &[Register] = &[
    Register { id: "A", code: 0x00 },
    Register { id: "B", code: 0x01 },
    Register { id: "C", code: 0x02 },
    Register { id: "H", code: 0x03 },
    Register { id: "L", code: 0x04 },
    Register { id: "I", code: 0x05 },
    Register { id: "J", code: 0x06 },
    Register { id: "BC", code: 0x9 },
    Register { id: "HL", code: 0xA },
    Register { id: "IJ", code: 0xB },
];
