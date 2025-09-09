use crate::mode::mode_group::ModeGroup;
use crate::mode::mode_group::ModeGroup::{
    Accumulator, AnyOperand, DirectAddress, Error, High, IndirectAddress, IndirectRegister,
    IndirectZeroPage, JumpAddress, Low, NoOperand, Register, Value, ZeroPage,
};
use crate::mode::ModeError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mode {
    pub keys: &'static [&'static str],
    pub group: ModeGroup,
    pub nibble: u8,
}

impl Mode {
    pub fn from_nibble(nibble: u8) -> Result<Self, ModeError> {
        MODES
            .iter()
            .find(|mode| mode.nibble == (nibble & 15))
            .copied()
            .ok_or_else(|| ModeError::IllegalNibble(nibble))
    }

    pub fn from_key(key: &str) -> Result<Self, ModeError> {
        MODES
            .iter()
            .find(|mode| mode.keys.contains(&key))
            .copied()
            .ok_or_else(|| ModeError::IllegalKey(key.to_string()))
    }

    pub fn from_byte(byte: u8) -> Result<(Mode, Mode), ModeError> {
        let left = Self::from_nibble((byte >> 4) & 0xF)?;
        let right = Self::from_nibble(byte & 0xF)?;

        Ok((left, right))
    }

    pub fn is_source(&self) -> bool {
        !matches!(self.group, NoOperand | JumpAddress | AnyOperand | Error)
    }

    pub fn is_destination(&self) -> bool {
        !matches!(
            self.group,
            NoOperand | Value | JumpAddress | Low | High | AnyOperand | Error
        )
    }

    pub fn is_inferred(&self) -> bool {
        matches!(self.group, Accumulator | Low | High)
    }

    pub fn are_compatible(first: (Mode, Mode), second: (Mode, Mode)) -> bool {
        Self::is_compatible(first.0, second.0) && Self::is_compatible(first.1, second.1)
    }

    pub fn is_compatible(primary: Mode, other: Mode) -> bool {
        other.group == ModeGroup::AnyOperand
            || primary.group == ModeGroup::AnyOperand
            || other.group == primary.group
            || (primary.is_inferred() && other.group == ModeGroup::NoOperand)
            || (other.is_inferred() && primary.group == ModeGroup::NoOperand)
    }
}

static MODES: &[Mode] = &[
    Mode {
        keys: &["_"],
        group: NoOperand,
        nibble: 0x0,
    },
    Mode {
        keys: &["V", "N", "#"],
        group: Value,
        nibble: 0x1,
    },
    Mode {
        keys: &["R"],
        group: Register,
        nibble: 0x2,
    },
    Mode {
        keys: &["IR", "@R"],
        group: IndirectRegister,
        nibble: 0x3,
    },
    Mode {
        keys: &["Z"],
        group: ZeroPage,
        nibble: 0x4,
    },
    Mode {
        keys: &["IZ", "@Z"],
        group: IndirectZeroPage,
        nibble: 0x5,
    },
    Mode {
        keys: &["M"],
        group: DirectAddress,
        nibble: 0x6,
    },
    Mode {
        keys: &["IM", "@M"],
        group: IndirectAddress,
        nibble: 0x7,
    },
    Mode {
        keys: &["J"],
        group: JumpAddress,
        nibble: 0x8,
    },
    Mode {
        keys: &["A"],
        group: Accumulator,
        nibble: 0x9,
    },
    Mode {
        keys: &["L", "1"],
        group: Low,
        nibble: 0xA,
    },
    Mode {
        keys: &["H", "255", "FF"],
        group: High,
        nibble: 0xB,
    },
    Mode {
        keys: &["*"],
        group: AnyOperand,
        nibble: 0xE,
    },
    Mode {
        keys: &["E"],
        group: Error,
        nibble: 0xF,
    },
];
