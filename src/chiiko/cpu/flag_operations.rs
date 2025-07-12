use crate::chiiko::cpu::{ Cpu, register::Register::StatusFlags };

pub trait FlagOperations {
    fn reset_status(&mut self) -> Result<(), &'static str>;
    fn set_zero(&mut self) -> Result<(), &'static str>;
    fn set_negative(&mut self) -> Result<(), &'static str>;
    fn set_overflow(&mut self) -> Result<(), &'static str>;
}

impl FlagOperations for Cpu {
    fn reset_status(&mut self) -> Result<(), &'static str> {
        self.load_restricted(StatusFlags, 0)
    }

    // Result of operation is zero
    fn set_zero(&mut self) -> Result<(), &'static str> {
        self.add_to_status(1)
    }

    // Result of operation went below zero
    fn set_negative(&mut self) -> Result<(), &'static str> {
        self.add_to_status(2)
    }

    // Result of operation went above 255
    fn set_overflow(&mut self) -> Result<(), &'static str> {
        self.add_to_status(4)
    }
}