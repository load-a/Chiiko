use crate::chiiko::cpu::{Cpu, register::Register::StackPointer};

const HIGH_BYTE: u8 = 1;

pub trait StackPointerOperations {
    fn stack_pointer(&self) -> u16;
    fn increment_sp(&mut self) -> Result<(), &'static str>;
    fn decrement_sp(&mut self) -> Result<(), &'static str>;
}

impl StackPointerOperations for Cpu {
    fn stack_pointer(&self) -> u16 {
        u16::from_be_bytes([HIGH_BYTE, self.read_register(StackPointer)])
    }

    fn increment_sp(&mut self) -> Result<(), &'static str> {
        let low = self.read_register(StackPointer);
        let (new_sp, overflow) = low.overflowing_add(1);

        if overflow { return Err("Stack Overflow Error") }

        self.load_restricted(StackPointer, new_sp)
    }

    fn decrement_sp(&mut self) -> Result<(), &'static str> {
        let low = self.read_register(StackPointer);
        let (new_sp, overflow) = low.overflowing_sub(1);

        if overflow { return Err("Stack Underflow Error") }

        self.load_restricted(StackPointer, new_sp)
    }
}