use crate::emulator::components::{
    chip::Chip, bus::Bus, instruction::Instruction,
};
use crate::emulator::EmulatorError;
use crate::register::Register;
use crate::operation::Operation;
use crate::mode::Mode;
use crate::operand::Operand;
use crate::mode::mode_group::ModeGroup;
use crate::chiiko_error::ChiikoError;

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

// Categories
// FETCH: Reads a value and moves the program counter
// PEEK:  Reads an address literally
// READ:  Reads a address directly (peeking) or indirectly (pointing)
// WRITE: Changes a value

impl Cpu {
    pub fn new(bus: Bus) -> Result<Self, EmulatorError> {
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
            instruction: Instruction::default(), // FIX
        };

        cpu.program_counter = cpu.peek_reset_vector()?;

        Ok(cpu)
    }

    pub fn find(&self, source: &Operand) -> Result<u8, EmulatorError> {
        match source {
            Operand::Number(value) => Ok(*value as u8),
            Operand::RegisterOp { register, direct } => {
                if *direct {
                    self.read_register(register.code)
                } else {
                    self.read(self.read_indirect_register(register.code)?)
                }
            },
            Operand::Address { location, direct, .. } => {
                if *direct {
                    Ok(self.read(location.unwrap())?)
                } else {
                    Ok(self.read(self.read(location.unwrap())? as u16)?)
                }
            },
            Operand::NoOperand => Ok(0),
            _ => Err(EmulatorError::CannotFind(format!("{:?}", source))),
        }
    }

    pub fn send(&mut self, destination: &Operand, value: u8) -> Result<(), EmulatorError> {
        match destination {
            Operand::RegisterOp { register, direct } => {
                if *direct {
                    self.write_register(register.code, value)?
                } else {
                    self.write(self.read_register(register.code).unwrap() as u16, value)?
                }
            },
            Operand::Address { location, direct, .. } => {
                if *direct {
                    self.write(location.unwrap(), value)?
                } else {
                    self.write(self.read(location.unwrap())? as u16, value)?
                }
            },
            _ => {
                return Err(
                    EmulatorError::CannotSend(format!("Invalid destination <{:?}>", destination))
                )
            }
        }

        Ok(())
    }

    pub fn resolve_address(&self, destination: &Operand) -> Result<u16, EmulatorError> {
        match destination {
            Operand::RegisterOp { register, direct } => {
                if *direct {
                    match register.code {
                        0..=6 => Err(EmulatorError::CannotResolve(format!("Register <{}>", register.code))),
                        9..=11 => self.peek_register_pair(register.code),
                        _ => Err(EmulatorError::CannotResolve(format!("Indirect Register <{}>", register.code))),
                    }
                } else {
                    Ok(self.read_register(register.code).unwrap() as u16)
                }
            }
            Operand::Address { location, direct, .. } => {
                if *direct {
                    Ok(location.unwrap())
                } else {
                    Ok(self.read(location.unwrap())? as u16)
                }
            }
            _ => Err(EmulatorError::InvalidDestination(format!("{:?}", destination))),
        }
    }

    // Returns register values as an Address
    pub fn read_indirect_register(&self, register_code: u8) -> Result<u16, EmulatorError> {
        match register_code {
            0..=6 => Ok(self.read_register(register_code).unwrap() as u16),
            9..=11 => self.peek_register_pair(register_code),
            _ => Err(EmulatorError::InvalidRead(format!("Register Pair Code <{}>", register_code)))
        }
    }

    // Returns Register Values
    pub fn read_register(&self, register_code: u8) -> Result<u8, EmulatorError> {
        match register_code {
            0 => Ok(self.accumulator),
            1 => Ok(self.b_register),
            2 => Ok(self.c_register),
            3 => Ok(self.h_register),
            4 => Ok(self.l_register),
            5 => Ok(self.i_register),
            6 => Ok(self.j_register),
            9..=11 => Err(EmulatorError::InvalidRead(format!("Register Pair as Register"))),
            _ => Err(EmulatorError::InvalidRead(format!("Register Code <{}>", register_code))),
        }
    }

    pub fn write_register(&mut self, register_code: u8, value: u8) -> Result<(), EmulatorError> {
        match register_code {
            0 => self.accumulator = value,
            1 => self.b_register = value,
            2 => self.c_register = value,
            3 => self.h_register = value,
            4 => self.l_register = value,
            5 => self.i_register = value,
            6 => self.j_register = value,
            _ => return Err(EmulatorError::InvalidWrite(format!("Register Code {}", register_code)))
        }

        Ok(())
    }

    // Returns value in register pair
    pub fn peek_register_pair(&self, register_code: u8) -> Result<u16, EmulatorError> {
        match register_code {
            9 => Ok(u16::from_be_bytes([self.b_register, self.c_register])),
            10 => Ok(u16::from_be_bytes([self.h_register, self.l_register])),
            11 => Ok(u16::from_be_bytes([self.i_register, self.j_register])),
            _ => Err(EmulatorError::InvalidRead(format!("Register Pair Code <{}>", register_code)))
        }
    }

    pub fn write_register_pair(&mut self, code: u8, value: u16) -> Result<(), EmulatorError> {
        let [big, small] = value.to_be_bytes();
        
        match code {
            9 => {
                self.b_register = big;
                self.c_register = small;
            },
            10 => {
                self.h_register = big;
                self.l_register = small;
            },
            11 => {
                self.i_register = big;
                self.j_register = small;
            },
            _ => {
                return Err(
                    EmulatorError::InvalidWrite(format!("Register Pair Code <{}>", code))
                )
            }
        }

        Ok(())
    }

    pub fn peek_reset_vector(&mut self) -> Result<u16, EmulatorError> {
        let high = self.bus.read(RESET_VECTOR_ADDRESS)?;
        let low = self.bus.read(RESET_VECTOR_ADDRESS + 1)?;
        Ok(u16::from_be_bytes([high, low]))
    }

    pub fn fetch_instruction(&mut self) -> Result<(), ChiikoError> {
        let operation = self.fetch_operation()?;
        let mode = self.fetch_grammar(&operation)?;
        let [left, right] = [self.fetch_operand(mode.0)?, self.fetch_operand(mode.1)?];

        self.instruction = Instruction::new(operation, mode, left, right);

        Ok(())
    }

    fn fetch_operation(&mut self) -> Result<Operation, ChiikoError> {
        let byte = self.fetch_byte()?;
        Ok(Operation::from_byte(byte)?)
    }

    fn fetch_grammar(&mut self, operation: &Operation) -> Result<(Mode, Mode), ChiikoError> {
        if operation.has_default_mode() {
            Ok(Mode::from_byte(operation.default_mode)?)
        } else {
            Ok(Mode::from_byte(self.fetch_byte()?)?)
        }
    }

    fn fetch_operand(&mut self, mode: Mode) -> Result<Operand, ChiikoError> {
        // fetches 0-2 bytes depending on the mode
        let value: u16 = match mode.nibble {
            0 => 0,
            1..=5 => self.fetch_byte()? as u16,
            6..=8 => u16::from_be_bytes([self.fetch_byte()?, self.fetch_byte()?]),
            _ => return Err(
                ChiikoError::Emulator(
                    EmulatorError::CannotFetch(format!("Unfetchable Mode {:?}", mode))
                ))
        };

        let operand = match mode.group {
            ModeGroup::NoOperand | ModeGroup::AnyOperand => Operand::NoOperand,
            ModeGroup::Value => Operand::Number(value),
            ModeGroup::Register => Operand::RegisterOp { 
                register: Register::from_code(value as u8)?, 
                direct: true 
            },
            ModeGroup::IndirectRegister => Operand::RegisterOp { 
                register: Register::from_code(value as u8)?, 
                direct: false 
            },
            ModeGroup::ZeroPage | ModeGroup::DirectAddress => Operand::Address { 
                id: None, 
                location: Some(value), 
                direct: true 
            },
            ModeGroup::IndirectZeroPage | ModeGroup::IndirectAddress => Operand::Address { 
                id: None, 
                location: Some(value), 
                direct: false 
            },
            ModeGroup::JumpAddress => Operand::JumpAddress { id: None, location: Some(value) },
            ModeGroup::Accumulator => Operand::RegisterOp {
                register: Register::from_name("A")?,
                direct: true
            },
            ModeGroup::Low => Operand::Number(0xFF),
            ModeGroup::High => Operand::Number(1),
            ModeGroup::Error => {
                return Err(ChiikoError::Emulator(
                    EmulatorError::CannotFetch(format!("Error Operand: {:?}", mode))
                ))
            }
        };

        Ok(operand)
    }

    fn fetch_byte(&mut self) -> Result<u8, EmulatorError> {
        let byte = self.bus.read(self.program_counter)?;
        self.increment_pc();
        Ok(byte)
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

    pub fn pop(&mut self) -> Result<u8, EmulatorError> {
        self.increment_sp();
        self.read(self.stack_pointer)
    }

    pub fn push(&mut self, value: u8) -> Result<(), EmulatorError> {
        let pointer = self.stack_pointer; // Prevents Multiple Borrow errors
        self.decrement_sp();
        self.write(pointer, value)
    }

    pub fn increment_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    pub fn decrement_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
}

impl Chip for Cpu {
    fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        self.bus.read(address)
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), EmulatorError> {
        self.bus.write(address, value)
    }

    fn tick(&mut self) -> Result<(), EmulatorError> {
        let _ = self.bus.tick()?;
        self.cycle_count = self.cycle_count.wrapping_add(1);
        Ok(())
    }

    fn reset(&mut self) -> Result<(), EmulatorError> {
        self.accumulator = 0;
        self.b_register = 0;
        self.c_register = 0;
        self.h_register = 0;
        self.l_register = 0;
        self.i_register = 0;
        self.j_register = 0;
        self.program_counter = self.peek_reset_vector()?;
        self.stack_pointer = STACK_ADDRESS;
        self.status = 0;
        self.cycle_count = 0;
        let _ = self.bus.reset()?;
        Ok(())
    }
}
