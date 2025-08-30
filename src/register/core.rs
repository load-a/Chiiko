#[derive(Clone, Debug, PartialEq)]
pub struct Register {
    pub id: &'static str,
    pub code: u8,
}

impl Register {
    pub fn is_register_pair(&self) -> bool {
        PAIR_NAMES.contains(&self.id)
    }

    pub fn from_name(name: &str) -> Self {
        REGISTERS
            .iter()
            .find(|reg| name.trim().to_uppercase() == reg.id)
            .unwrap_or_else(|| panic!("Register Struct Error: Invalid Register Name: {}", name))
            .clone()
    }

    pub fn is_register_name(name: &str) -> bool {
        REGISTERS
            .iter()
            .find(|reg| name.trim().to_uppercase() == reg.id)
            .is_some()
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
