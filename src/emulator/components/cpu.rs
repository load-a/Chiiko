use crate::emulator::components::{
    chip::Chip, bus::Bus, instruction::Instruction,
};
use crate::emulator::EmulatorError;
use crate::register::Register;
use crate::operation::Operation;
use crate::mode::Mode;
use crate::operand::Operand;
use crate::mode::mode_group::ModeGroup;

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

        cpu.program_counter = cpu.fetch_reset_vector()?;

        Ok(cpu)
    }

    pub fn find(&self, source: &Operand) -> Result<u8, EmulatorError> {
        match source {
            Operand::Number(value) => Ok(*value as u8),
            Operand::RegisterOp { register, direct } => {
                if *direct {
                    self.read_register(register.code)
                } else {
                    Ok(self.read(self.read_indirect_register(register.code).unwrap()))
                }
            },
            Operand::Address { location, direct, .. } => {
                if *direct {
                    Ok(self.read(*location))
                } else {
                    Ok(self.read(self.read(*location) as u16))
                }
            },
            Operand::NoOperand => Ok(0),
            _ => Err(EmulatorError::CannotFind(format!("{:?}", source))),
        }
    }

//     pub fn send(&mut self, destination: &Operand, value: u8) -> Result<(), &str> {
//         match destination {
//             Operand::RegisterOp { register, direct } => {
//                 if *direct {
//                     self.write_register(register.code, value)?
//                 } else {
//                     self.write(self.read_register(register.code).unwrap() as u16, value)?
//                 }
//             },
//             Operand::Address { location, direct, .. } => {
//                 if *direct {
//                     self.write(*location, value)?
//                 } else {
//                     self.write(self.read(*location) as u16, value)?
//                 }
//             },
//             _ => return Err("Invalid destination"),
//         }

//         Ok(())
//     }

//     pub fn resolve_address(&self, destination: &Operand) -> Result<u16, &str> {
//         match destination {
//             Operand::RegisterOp { register, direct } => {
//                 if *direct {
//                     match register.code {
//                         0..=6 => Err("Direct Register does not resolve to address"),
//                         9..=11 => self.read_register_pair(register.code),
//                         _ => Err("Cannot resolve address for Invalid Register Code"),
//                     }
//                 } else {
//                     Ok(self.read_register(register.code).unwrap() as u16)
//                 }
//             }
//             Operand::Address { location, direct, .. } => {
//                 if *direct {
//                     Ok(*location)
//                 } else {
//                     Ok(self.read(*location) as u16)
//                 }
//             }
//             _ => Err("Invalid destination"),
//         }
//     }

//     // Returns register values as an Address
//     pub fn read_indirect_register(&self, register_code: u8) -> Result<u16, &str> {
//         match register_code {
//             0..=6 => Ok(self.read_register(register_code).unwrap() as u16),
//             9..=11 => self.read_register_pair(register_code),
//             _ => Err("Cannot find register: Bad Register Code")
//         }
//     }

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

//     pub fn write_register(&mut self, register_code: u8, value: u8) -> Result<(), &str> {
//         match register_code {
//             0 => self.accumulator = value,
//             1 => self.b_register = value,
//             2 => self.c_register = value,
//             3 => self.h_register = value,
//             4 => self.l_register = value,
//             5 => self.i_register = value,
//             6 => self.j_register = value,
//             _ => return Err("Write to invalid Register Code"),
//         }

//         Ok(())
//     }

//     // Returns value in register pair
//     pub fn read_register_pair(&self, register_code: u8) -> Result<u16, &str> {
//         match register_code {
//             9 => Ok(u16::from_be_bytes([self.b_register, self.c_register])),
//             10 => Ok(u16::from_be_bytes([self.h_register, self.l_register])),
//             11 => Ok(u16::from_be_bytes([self.i_register, self.j_register])),
//             _ => Err("Invalid Register Pair code")
//         }
//     }

//     pub fn write_register_pair(&mut self, code: u8, value: u16) -> Result<(), &str> {
//         let [big, small] = value.to_be_bytes();
        
//         match code {
//             9 => {
//                 self.b_register = big;
//                 self.c_register = small;
//             },
//             10 => {
//                 self.h_register = big;
//                 self.l_register = small;
//             },
//             11 => {
//                 self.i_register = big;
//                 self.j_register = small;
//             },
//             _ => return Err("Invalid Register Pair code")
//         }

//         Ok(())
//     }

    pub fn fetch_reset_vector(&mut self) -> Result<u16, EmulatorError> {
        let high = self.bus.read(RESET_VECTOR_ADDRESS)?;
        let low = self.bus.read(RESET_VECTOR_ADDRESS + 1)?;
        Ok(u16::from_be_bytes([high, low]))
    }

//     pub fn fetch_instruction(&mut self) -> Result<(), &str> {
//         let operation = self.fetch_operation();
//         let mode = self.fetch_grammar(&operation);
//         let [left, right] = [self.fetch_operand(mode.0), self.fetch_operand(mode.1)];

//         self.instruction = Instruction::new(operation, mode, left, right);

//         Ok(())
//     }

//     fn fetch_operation(&mut self) -> Operation {
//         let byte = self.fetch_byte();
//         Operation::from_byte(byte)
//     }

//     fn fetch_grammar(&mut self, operation: &Operation) -> (Mode, Mode) {
//         if operation.has_default_mode() {
//             Mode::from_byte(operation.default_mode)
//         } else {
//             Mode::from_byte(self.fetch_byte())
//         }
//     }

//     fn fetch_operand(&mut self, mode: Mode) -> Operand {
//         // fetches 0-2 bytes depending on the mode
//         let value: u16 = match mode.nibble {
//             1..=5 => self.fetch_byte() as u16,
//             6..=8 => u16::from_be_bytes([self.fetch_byte(), self.fetch_byte()]),
//             _ => 0xFFFF // Fetch no bytes
//         };

//         match mode.group {
//             ModeGroup::NoOperand | ModeGroup::Default => Operand::NoOperand,
//             ModeGroup::Value => Operand::Number(value),
//             ModeGroup::Register => Operand::RegisterOp { 
//                 register: Register::from_byte(value as u8), 
//                 direct: true 
//             },
//             ModeGroup::IndirectRegister => Operand::RegisterOp { 
//                 register: Register::from_byte(value as u8), 
//                 direct: false 
//             },
//             ModeGroup::ZeroPage | ModeGroup::DirectAddress => Operand::Address { 
//                 id: String::new(), 
//                 location: value, 
//                 direct: true 
//             },
//             ModeGroup::IndirectZeroPage | ModeGroup::IndirectAddress => Operand::Address { 
//                 id: String::new(), 
//                 location: value, 
//                 direct: false 
//             },
//             ModeGroup::JumpAddress => Operand::JumpAddress { id: String::new(), location: value },
//             ModeGroup::Accumulator => Operand::RegisterOp {
//                 register: Register::from_name("A"),
//                 direct: true
//             },
//             ModeGroup::Low => Operand::Number(0xFF),
//             ModeGroup::High => Operand::Number(1),
//             ModeGroup::Error => Operand::Error(format!("Cannot fetch Operand: {:?}", mode)),
//         }
//     }

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

//     pub fn pop(&mut self) -> Result<u8, &str> {
//         self.increment_sp();
//         Ok(self.read(self.stack_pointer))
//     }

//     pub fn push(&mut self, value: u8) -> Result<(), &str> {
//         let pointer = self.stack_pointer; // Prevents Multiple Borrow errors
//         self.decrement_sp();
//         self.write(pointer, value)?;
//         Ok(())
//     }

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
        self.program_counter = self.fetch_reset_vector()?;
        self.stack_pointer = STACK_ADDRESS;
        self.status = 0;
        self.cycle_count = 0;
        let _ = self.bus.reset()?;
        Ok(())
    }
}
