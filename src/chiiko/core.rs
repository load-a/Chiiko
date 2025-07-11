use crate::chiiko::cpu::Cpu;
use crate::chiiko::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub struct Chiiko {
    pub cpu: Cpu,
}

impl Chiiko {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
        }
    }
}
