use crate::emulator::components::{chip::Chip, memory_exchange::MemoryExchange};
use crate::emulator::EmulatorError;

const RAM_SIZE: usize = 0x2000;
const BASE_ADDRESS: u16 = 0x0000;

pub struct Ram {
    memory: [u8; RAM_SIZE],
    base_address: u16,
}

impl Default for Ram {
    fn default() -> Self {
        Self { 
            memory: [0; RAM_SIZE], 
            base_address: BASE_ADDRESS, 
        }
    }
}

impl Ram {
    pub fn new(memory: &[u8], base_address: u16) -> Self {
        let mut ram = Self::default();
        let _ = ram.set_base_address(base_address);
        let _ = ram.import(0, memory);

        ram
    }

    fn set_base_address(&mut self, base_address: u16) -> Result<(), EmulatorError> {
        self.base_address = base_address;
        Ok(())
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
    fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        self.offset(address)
        .map(|index| self.memory[index])
        .ok_or_else(|| EmulatorError::InvalidRead(format!("RAM Address <{}>", address)))
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulatorError> {
        if let Some(index) = self.offset(address) {
            self.memory[index] = value;
            Ok(())
        } else {
            Err(EmulatorError::InvalidWrite("Out of Bounds".to_string()))
        }
    }

    fn tick(&mut self) -> Result<(), EmulatorError> {
        Ok(()) // RAM is passive
    }

    fn reset(&mut self) -> Result<(), EmulatorError> {
        self.memory = [0; RAM_SIZE];
        Ok(())
    }
}

impl MemoryExchange for Ram {
    fn import(&mut self, start_address: u16, data: &[u8]) -> Result<(), EmulatorError> {
        let start = start_address as usize;
        let size = data.len();
        let end = size + start;

        if end > RAM_SIZE {
            return Err(EmulatorError::ImportOverload(format!("RAM -> {} / {}", size, RAM_SIZE)))
        }

        self.memory[start..end].copy_from_slice(data);
        Ok(())
    }

    fn export(&self) -> Vec<u8> {
        self.memory.to_vec()
    }
}
