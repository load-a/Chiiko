#[derive(PartialEq, Copy, Clone)]
pub enum LexerState {
    Normal,
    StringLiteral,
    ByteArray,
    ModeSignature,
}
