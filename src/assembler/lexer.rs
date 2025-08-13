use crate::assembler::{assembly_error::AssemblyError, source::Source, token::Token};

const MAX_OPERAND_COUNT: usize = 2;

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

    pub fn standard_lex(line: &str) -> Result<Vec<Token>, AssemblyError> {
        let mut tokens = Vec::new();
        let trimmed = line.trim();

        if trimmed.is_empty() {
            return Ok(tokens);
        }

        let mut code = trimmed;
        let mut comment = "";

        if let Some(index) = trimmed.find(';') {
            code = trimmed[..index].trim_end();
            comment = trimmed[index + 1..].trim_start();
        }

        if code.is_empty() {
            if !comment.is_empty() {
                tokens.push(Token::Comment(comment.to_string()))
            }

            return Ok(tokens);
        }

        let mut parts = code.split_whitespace();

        if let Some(first) = parts.next() {
            if first.starts_with('#') {
                tokens.push(Token::Directive(first.trim_start_matches('#').to_string()));
            } else if first.ends_with(':') {
                tokens.push(Token::Label(first.trim_end_matches(':').to_string()));
            } else {
                tokens.push(Token::Opcode(first.to_string()));
            }
        }

        if let Some(second) = parts.next() {
            if second.starts_with('(') {
                tokens.push(Token::Mode(second.to_string()));
            } else {
                for operand in second.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    tokens.push(Token::Operand(operand.to_string()));
                }
            }
        }

        for operand_group in parts {
            for operand in operand_group.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                tokens.push(Token::Operand(operand.to_string()));
            }
        }

        let operand_count = tokens.iter().filter(|t| matches!(t, Token::Operand(_))).count();

        if operand_count > MAX_OPERAND_COUNT {
            return Err(AssemblyError::TooManyOperands(line.to_string()))
        }

        if !comment.is_empty() {
            tokens.push(Token::Comment(comment.to_string()))
        }

        Ok(tokens)
    }
}
