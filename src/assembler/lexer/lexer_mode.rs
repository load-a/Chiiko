use crate::assembler::lexer::{lexer_state::LexerState, Lexer};

impl Lexer {
    pub(crate) fn enter_mode(&mut self, state: LexerState) {
        self.mode.push(state);
    }

    pub(crate) fn exit_mode(&mut self) {
        self.mode.pop();
    }

    pub(crate) fn current_mode(&self) -> LexerState {
        *self.mode.last().unwrap_or(&LexerState::Normal)
    }
}
