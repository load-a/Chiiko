use crate::emulator::components::{chip::Chip, chip::ChipError, ram::Ram, rom::Rom, bus::BusError};

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
        Self { ram: ram, rom: rom }
    }
}

impl Chip for Bus {
    fn read(&self, address: u16) -> Result<u8, ChipError> {
        match address {
            0x0000..=0x1FFF => self.ram.read(address),
            0x8000..=0xFFFF => self.rom.read(address),
            _ => Err(BusError::UnmappedRead(address))?,
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), ChipError> {
        match address {
            0x0000..=0x1FFF => self.ram.write(address, value),
            0x8000..=0xFFFF => Err(BusError::UnmappedWrite(address))?,
            _ => Err(BusError::UnmappedWrite(address))?,
        }
    }

    fn tick(&mut self) -> Result<(), ChipError> {
        self.ram.tick()?;
        self.rom.tick()?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), ChipError> {
        self.ram.reset()?;
        self.rom.reset()?;
        Ok(())
    }
}
