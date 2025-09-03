use crate::chiiko::components::{chip::Chip, ram::Ram, rom::Rom};

pub struct Bus {
    ram: Ram,
    rom: Rom,
}

impl Bus {
    pub fn default() -> Self {
        Self {
            ram: Ram::default(),
            rom: Rom::default(),
        }
    }

    pub fn new(ram: Ram, rom: Rom) -> Self {
        Self {
            ram: ram,
            rom: rom,
        }
    }
}

impl Chip for Bus {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.read(address),
            0x8000..=0xFFFF => self.rom.read(address),
            _ => 0
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), &str> {
        match address {
            0x0000..=0x1FFF => self.ram.write(address, value),
            0x8000..=0xFFFF => Err("Cannot write to ROM"),
            _ => Err("Write to un-mapped address")
        }
    }

    fn tick(&mut self) -> Result<(), &str> {
        let _ = self.ram.tick()?;
        let _ = self.rom.tick()?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), &str> {
        let _ = self.ram.reset()?;
        let _ = self.rom.reset()?;
        Ok(())
    }
}
