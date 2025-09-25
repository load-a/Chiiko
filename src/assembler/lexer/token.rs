#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate)variant: TokenVariant,
    pub(crate)position: (usize, usize),
    pub(crate)snippet: String,
}

impl Token {
    pub fn new(variant: TokenVariant, position: (usize, usize), snippet: &str) -> Self {
        Self {
            variant: variant,
            position: position,
            snippet: snippet.to_string(),
        }
    }

    pub fn newline(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::Newline,
            position: position,
            snippet: "\n".to_string(),
        }
    }

    pub fn end_of_file(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::EndOfFile,
            position: position,
            snippet: "&END_OF_FILE".to_string(),
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
    LabelHeader,
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
