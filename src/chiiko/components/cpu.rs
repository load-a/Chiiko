use crate::chiiko::components::{
    chip::Chip, bus::Bus, cpu_operand::CpuOperand::*, instruction::Instruction, cpu_operand::CpuOperand,
};
use crate::operation::Operation;

const RESET_VECTOR_ADDRESS: u16 = 0xFFFE; // The last two bytes of ROM (big endian)
const NO_OPERAND: u8 = 0;
const OPERAND_ERROR: u8 = 0xF;
const STACK_ADDRESS: u16 = 0x1FFF;

pub struct Cpu {
    pub accumulator: u8,
    pub b_register: u8,
    pub c_register: u8,
    pub h_register: u8,
    pub l_register: u8,
    pub i_register: u8,
    pub j_register: u8,
    pub program_counter: u16,
    stack_pointer: u16,
    pub status : u8,
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
            stack_pointer: STACK_ADDRESS,
            status : 0,
            cycle_count: 0,
            bus: bus,
            instruction: Instruction::default(),
        };

        cpu.program_counter = cpu.fetch_reset_vector();

        cpu
    }

    pub fn find(&self, source: CpuOperand) -> Result<u8, &'static str> {
        match source {
            Value(value) => Ok(value),
            Register(register_code) => self.read_register(register_code),
            IndirectRegister(register_code) => Ok(
            self.read(self.register_pointer(register_code).unwrap())
            ),
            ZeroPageAddress(address) => Ok(self.read(address as u16)),
            IndirectZeroPageAddress(address) => Ok(self.read(self.read(address as u16) as u16)),
            MemoryAddress(address) | JumpAddress(address) => Ok(self.read(address)),
            IndirectMemoryAddress(address) => Ok(self.read(self.read(address) as u16)),
            None => Ok(0),
            Error => Err("Invalid source"),
        }
    }

    pub fn send(&mut self, destination: CpuOperand, value: u8) -> Result<(), &'static str> {
        match destination {
            Register(register_code) => self.write_register(register_code, value),
            IndirectRegister(register_code) => self.write(
            self.read_register(register_code).unwrap() as u16, value
            ),
            ZeroPageAddress(address) => self.write(address as u16, value),
            IndirectZeroPageAddress(address) => self.write(
            self.read(address as u16) as u16, value
            ),
            MemoryAddress(address) | JumpAddress(address) => self.write(address, value),
            IndirectMemoryAddress(address) => self.write(self.read(address) as u16, value),
            Error | None | Value(_) => Err("Invalid destination"),
        }
    }

    pub fn resolve_address(&self, destination: &CpuOperand) -> Result<u16, &'static str> {
        match destination {
            Register(register_code) => match register_code {
                9..=11 => self.read_register_pair(*register_code),
                _ => Err("Direct Register does not resolve to address"),
            },
            IndirectRegister(register_code) => Ok(self.read_register(*register_code).unwrap() as u16),
            ZeroPageAddress(address) => Ok(*address as u16),
            IndirectZeroPageAddress(address) => Ok(self.read(*address as u16) as u16),
            MemoryAddress(address) => Ok(*address),
            IndirectMemoryAddress(address) => Ok(self.read(*address) as u16),
            Error | None | Value(_) | JumpAddress(_) => Err("Invalid destination"),
        }
    }

    // Returns register values as an Address
    pub fn register_pointer(&self, register_code: u8) -> Result<u16, &'static str> {
        match register_code {
            0..=6 => Ok(self.read_register(register_code).unwrap() as u16),
            9..=11 => self.read_register_pair(register_code),
            _ => Err("Cannot find register: Bad Register Code")
        }
    }

    // Returns Register Values
    pub fn read_register(&self, register_code: u8) -> Result<u8, &'static str> {
        match register_code {
            0 => Ok(self.accumulator),
            1 => Ok(self.b_register),
            2 => Ok(self.c_register),
            3 => Ok(self.h_register),
            4 => Ok(self.l_register),
            5 => Ok(self.i_register),
            6 => Ok(self.j_register),
            9..=11 => Err("Cannot read Register Pair as direct register"),
            _ => Err("Read to invalid Register Code"),
        }
    }

    pub fn write_register(&mut self, register_code: u8, value: u8) -> Result<(), &'static str> {
        match register_code {
            0 => self.accumulator = value,
            1 => self.b_register = value,
            2 => self.c_register = value,
            3 => self.h_register = value,
            4 => self.l_register = value,
            5 => self.i_register = value,
            6 => self.j_register = value,
            _ => return Err("Write to invalid Register Code"),
        }

        Ok(())
    }

    // Returns Register Pair Literal
    pub fn read_register_pair(&self, register_code: u8) -> Result<u16, &'static str> {
        match register_code {
            9 => Ok(u16::from_be_bytes([self.b_register, self.c_register])),
            10 => Ok(u16::from_be_bytes([self.h_register, self.l_register])),
            11 => Ok(u16::from_be_bytes([self.i_register, self.j_register])),
            _ => Err("Invalid Register Pair code")
        }
    }

    pub fn write_register_pair(&mut self, code: u8, value: u16) -> Result<(), &'static str> {
        let bytes = value.to_be_bytes();
        
        match code {
            9 => {
                self.write_register(1, bytes[0])?;
                self.write_register(2, bytes[1])?;
            },
            10 => {
                self.write_register(3, bytes[0])?;
                self.write_register(4, bytes[1])?;
            },
            11 => {
                self.write_register(5, bytes[0])?;
                self.write_register(6, bytes[1])?;
            },
            _ => return Err("Invalid Register Pair code")
        }

        Ok(())
    }

    pub fn fetch_reset_vector(&mut self) -> u16 {
        let high = self.bus.read(RESET_VECTOR_ADDRESS);
        let low = self.bus.read(RESET_VECTOR_ADDRESS + 1);
        u16::from_be_bytes([high, low])
    }

    pub fn fetch_instruction(&mut self) -> Result<(), &'static str> {
        let operation = self.fetch_operation();
        let mode = self.fetch_grammar(&operation);
        let [left, right] = [self.fetch_operand(mode >> 4), self.fetch_operand(mode & 0x0F)];

        self.instruction = Instruction::new(operation, mode, left, right);

        Ok(())
    }

    fn fetch_operation(&mut self) -> Operation {
        let byte = self.fetch_byte();
        Operation::from_byte(byte)
    }

    fn fetch_grammar(&mut self, operation: &Operation) -> u8 {
        if operation.has_default_mode() {
            operation.default_mode
        } else {
            self.fetch_byte()
        }
    }

    fn fetch_operand(&mut self, mode: u8) -> CpuOperand {
        if mode == NO_OPERAND {
            return CpuOperand::None
        } else if mode == OPERAND_ERROR {
            return CpuOperand::Error
        }

        // fetches 0-2 bytes depending on the mode
        let value: u16 = match mode {
            1..=5 => self.fetch_byte() as u16,
            6..=8 => u16::from_be_bytes([self.fetch_byte(), self.fetch_byte()]),
            _ => 0xFFFF // Fetch no bytes
        };

        match mode {
            1 => CpuOperand::Value(value as u8),
            2 => CpuOperand::Register(value as u8),
            3 => CpuOperand::IndirectRegister(value as u8),
            4 => CpuOperand::ZeroPageAddress(value as u8),
            5 => CpuOperand::IndirectZeroPageAddress(value as u8),
            6 => CpuOperand::MemoryAddress(value),
            7 => CpuOperand::IndirectMemoryAddress(value),
            8 => CpuOperand::JumpAddress(value),
            9 => CpuOperand::Register(0),
            10 => CpuOperand::Value(1),
            11 => CpuOperand::Value(255),
            _ => CpuOperand::Error,
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.read(self.program_counter);
        self.increment_pc();
        byte
    }

    pub fn increment_pc(&mut self) {
        let (result, end) = self.program_counter.overflowing_add(1);

        if end {
            panic!("End of ROM")
        } else {
            self.program_counter = result
        }
    }

    pub fn set_pc(&mut self, address: u16) {
        self.program_counter = address;
    }

    pub fn relative_jump(&mut self, offset: u8) {
        self.program_counter = self.program_counter.wrapping_add(offset as u16);
    }

    pub fn pop(&mut self) -> Result<u8, &'static str> {
        // self.warn_stack_interaction()?;

        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        Ok(self.read(self.stack_pointer))
    }

    pub fn push(&mut self, value: u8) -> Result<(), &'static str> {
        // self.warn_stack_interaction()?;

        self.write(self.stack_pointer, value)?;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        Ok(())
    }

    pub fn clear_flags(&mut self) {
        self.status = 0;
    }

    pub fn set_zero(&mut self) {
        self.status |= 0b0000_0001;
    }

    pub fn set_negative(&mut self) {
        self.status |= 0b0000_0010;
    }

    pub fn set_zero_or_negative(&mut self, result: u8) {
        if result == 0 { self.set_zero() }
        if result & 0x80 != 0 { self.set_negative() }
    }

    pub fn set_carry(&mut self) {
        self.status |= 0b0000_0100;
    }

    pub fn set_interrupt(&mut self) {
        self.status |= 0b1000_0000;
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
        self.stack_pointer = STACK_ADDRESS;
        self.status = 0;
        self.cycle_count = 0;
        Ok(())
    }
}
