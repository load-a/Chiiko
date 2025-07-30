use crate::chiiko::components::{chip::Chip, ram::Ram, rom::Rom};

pub struct Bus {
    ram: Ram,
    rom: Rom,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            ram: Ram::new(0),
            rom: Rom::new_with_reset_vector(0x8000),
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

    fn write(&mut self, address: u16, value: u8) -> Result<(), &'static str> {
        match address {
            0x0000..=0x1FFF => self.ram.write(address, value),
            0x8000..=0xFFFF => Err("Cannot write to ROM"),
            _ => Err("Address not mapped")
        }
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        let _ = self.ram.tick()?;
        let _ = self.rom.tick()?;
        Ok(())
    }

    fn reset(&mut self) -> Result<(), &'static str> {
        let _ = self.ram.reset()?;
        let _ = self.rom.reset()?;
        Ok(())
    }
}