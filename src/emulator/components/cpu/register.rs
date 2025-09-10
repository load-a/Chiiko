use crate::emulator::components::cpu::{Cpu, CpuError};
use crate::emulator::EmulatorError;

impl Cpu {
    pub fn read_register(&self, code: u8) -> Result<u8, CpuError> {
        match code {
            0 => Ok(self.accumulator),
            1 => Ok(self.b_register),
            2 => Ok(self.c_register),
            3 => Ok(self.h_register),
            4 => Ok(self.l_register),
            5 => Ok(self.i_register),
            6 => Ok(self.j_register),
            9..=11 => Err(CpuError::InvalidRegisterPair(code)),
            _ => Err(CpuError::InvalidSingleRegister(code)),
        }
    }

    pub fn read_register_pair(&self, code: u8) -> Result<u16, CpuError> {
        match code {
            9 => Ok(u16::from_be_bytes([self.b_register, self.c_register])),
            10 => Ok(u16::from_be_bytes([self.h_register, self.l_register])),
            11 => Ok(u16::from_be_bytes([self.i_register, self.j_register])),
            _ => Err(CpuError::InvalidRegisterPair(code)),
        }
    }

    pub fn write_register(&mut self, code: u8, value: u8) -> Result<(), CpuError> {
        match code {
            0 => self.accumulator = value,
            1 => self.b_register = value,
            2 => self.c_register = value,
            3 => self.h_register = value,
            4 => self.l_register = value,
            5 => self.i_register = value,
            6 => self.j_register = value,
            _ => {
                return Err(CpuError::InvalidRegister(code))
            }
        }

        Ok(())
    }

    pub fn write_register_pair(&mut self, code: u8, value: u16) -> Result<(), CpuError> {
        let [big, small] = value.to_be_bytes();

        match code {
            9 => {
                self.b_register = big;
                self.c_register = small;
            }
            10 => {
                self.h_register = big;
                self.l_register = small;
            }
            11 => {
                self.i_register = big;
                self.j_register = small;
            }
            _ => {
                return Err(CpuError::InvalidSingleRegister(code))
            }
        }

        Ok(())
    }
}
