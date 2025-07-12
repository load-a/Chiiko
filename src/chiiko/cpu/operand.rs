use crate::chiiko::cpu::register::Register;

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    Immediate(u8),
    Direct(u16),
    Indirect(u16),
    RegisterPair([Register; 2]),
    IndirectRegisterPair([Register; 2]),
}