use crate::chiiko::cpu::{ Cpu, 
    register::Register::{ProgramCounterHigh, ProgramCounterLow}, 
};

pub trait ProgramCounterOperations {
    fn program_counter(&self) -> u16;
    fn advance_pc(&mut self) -> Result<(), &'static str>;
    fn set_pc(&mut self, value: u16) -> Result<(), &'static str>;
    fn add_pc(&mut self, value: u16) -> Result<(), &'static str>;
    fn sub_pc(&mut self, value: u16) -> Result<(), &'static str>;
}

impl ProgramCounterOperations for Cpu {
    fn advance_pc(&mut self) -> Result<(), &'static str> {
        self.add_pc(1)
    }

    fn add_pc(&mut self, value: u16) -> Result<(), &'static str> {
        let (new_pc, overflow) = self.program_counter().overflowing_add(value);

        if overflow { return Err("Program Counter overflowed") }

        self.set_pc(new_pc)
    }

    fn sub_pc(&mut self, value: u16) -> Result<(), &'static str> {
        let (new_pc, underflow) = self.program_counter().overflowing_sub(value);

        if underflow { return Err("Program Counter underflowed") }

        self.set_pc(new_pc)
    }

    fn set_pc(&mut self, value: u16) -> Result<(), &'static str> {
        self.load_restricted(ProgramCounterHigh, (value >> 8) as u8)?;
        self.load_restricted(ProgramCounterLow, value as u8)?;

        Ok(())
    }

    fn program_counter(&self) -> u16 {
        u16::from_be_bytes(
            [self.read_register(ProgramCounterHigh), self.read_register(ProgramCounterLow)]
        )
    }
}