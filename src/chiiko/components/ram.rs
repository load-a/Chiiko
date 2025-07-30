use crate::chiiko::components::{chip::Chip, memory_exchange::MemoryExchange};

const RAM_SIZE: usize = 0x2000;

pub struct Ram {
    memory: [u8; RAM_SIZE],
    base_address: u16,
}

impl Ram {
    pub fn new(base_address: u16) -> Self {
        Self { 
            memory: [0; RAM_SIZE], 
            base_address 
        }
    }

    fn offset(&self, address: u16) -> Option<usize> {
        let offset = address.wrapping_sub(self.base_address) as usize;

        if offset < RAM_SIZE {
            Some(offset)
        } else {
            None
        }
    }
}

impl Chip for Ram {
    fn read(&self, address: u16) -> u8 {
        self.offset(address)
            .map(|index| self.memory[index])
            .unwrap_or(0xFF)
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), &'static str> {
        if let Some(index) = self.offset(address) {
            self.memory[index] = value;
            Ok(())
        } else {
            Err("Write out of bounds")
        }
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        Ok(()) // RAM is passive
    }

    fn reset(&mut self) -> Result<(), &'static str> {
        self.memory = [0; RAM_SIZE];
        Ok(())
    }
}

impl MemoryExchange for Ram {
    fn import(&mut self, start_address: u16, data: &[u8]) -> Result<(), &'static str> {
        let start = start_address as usize;
        let end = data.len() + start;

        if end > RAM_SIZE {
            return Err("Imported data is too large")
        }

        self.memory[start..end].copy_from_slice(data);
        Ok(())
    }

    fn export(&self) -> Vec<u8> {
        self.memory.to_vec()
    }
}