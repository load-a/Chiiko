use crate::chiiko::components::{chip::Chip, bus::Bus};
use crate::chiiko::opcode::Opcode;

const RESET_VECTOR_ADDRESS: u16 = 0xFFFE; // The last two bytes of ROM (big endian)

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
    cycle_count: u8,
    instruction: Vec<u16>,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Self {
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
            instruction: [].to_vec(),
        }
    }

    fn fetch_reset_vector(&mut self) -> u16 {
        let high = self.bus.read(RESET_VECTOR_ADDRESS);
        let low = self.bus.read(RESET_VECTOR_ADDRESS + 1);
        u16::from_be_bytes([high, low])
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.program_counter);
        self.increment_pc();
        byte
    }

    fn fetch_instruction(&mut self) -> Result<(), &'static str> {
        let opcode = self.fetch_byte();
        let mode = self.fetch_grammar(Opcode::decode(opcode)?);
        let [left, right] = [self.fetch_operand(mode >> 4), self.fetch_operand(mode & 0x0F)];

        self.instruction = [opcode as u16, mode as u16, left.unwrap(), right.unwrap()].to_vec();

        Ok(())
    }

    fn fetch_opcode(&mut self) -> Result<Opcode, &'static str> {
        let byte = self.fetch_byte();
        Opcode::decode(byte)
    }

    fn fetch_grammar(&mut self, opcode: Opcode) -> u8 {
        if opcode.mode {
            self.fetch_byte()
        } else {
            opcode.default_grammar()
        }
    }

    fn fetch_operand(&mut self, nibble: u8) -> Option<u16> {
        match nibble {
            1..=5 => Some(self.fetch_byte() as u16),
            6..=8 => Some(u16::from_be_bytes([self.fetch_byte(), self.fetch_byte()])),
            9 => Some(0),
            _ => None
        }
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

// TODO:
// - Change the instruction array so that it holds the true values for its Opcode and Operands at least
// - Get the Operand Struct from /cpu to work here
