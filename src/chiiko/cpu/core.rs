use crate::chiiko::cpu::register::Register;

use Register::*;

const MEMORY_SIZE: u16 = 0xFFFF;
const RESTRICTED_REGISTERS: usize = 7;
const REGISTER_LIMIT: u16 = 10;
const STACK_LOCATION: std::ops::Range<u16> = 0x100..0x200;

#[derive(Debug, PartialEq)]
pub struct Cpu {
    memory: [u8; MEMORY_SIZE as usize],
}

impl Cpu {
    pub fn new() -> Self {
        Self { memory: [0; MEMORY_SIZE as usize] }
    }

    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE as usize];
    }

    pub fn read(&self, address: u16) -> u8 {
        if address > MEMORY_SIZE { panic!("Memory Read out of bounds") }
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), &'static str> {
        if address <= REGISTER_LIMIT || STACK_LOCATION.contains(&address) {
            return Err("Cannot write to restricted address");
        } else if address > MEMORY_SIZE {
            return Err("Memory Write out of bounds");
        }

        self.memory[address as usize] = value;
        Ok(())
    }

    pub fn read_register(&self, register: Register) -> u8 {
        let index = Self::register_index(register);
        self.memory[index]
    }

    pub fn load_register(&mut self, register: Register, value: u8) -> Result<(), &'static str> {
        let index = Self::register_index(register);

        if index >= RESTRICTED_REGISTERS { 
            return Err("Restricted register access");
        }

        self.memory[index] = value;
        Ok(())
    }

    // For setting the value of Stack elements and the Restricted Registers
    pub fn load_restricted(&mut self, register: Register, value: u8) -> Result<(), &'static str> {
        let index = Self::register_index(register);
        self.memory[index] = value;
        Ok(())
    }

    fn register_index(register: Register) -> usize {
        match register {
            Accumulator => 0,
            BRegister => 1,
            CRegister => 2,
            HRegister => 3,
            LRegister => 4,
            IRegister => 5,
            JRegister => 6,
            Reserved => 7,
            ProgramCounter => 8,
            StackPointer => 9,
            StatusFlags => 10,
        }
    }
}