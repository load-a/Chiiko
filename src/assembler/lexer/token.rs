#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub(crate) variant: TokenVariant,
    pub(crate) position: (usize, usize),
    pub(crate) id: String,
}

impl Token {
    pub fn new(variant: TokenVariant, position: (usize, usize), id: &str) -> Self {
        Self {
            variant: variant,
            position: position,
            id: id.to_uppercase().to_string(),
        }
    }

    pub fn string(position: (usize, usize), message: &str) -> Self {
        Self {
            variant: TokenVariant::StringLiteral,
            position: position,
            id: message.to_string(),
        }
    }

    pub fn comment(position: (usize, usize), message: &str) -> Self {
        Self {
            variant: TokenVariant::Comment,
            position: position,
            id: message.to_string(),
        }
    }

    pub fn newline(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::Newline,
            position: position,
            id: "\n".to_string(),
        }
    }

    pub fn comma(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::Comma,
            position: position,
            id: ",".to_string(),
        }
    }

    pub fn assignment(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::AssignmentOperator,
            position: position,
            id: "=".to_string(),
        }
    }

    pub fn error(position: (usize, usize), message: String, id: &str) -> Self {
        Self {
            variant: TokenVariant::Error(message),
            position: position,
            id: id.to_string(),
        }
    }

    pub fn terminator(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::InstructionTerminator,
            position: position,
            id: "%TERMINATOR".to_string(),
        }
    }

    pub fn end_of_file(position: (usize, usize)) -> Self {
        Self {
            variant: TokenVariant::EndOfFile,
            position: position,
            id: "%END_OF_FILE".to_string(),
        }
    }

    pub fn log(&self) -> String {
        match &self.variant {
            TokenVariant::Error(e) => {
                format!(
                    "{:#04}.{:#02} %ERROR: {:012} e: {:?}",
                    self.position.0, self.position.1, self.id, e,
                )
            }
            TokenVariant::Comment => {
                format!(
                    "{:#04}.{:#02} ; {}",
                    self.position.0, self.position.1, self.id,
                )
            }
            TokenVariant::Quote => {
                format!("{:#04}.{:#02} %QUOTE", self.position.0, self.position.1,)
            }
            _ => {
                format!(
                    "{:#04}.{:#02} {:020} v: {:?}",
                    self.position.0, self.position.1, self.id, self.variant,
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenVariant {
    Directive,
    Identifier,
    Number,
    StringLiteral,
    JumpHeader,
    JumpLabel,
    DirectAddress,
    IndirectAddress,
    LazyAddress,
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
    AssignmentOperator,
    ChipLabel,
    InstructionTerminator,
    Error(String),
}
