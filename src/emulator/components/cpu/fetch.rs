use crate::emulator::components::{chip::Chip, cpu::Cpu, cpu::CpuError, instruction::Instruction};
use crate::emulator::EmulatorError;
use crate::mode::mode_group::ModeGroup;
use crate::mode::Mode;
use crate::operand::Operand;
use crate::operation::Operation;
use crate::register::Register;

impl Cpu {
    pub fn fetch_instruction(&mut self) -> Result<(), CpuError> {
        let operation = self.fetch_operation()?;
        let mode = self.fetch_grammar(&operation)?;
        let [left, right] = [self.fetch_operand(mode.0)?, self.fetch_operand(mode.1)?];

        self.instruction = Instruction::new(operation, mode, left, right);

        Ok(())
    }

    pub(crate) fn fetch_operation(&mut self) -> Result<Operation, CpuError> {
        let byte = self.fetch_byte()?;
        Ok(Operation::from_byte(byte)?)
    }

    pub(crate) fn fetch_grammar(
        &mut self,
        operation: &Operation,
    ) -> Result<(Mode, Mode), CpuError> {
        if operation.has_default_mode() {
            Ok(Mode::from_byte(operation.default_mode)?)
        } else {
            Ok(Mode::from_byte(self.fetch_byte()?)?)
        }
    }

    pub(crate) fn fetch_operand(&mut self, mode: Mode) -> Result<Operand, CpuError> {
        // fetches 0-2 bytes depending on the mode
        let value: u16 = match mode.nibble {
            0 => 0,
            1..=5 => self.fetch_byte()? as u16,
            6..=8 => u16::from_be_bytes([self.fetch_byte()?, self.fetch_byte()?]),
            9..=0xB => 0, // Accumulator, Low and High get values later
            0xE..=0xF => {
                // Error and AnyOperand modes should never appear in an executable.
                return Err(CpuError::CannotFetch(format!(
                    "Unfetchable Mode >{:?}<", 
                    mode)
                ))?
            }
            _ => {
                return Err(CpuError::CannotFetch(format!(
                    "Invalid Mode Nibble >{:?}<",
                    mode
                )))?
            }
        };

        let operand = match mode.group {
            ModeGroup::NoOperand | ModeGroup::AnyOperand => Operand::NoOperand,
            ModeGroup::Value => Operand::Number(value),
            ModeGroup::Register => Operand::RegisterOp {
                register: Register::from_code(value as u8)?,
                direct: true,
            },
            ModeGroup::IndirectRegister => Operand::RegisterOp {
                register: Register::from_code(value as u8)?,
                direct: false,
            },
            ModeGroup::ZeroPage | ModeGroup::DirectAddress => Operand::Address {
                id: None,
                location: Some(value),
                direct: true,
            },
            ModeGroup::IndirectZeroPage | ModeGroup::IndirectAddress => Operand::Address {
                id: None,
                location: Some(value),
                direct: false,
            },
            ModeGroup::JumpAddress => Operand::JumpAddress {
                id: None,
                location: Some(value),
            },
            ModeGroup::Accumulator => Operand::RegisterOp {
                register: Register::from_name("A")?,
                direct: true,
            },
            ModeGroup::Low => Operand::Number(0x1),
            ModeGroup::High => Operand::Number(0xFF),
            ModeGroup::Error => {
                return Err(CpuError::from(CpuError::CannotFetch(format!(
                    "Error Operand: {:?}",
                    mode
                ))))
            }
        };

        Ok(operand)
    }

    fn fetch_byte(&mut self) -> Result<u8, CpuError> {
        // This has been programmed as such in order to prevent Multiple Borrow errors.
        //  However, this causes, as a side effect, the inability to fetch the last byte of ROM.
        //  The last two bytes of ROM are dedicated to the Reset Vector anyway, so this likely 
        //  won't be an issue, but it's worth remembering.

        let byte = self.bus.read(self.program_counter)?;
        self.increment_pc()?;
        Ok(byte)
    }

    pub fn increment_pc(&mut self) -> Result<(), CpuError> {
        let (result, end) = self.program_counter.overflowing_add(1);

        if end {
            Err(CpuError::EndOfProgram)
        } else {
            self.program_counter = result;
            Ok(())
        }
    }

    pub fn set_pc(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn relative_jump(&mut self, offset: u8) {
        self.program_counter = self.program_counter.wrapping_add(offset as u16);
    }
}
