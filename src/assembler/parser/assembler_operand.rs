#[derive(Clone, PartialEq, Debug)]
pub enum AssemblerOperand {
    Number(u16),
    Register(String),
    DirectAddress(String),
    IndirectAddress(String),
    String(String),
    JumpAddress(String),
    Identifier(String),
    Error(String),
    Placeholder,
    StartCount(usize),
    EndCount,
    NamedElement {name: String, value: u8},
    NoOperand,
}
