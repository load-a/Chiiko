use crate::emulator::components::{chip::Chip, cpu::Cpu, cpu::CpuError};
use crate::emulator::EmulatorError;

impl Cpu {
    pub fn pop(&mut self) -> Result<u8, CpuError> {
        self.increment_sp();
        Ok(self.read(self.stack_pointer)?)
    }

    pub fn increment_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    pub fn push(&mut self, value: u8) -> Result<(), CpuError> {
        let pointer = self.stack_pointer; // Prevents Multiple Borrow errors
        self.decrement_sp();
        Ok(self.write(pointer, value)?)
    }

    pub fn decrement_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    #[cfg(test)]
    pub fn peek_previous_stack(&self) -> u8 {
        self.read(self.stack_pointer + 1).unwrap()
    }
}
