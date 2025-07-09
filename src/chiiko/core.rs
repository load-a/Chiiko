#[derive(Debug, PartialEq)]
pub struct Chiiko {
    pub memory: [u8; 0xFFFF],
}

impl Chiiko {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xFFFF]
        }
    }
}