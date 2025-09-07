use crate::emulator::EmulatorError;

pub trait Chip {
    fn read(&self, address: u16) -> Result<u8, EmulatorError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulatorError>;
    fn tick(&mut self) -> Result<(), EmulatorError>;
    fn reset(&mut self) -> Result<(), EmulatorError>;
}
