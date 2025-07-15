use crate::chiiko::cpu::{ Cpu, 
    register::Register, 
    operand::Operand, operand::Operand::* 
};

pub trait ExtendedReadWrite {
    fn read_indirect_address(&self, address: u16) -> u8;
    fn read_dual_register(&self, pair: [Register; 2]) -> u16;
    fn dual_register_values(&self, pair: [Register; 2]) -> u16;
    fn load_dual_register(&mut self, pair: [Register; 2], value: u16) -> Result<(), &'static str>;
    fn read_operand(&self, operand: Operand) -> u8;
    fn write_operand(&mut self, operand: Operand, value: u8) -> Result<(), &'static str>;
}

impl ExtendedReadWrite for Cpu {
    fn read_indirect_address(&self, address: u16) -> u8 {
        let pointer = self.read(address) as u16;
        self.read(pointer)
    }

    fn read_dual_register(&self, pair: [Register; 2]) -> u16 {
        let high: u8 = self.read_register(pair[0]);
        let low: u8 = self.read_register(pair[1]);

        u16::from_be_bytes([high, low])
    }

    fn load_dual_register(&mut self, pair: [Register; 2], value: u16) -> Result<(), &'static str> {
        let [high, low] = u16::to_be_bytes(value);

        self.load_register(pair[0], high)?;
        self.load_register(pair[1], low)?;
        Ok(())
    }

    // Shows the literal values in the registers
    fn dual_register_values(&self, pair: [Register; 2]) -> u16 {
        u16::from_be_bytes(pair.map(|reg| self.read_register(reg)))
    }

    fn read_operand(&self, operand: Operand) -> u8 {
        match operand {
            Value(value) => value,
            Direct(address) => self.read(address),
            Indirect(pointer) => self.read_indirect_address(pointer),
            RegisterPair(pair) => self.read(self.read_dual_register(pair)),
            IndirectRegisterPair(pair) => {
                let pointer = self.read_dual_register(pair);
                self.read(pointer)
            },
        }
    }

    fn write_operand(&mut self, operand: Operand, value: u8) -> Result<(), &'static str> {
        match operand {
            Value(_) => Err("Cannot write to immediate value"),
            Direct(address) => self.write(address, value),
            Indirect(pointer) => {
                let address = self.read(pointer) as u16;
                self.write(address, value)
            },
            RegisterPair(pair) => self.load_dual_register(pair, value as u16),
            IndirectRegisterPair(pair) => {
                let pointer = self.read_dual_register(pair);
                self.write(pointer, value)
            }
        }
    }
}