use crate::emulator::components::cpu::Cpu;

impl Cpu {
    pub fn update_flags(&mut self, result: u8, overflow: bool) {
        self.clear_flags();
        self.set_zero_or_negative(result);
        if overflow { self.set_carry() }
    }

    pub fn clear_flags(&mut self) {
        self.status = 0;
    }

    pub fn set_zero_or_negative(&mut self, result: u8) {
        if result == 0 {
            self.set_zero()
        } else {
            self.clear_zero()
        }

        if result & 0x80 != 0 {
            self.set_negative()
        } else {
            self.clear_negative()
        }
    }


    pub fn set_zero(&mut self) {
        self.status |= 0b0000_0001;
    }

    pub fn is_zero(&self) -> bool {
        self.status & 1 > 0
    }

    pub fn clear_zero(&mut self) {
        self.status &= 0b1111_1110;
    }


    pub fn set_negative(&mut self) {
        self.status |= 0b0000_0010;
    }

    pub fn is_negative(&self) -> bool {
        self.status & 0b0000_0010 > 0
    }

    pub fn clear_negative(&mut self) {
        self.status &= 0b1111_1101;
    }


    pub fn set_carry(&mut self) {
        self.status |= 0b0000_0100;
    }

    pub fn is_carry(&self) -> bool {
        self.status & 0b0000_0100 > 0
    }

    pub fn clear_carry(&mut self) {
        self.status &= 0b1111_1011;
    }


    pub fn set_interrupt(&mut self) {
        self.status |= 0b1000_0000;
    }
}
