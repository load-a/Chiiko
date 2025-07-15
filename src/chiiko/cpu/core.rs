use crate::chiiko::cpu::{
    register::Register, register::Register::*
};

const STACK_RANGE: std::ops::Range<u16> = 0x100..0x200;
const RESTRICTED_REGISTER_RANGE: std::ops::Range<u16> = 7..10;
const RESTRICTED_REGISTERS_START: usize = 7;
const ROM_START: usize = 0xC100;
const MEMORY_SIZE: usize = 0xFFFF;

#[derive(Debug, PartialEq)]
pub struct Cpu {
    memory: [u8; MEMORY_SIZE],
}

impl Cpu {
    pub fn new() -> Self {
        Self { memory: [0; MEMORY_SIZE] }
    }

    pub fn reset(&mut self) {
        self.memory = [0; MEMORY_SIZE];
    }

    pub fn read(&self, address: u16) -> u8 {
        if address as usize > MEMORY_SIZE { panic!("Memory Read out of bounds") }
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), &'static str> {
        if RESTRICTED_REGISTER_RANGE.contains(&address) || STACK_RANGE.contains(&address) {
            return Err("Cannot write to restricted address");
        } else if address as usize > MEMORY_SIZE {
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

        if index >= RESTRICTED_REGISTERS_START { 
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

    pub fn add_to_status(&mut self, value: u8) -> Result<(), &'static str> {
        self.memory[Self::register_index(Register::StatusFlags)] |= value;
        Ok(())
    }

    pub fn advance_program_counter(&mut self) -> Result<(), &'static str> {
        self.memory[Self::register_index(ProgramCounter)] = self.read_register(ProgramCounter).saturating_add(1);
        Ok(())
    }

    pub fn load_rom(&mut self, source: Vec<u8>) -> Result<(), &'static str> {
        let end = ROM_START + source.len();

        self.memory[ROM_START..end].copy_from_slice(&source);
        Ok(())
    }

    pub fn rom(&self) -> Vec<u8> {
        self.memory[ROM_START as usize..MEMORY_SIZE].to_vec()
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