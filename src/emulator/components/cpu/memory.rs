use crate::emulator::components::{chip::Chip, cpu::Cpu};
use crate::emulator::EmulatorError;

use crate::operand::Operand;

impl Cpu {
    pub fn register_pointer(&self, register_code: u8) -> Result<u16, EmulatorError> {
        match register_code {
            0..=6 => Ok(self.read_register(register_code).unwrap() as u16),
            9..=11 => self.read_register_pair(register_code),
            _ => Err(EmulatorError::InvalidRead(format!(
                "Register Code <{}>",
                register_code
            ))),
        }
    }

    pub fn find(&self, source: &Operand) -> Result<u8, EmulatorError> {
        match source {
            Operand::NoOperand => Ok(0),
            Operand::Number(value) => Ok(*value as u8),
            Operand::RegisterOp { register, direct } => {
                if *direct {
                    self.read_register(register.code)
                } else {
                    self.read(self.register_pointer(register.code)?)
                }
            }
            Operand::Address {
                location, direct, ..
            } => {
                if *direct {
                    Ok(self.read(location.unwrap())?)
                } else {
                    Ok(self.read(self.read(location.unwrap())? as u16)?)
                }
            }
            _ => Err(EmulatorError::CannotFind(format!("{:?}", source))),
        }
    }

    pub fn send(&mut self, destination: &Operand, value: u8) -> Result<(), EmulatorError> {
        match destination {
            Operand::RegisterOp { register, direct } => {
                if *direct {
                    self.write_register(register.code, value)?
                } else {
                    self.write(self.read_register(register.code).unwrap() as u16, value)?
                }
            }
            Operand::Address {
                location, direct, ..
            } => {
                if *direct {
                    self.write(location.unwrap(), value)?
                } else {
                    self.write(self.read(location.unwrap())? as u16, value)?
                }
            }
            _ => {
                return Err(EmulatorError::CannotSend(format!(
                    "Invalid destination <{:?}>",
                    destination
                )))
            }
        }

        Ok(())
    }

    pub fn resolve_address(&self, destination: &Operand) -> Result<u16, EmulatorError> {
        match destination {
            Operand::RegisterOp { register, direct } => {
                if *direct {
                    match register.code {
                        0..=6 => Err(EmulatorError::CannotResolve(format!(
                            "Register <{}>",
                            register.code
                        ))),
                        9..=11 => self.read_register_pair(register.code),
                        _ => Err(EmulatorError::CannotResolve(format!(
                            "Indirect Register <{}>",
                            register.code
                        ))),
                    }
                } else {
                    Ok(self.read_register(register.code).unwrap() as u16)
                }
            }
            Operand::Address {
                location, direct, ..
            } => {
                if *direct {
                    Ok(location.unwrap())
                } else {
                    Ok(self.read(location.unwrap())? as u16)
                }
            }
            _ => Err(EmulatorError::InvalidDestination(format!(
                "{:?}",
                destination
            ))),
        }
    }
}
