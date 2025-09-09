use crate::emulator::components::{bus::Bus, chip::Chip, instruction::Instruction};
use crate::emulator::EmulatorError;

const RESET_VECTOR_ADDRESS: u16 = 0xFFFE; // The last two bytes of ROM (big endian)
const NO_OPERAND: u8 = 0;
const OPERAND_ERROR: u8 = 0xF;
const STACK_ADDRESS: u16 = 0x1FFF;

pub struct Cpu {
    pub(crate) accumulator: u8,
    pub(crate) b_register: u8,
    pub(crate) c_register: u8,
    pub(crate) h_register: u8,
    pub(crate) l_register: u8,
    pub(crate) i_register: u8,
    pub(crate) j_register: u8,
    pub(crate) program_counter: u16,
    pub(crate) stack_pointer: u16,
    pub(crate) status: u8,
    pub(crate) bus: Bus,
    pub(crate) cycle_count: u8,
    pub(crate) instruction: Instruction,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new(Bus::default()).unwrap()
    }
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
            status: 0,
            cycle_count: 0,
            bus: bus,
            instruction: Instruction::default(), // FIX
        };

        cpu.program_counter = cpu.peek_reset_vector()?;

        Ok(cpu)
    }

    pub fn peek_reset_vector(&mut self) -> Result<u16, EmulatorError> {
        let high = self.bus.read(RESET_VECTOR_ADDRESS)?;
        let low = self.bus.read(RESET_VECTOR_ADDRESS + 1)?;
        Ok(u16::from_be_bytes([high, low]))
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
