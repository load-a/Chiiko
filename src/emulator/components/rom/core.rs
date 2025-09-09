use crate::emulator::components::{chip::Chip, memory_exchange::MemoryExchange};
use crate::emulator::EmulatorError;

const ROM_SIZE: usize = 0x8000; // 32 KB
const BASE_ADDRESS: u16 = 0x8000; 

#[derive(Copy, Clone)]
pub struct Rom {
    pub memory: [u8; ROM_SIZE],
    pub base_address: u16,
}

impl Default for Rom {    
    fn default() -> Self {
        let mut memory = [0 as u8; ROM_SIZE];
        let reset_address = memory.len() - 2;
        memory[reset_address] = (BASE_ADDRESS >> 8) as u8;
        memory[reset_address + 1] = BASE_ADDRESS as u8;
        

        Self { 
            memory: memory, 
            base_address: BASE_ADDRESS, 
        }
    }
}

impl Rom {
    pub fn new(memory: &[u8], base_address: u16) -> Result<Self, EmulatorError> {
        let mut rom = Self::default();
        rom.set_base_address(base_address)?;
        rom.import(0, memory)?;
        rom.set_reset_vector()?;

        Ok(rom)
    }

    fn set_base_address(&mut self, base_address: u16) -> Result<(), EmulatorError> {
        self.base_address = base_address;
        Ok(())
    }

    fn set_reset_vector(&mut self) -> Result<(), EmulatorError> {
        // Addresses are read in big-endian
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
    fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        self.offset(address)
            .map(|index| self.memory[index])
            .ok_or_else(|| EmulatorError::InvalidRead(format!("ROM offset {:#04X}", address)))
    }

    fn write(&mut self, _: u16, _: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::InvalidWrite("ROM".to_string()))
    }

    fn tick(&mut self) -> Result<(), EmulatorError> {
        Ok(()) // Rom is passive
    }

    fn reset(&mut self) -> Result<(), EmulatorError> {
        // ROM does not change on reset
        Ok(())
    }
}

impl MemoryExchange for Rom {
    fn import(&mut self, start_address: u16, data: &[u8]) -> Result<(), EmulatorError> {
        let start = start_address as usize;
        let end = data.len() + start;

        if end > ROM_SIZE {
            return Err(EmulatorError::ImportOverload("ROM".to_string()))
        }

        self.memory[start..end].copy_from_slice(data);
        Ok(())
    }

    fn export(&self) -> Vec<u8> {
        self.memory.to_vec()
    }
}
