use crate::emulator::components::chip::ChipError;

pub trait Chip {
    fn read(&self, address: u16) -> Result<u8, ChipError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), ChipError>;
    fn tick(&mut self) -> Result<(), ChipError>;
    fn reset(&mut self) -> Result<(), ChipError>;
}
