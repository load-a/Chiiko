#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate)variant: TokenVariant,
    pub(crate)position: (usize, usize),
    pub(crate)id: String,
}

impl Token {
    pub fn new(variant: TokenVariant, position: (usize, usize), id: &str) -> Self {
        Self {
            variant: variant,
            position: position,
            id: id.to_string(),
        }
    }

    pub fn newline(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::Newline,
            position: position,
            id: "\n".to_string(),
        }
    }

    pub fn end_of_file(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::EndOfFile,
            position: position,
            id: "&END_OF_FILE".to_string(),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum TokenVariant {
    Directive,
    Identifier,
    BinaryNumber,
    OctalNumber,
    DecimalNumber,
    HexNumber,
    StringLiteral,
    Element,
    JumpHeader,
    JumpLabel,
    DirectAddress,
    IndirectAddress,
    Comment,
    Comma,
    Newline,
    OpenBracket,
    CloseBracket,
    Quote,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    EndOfFile,
    ModeKey,
    Error(String),
}
