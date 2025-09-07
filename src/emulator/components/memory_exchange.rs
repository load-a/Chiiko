use crate::emulator::EmulatorError;

pub trait MemoryExchange {
    fn import(&mut self, start_address: u16, data: &[u8]) -> Result<(), EmulatorError>;
    fn export(&self) -> Vec<u8>;
}
