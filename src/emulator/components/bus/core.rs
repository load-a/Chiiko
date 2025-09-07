use crate::emulator::components::{chip::Chip, ram::Ram, rom::Rom};
use crate::emulator::EmulatorError;

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
    fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        match address {
            0x0000..=0x1FFF => self.ram.read(address),
            0x8000..=0xFFFF => self.rom.read(address),
            _ => Err(EmulatorError::InvalidRead(format!("Address <{}>", address)))
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulatorError> {
        match address {
            0x0000..=0x1FFF => self.ram.write(address, value),
            0x8000..=0xFFFF => Err(EmulatorError::InvalidWrite(format!("ROM <{}>", address))),
            _ => Err(EmulatorError::InvalidWrite(format!("Un-mapped Address <{}>", address))),
        }
    }

    fn tick(&mut self) -> Result<(), EmulatorError> {
        let _ = self.ram.tick()?;
        let _ = self.rom.tick()?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), EmulatorError> {
        let _ = self.ram.reset()?;
        let _ = self.rom.reset()?;
        Ok(())
    }
}
