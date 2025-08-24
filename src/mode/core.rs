use crate::mode::mode_group::ModeGroup;
use crate::mode::mode_group::ModeGroup::{NoOperand, Value, Register, IndirectRegister, ZeroPage,
    IndirectZeroPage, DirectAddress, IndirectAddress, JumpAddress, Accumulator,
    Low, High, Error,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Mode {
    pub keys: &'static [&'static str],
    pub group: ModeGroup,
    pub nibble: u8,
}

impl Mode {
    pub fn from_byte(byte: u8) -> (Mode, Mode) {
        let left = Self::from_nibble(byte >> 4);
        let right = Self::from_nibble(byte & 0xF);

        (left, right)
    }

    pub fn from_nibble(nibble: u8) -> Self {
        MODES
            .iter()
            .find(|mode| mode.nibble == (nibble & 15))
            .expect("Invalid Opcode")
            .clone()
    }

    pub fn from_key(key: &str) -> Self {
        MODES
            .iter()
            .find(|mode| mode.keys.contains(&key))
            .unwrap_or_else(|| panic!("Invalid Mode Key: {}", key))
            .clone()
    }

    pub fn into_nibble(&self) -> u8 {
        MODES
            .iter()
            .find(|mode| mode.group == self.group)
            .map(|mode| mode.nibble)
            .unwrap_or_else(|| panic!("Mode has invalid Group: {:?}", self))
    }

    pub fn is_source(&self) -> bool {
        !matches!(self.group, NoOperand | JumpAddress | Error)
    }

    pub fn is_destination(&self) -> bool {
        !matches!(self.group, NoOperand | JumpAddress | Low | High | Error)
    }
}

static MODES: &[Mode] = &[
    Mode { keys: &["_"], group: NoOperand, nibble: 0x0 },
    Mode { keys: &["V", "#"], group: Value, nibble: 0x1 },
    Mode { keys: &["R"], group: Register, nibble: 0x2 },
    Mode { keys: &["IR", "@R"], group: IndirectRegister, nibble: 0x3 },
    Mode { keys: &["Z"], group: ZeroPage, nibble: 0x4 },
    Mode { keys: &["IZ", "@Z"], group: IndirectZeroPage, nibble: 0x5 },
    Mode { keys: &["M"], group: DirectAddress, nibble: 0x6 },
    Mode { keys: &["IM", "@M"], group: IndirectAddress, nibble: 0x7 },
    Mode { keys: &["J"], group: JumpAddress, nibble: 0x8 },
    Mode { keys: &["A"], group: Accumulator, nibble: 0x9 },
    Mode { keys: &["L", "1"], group: Low, nibble: 0xA },
    Mode { keys: &["H", "255", "FF"], group: High, nibble: 0xB },
    Mode { keys: &["E"], group: Error, nibble: 0xF },
];
