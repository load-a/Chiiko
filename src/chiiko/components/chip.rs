pub trait Chip {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8) -> Result<(), &str>;
    fn tick(&mut self) -> Result<(), &str>;
    fn reset(&mut self) -> Result<(), &str>;
}
