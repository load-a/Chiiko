#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Header(String),
    Label(String),
    Opcode(String),
    Mode(String),
    Operand(String),
    Comment(String),
    Directive(String),
    BlockStart,
    BlockEnd,
    Unknown(String)
}