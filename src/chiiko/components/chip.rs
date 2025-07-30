pub trait Chip {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8) -> Result<(), &'static str>;
    fn tick(&mut self) -> Result<(), &'static str>;
    fn reset(&mut self) -> Result<(), &'static str>;
}