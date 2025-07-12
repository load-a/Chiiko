use crate::chiiko::cpu::{
    Cpu, 
    operand::Operand,
    flag_operations::FlagOperations, 
    extended_read_write::ExtendedReadWrite,
};

pub trait Alu {
    fn add(&mut self, source: Operand, destination: Operand) -> Result<(), &'static str>;
    fn subtract(&mut self, source: Operand, destination: Operand) -> Result<(), &'static str>;
}

impl Alu for Cpu {
    fn add(&mut self, source: Operand, destination: Operand) -> Result<(), &'static str> {
        let left = self.read_operand(source);
        let right = self.read_operand(destination);
        let (result, overflow) = left.overflowing_add(right);

        if overflow { self.set_overflow()? } 
        if result == 0 { self.set_zero()? }

        self.write_operand(destination, result)?;
        Ok(())
    }

    fn subtract(&mut self, source: Operand, destination: Operand) -> Result<(), &'static str> {
        let left = self.read_operand(source);
        let right = self.read_operand(destination);
        let (result, underflow) = left.overflowing_sub(right);

        if underflow { self.set_negative()? } 
        if result == 0 { self.set_zero()? }

        self.write_operand(destination, result)?;
        Ok(())
    }
}