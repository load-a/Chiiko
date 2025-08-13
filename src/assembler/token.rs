#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Directive(String),
    Label(String),
    Opcode(String),
    Mode(String),
    Operand(String),
    Comment(String),
    BlockStart,
    BlockEnd,
    Unknown(String),
    Error(String)
}