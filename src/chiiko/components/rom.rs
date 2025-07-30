use crate::chiiko::components::{chip::Chip, memory_exchange::MemoryExchange};

const ROM_SIZE: usize = 0x8000; // 32 KB

pub struct Rom {
    memory: [u8; ROM_SIZE],
    base_address: u16,
}

impl Rom {
    pub fn new(base_address: u16) -> Self {
        Self { 
            memory: [0; ROM_SIZE], 
            base_address 
        }
    }

    pub fn new_with_reset_vector(base_address: u16) -> Self {
        let mut rom = Self::new(base_address);
        let _ = rom.set_reset_vector();
        rom
    }

    fn set_reset_vector(&mut self) -> Result<(), &'static str> {
        // Addresses are read in little-endian
        let reset_address = ROM_SIZE - 2;
        self.memory[reset_address] = (self.base_address >> 8) as u8;
        self.memory[reset_address + 1] = (self.base_address & 0xFF) as u8;
        Ok(())
    }

    fn offset(&self, address: u16) -> Option<usize> {
        let offset = address.wrapping_sub(self.base_address) as usize;

        if offset < ROM_SIZE {
            Some(offset)
        } else {
            None
        }
    }
}

impl Chip for Rom {
    fn read(&self, address: u16) -> u8 {
        self.offset(address)
            .map(|index| self.memory[index])
            .unwrap_or(0xFF)
    }

    fn write(&mut self, _: u16, _: u8) -> Result<(), &'static str> {
        Err("Cannot write to ROM")
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        Ok(()) // Rom is passive
    }

    fn reset(&mut self) -> Result<(), &'static str> {
        // ROM does not change on reset
        Ok(())
    }
}

impl MemoryExchange for Rom {
    fn import(&mut self, start_address: u16, data: &[u8]) -> Result<(), &'static str> {
        let start = start_address as usize;
        let end = data.len() + start;

        if end > ROM_SIZE {
            return Err("Imported data is too large")
        }

        self.memory[start..end].copy_from_slice(data);
        Ok(())
    }

    fn export(&self) -> Vec<u8> {
        self.memory.to_vec()
    }
}