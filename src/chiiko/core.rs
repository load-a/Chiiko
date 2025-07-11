use crate::chiiko::cpu::Cpu;

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
