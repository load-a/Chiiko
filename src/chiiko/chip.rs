pub trait Chip {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8) -> Result<(), &'static str>;
    fn tick(&mut self) -> Result<(), &'static str> {} // Does nothing by default
    fn import(&mut self, start_address: u16, data: &[u8]) -> Result<(), &'static str>;
    fn export(&self) -> Vec<u8>;
    fn reset(&mut self) -> Result<(), &'static str>;
}