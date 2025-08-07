use crate::assembler::{assembly_error::AssemblyError, source::Source, token::Token};

pub mod Lexer {
    use super::*;

    fn lex() -> Result<(), AssemblyError> {
        Ok(())
    }

    fn group_bracketed_lines(lines: &[String]) -> Result<Vec<String>, AssemblyError> {
        let mut grouped = Vec::new();
        let mut buffer = String::new();
        let mut open_brackets = 0;

        for line in lines {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            buffer.push_str(trimmed);
            buffer.push(' ');

            open_brackets += trimmed.matches('[').count();
            open_brackets -= trimmed.matches(']').count();

            if open_brackets == 0 && !buffer.is_empty() {
                grouped.push(buffer.trim().to_string());
                buffer.clear();
            }
        }

        if buffer.is_empty() {
            grouped.push(buffer.trim().to_string())
        }

        if open_brackets != 0 {
            Err(AssemblyError::UnmatchedBracket)
        } else {
            Ok(grouped)
        }
    }

    fn course_lex(line: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let line = line.trim();

        if line.is_empty() {
            return tokens;
        }

        if let Some(index) = line.find(';') {
            let (code, comment) = line.split_at(index);

            if !code.trim().is_empty() {
                tokens.extend(course_lex(code));
            }

            tokens.push(Token::Comment(comment.trim().to_string()));
            return tokens
        }

        if line.ends_with(':') {
            tokens.push(Token::Label(line.trim_end_matches(':').to_string()));
            return tokens
        }

        tokens.push(Token::Unknown(line.to_string()));
        tokens
    }
}
