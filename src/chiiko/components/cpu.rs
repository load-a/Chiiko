use crate::chiiko::components::{
    chip::Chip, bus::Bus, operand::Operand, instruction::Instruction,
};
use crate::chiiko::opcode::Opcode;

const RESET_VECTOR_ADDRESS: u16 = 0xFFFE; // The last two bytes of ROM (big endian)
const NO_OPERAND: u8 = 0;
const OPERAND_ERROR: u8 = 0xF;

pub struct Cpu {
    accumulator: u8,
    b_register: u8,
    c_register: u8,
    h_register: u8,
    l_register: u8,
    i_register: u8,
    j_register: u8,
    program_counter: u16,
    stack_pointer: u8,
    flags : u8,
    bus: Bus,
    pub cycle_count: u8,
    pub instruction: Instruction,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        let mut cpu = Self {
            accumulator: 0,
            b_register: 0,
            c_register: 0,
            h_register: 0,
            l_register: 0,
            i_register: 0,
            j_register: 0,
            program_counter: 0,
            stack_pointer: 0,
            flags : 0,
            cycle_count: 0,
            bus: bus,
            instruction: Instruction::default(),
        };

        cpu.program_counter = cpu.fetch_reset_vector();

        cpu
    }

    pub fn fetch_reset_vector(&mut self) -> u16 {
        let high = self.bus.read(RESET_VECTOR_ADDRESS);
        let low = self.bus.read(RESET_VECTOR_ADDRESS + 1);
        u16::from_be_bytes([high, low])
    }

    pub fn fetch_instruction(&mut self) -> Result<(), &'static str> {
        let opcode = self.fetch_opcode();
        let mode = self.fetch_grammar(&opcode); 
        let [left, right] = [self.fetch_operand(mode >> 4), self.fetch_operand(mode & 0x0F)];

        self.instruction = Instruction::new(opcode, mode, left, right);

        Ok(())
    }

    fn fetch_opcode(&mut self) -> Opcode {
        let byte = self.fetch_byte();
        Opcode::decode(byte)
    }

    fn fetch_grammar(&mut self, opcode: &Opcode) -> u8 {
        if opcode.mode {
            self.fetch_byte()
        } else {
            opcode.default_grammar()
        }
    }

    fn fetch_operand(&mut self, mode: u8) -> Operand {
        if mode == NO_OPERAND {
            return Operand::None
        } else if mode == OPERAND_ERROR {
            return Operand::Error
        }

        // fetches 0-2 bytes depending on the mode
        let value: u16 = match mode {
            1..=5 => self.fetch_byte() as u16,
            6..=8 => u16::from_be_bytes([self.fetch_byte(), self.fetch_byte()]),
            _ => 0xFFFF // Fetch no bytes
        };

        match mode {
            1 => Operand::Value(value as u8),
            2 => Operand::Register(value as u8),
            3 => Operand::IndirectRegister(value as u8),
            4 => Operand::ZeroPageAddress(value as u8),
            5 => Operand::IndirectZeroPageAddress(value as u8),
            6 => Operand::MemoryAddress(value),
            7 => Operand::IndirectMemoryAddress(value),
            8 => Operand::JumpAddress(value),
            9 => Operand::Register(0),
            _ => Operand::Error,
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.program_counter);
        self.increment_pc();
        byte
    }

    fn increment_pc(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }
}

impl Chip for Cpu {
    fn read(&self, address: u16) -> u8 {
        self.bus.read(address)
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), &'static str> {
        let _ = self.bus.write(address, value)?;
        Ok(())
    }

    fn tick(&mut self) -> Result<(), &'static str> {
        let _ = self.bus.tick()?;
        self.cycle_count = self.cycle_count.wrapping_add(1);
        Ok(())
    }

    fn reset(&mut self) -> Result<(), &'static str> {
        let _ = self.bus.reset()?;
        self.accumulator = 0;
        self.b_register = 0;
        self.c_register = 0;
        self.h_register = 0;
        self.l_register = 0;
        self.i_register = 0;
        self.j_register = 0;
        self.program_counter = self.fetch_reset_vector();
        self.stack_pointer = 0;
        self.flags = 0;
        self.cycle_count = 0;
        Ok(())
    }
}
