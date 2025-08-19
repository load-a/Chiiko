use crate::assembler::lexer::token::Token;
use crate::assembler::parser::{operand::Operand, instruction::Instruction};
use std::num::ParseIntError;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    instructions: Vec<Instruction>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens,
            instructions: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Instruction> {
        let mut position = 0;
        let mut instruction = Instruction::default();
        let mut program = Vec::new();
        let mut counter_id = 0;

        while let Some(token) = self.tokens.get(position) {
            match &self.tokens[position] {
                Token::Identifier(id) => {
                    let value = id.to_uppercase().to_string();

                    if instruction.opcode.is_empty() {
                        instruction.opcode = value;
                    }  else {
                        instruction.operands.push(Operand::Identifier(value))
                    }
                },
                Token::BinaryNumber(number) => {
                    let value = u16::from_str_radix(number, 2).unwrap();
                    instruction.operands.push(Operand::Number(value))
                },
                Token::OctalNumber(number) => {
                    let value = u16::from_str_radix(number, 8).unwrap();
                    instruction.operands.push(Operand::Number(value))
                },
                Token::DecimalNumber(number) => {
                    let value = u16::from_str_radix(number, 10).unwrap();
                    instruction.operands.push(Operand::Number(value))
                },
                Token::HexNumber(number) => {
                    let value = u16::from_str_radix(number, 16).unwrap();
                    instruction.operands.push(Operand::Number(value))
                },
                Token::LabelHeader(id) => {
                    if !instruction.opcode.is_empty() {
                        panic!("Parse Error: Label Header processed as operand")
                    }

                    let value = id.to_uppercase().to_string();
                    instruction.opcode = "&HEADER".to_string();
                    instruction.operands.push(Operand::JumpLabel {id: value, address: 0});
                },
                Token::JumpLabel(id) => {
                    let value = id.to_uppercase().to_string();
                    instruction.operands.push(Operand::JumpLabel {id: value, address: 0});
                },
                Token::DirectAddress(address) => {
                    let value = address.to_uppercase().to_string();

                    if matches!(value.chars().next().unwrap(), '0'..='9') || 
                        (value.len() > 1 && matches!(&value[0..2], "0x" | "0b" | "0o"))
                    {
                        instruction.operands.push(
                            Operand::DirectAddress(Self::parse_number(&value).unwrap() as u16)
                        )
                    } else {
                        instruction.operands.push(Operand::DirectVariable(value))
                    }
                },
                Token::IndirectAddress(address) => {
                    let value = address.to_uppercase().to_string();

                    if matches!(value.chars().next().unwrap(), '0'..='9') || 
                        (value.len() > 1 && matches!(&value[0..2], "0x" | "0b" | "0o"))
                    {
                        instruction.operands.push(
                            Operand::IndirectAddress(Self::parse_number(&value).unwrap() as u16)
                        )
                    } else {
                        instruction.operands.push(Operand::IndirectVariable(value))
                    }
                },
                Token::String(slice) => {
                    instruction.operands.push(Operand::String(slice.to_string()))
                }
                Token::Element(assignment) => {
                    let element = assignment.to_uppercase().to_string();

                    if let Some(index) = element.find('=') {
                        let name = element[..index].trim_end().to_string();
                        let value = element[index + 1..].trim_start();
                        
                        instruction.operands.push(
                            Operand::NamedElement {
                                name: name, 
                                value: Self::parse_number(&value).unwrap() as u16,
                            }
                        );

                        position += 1;
                        continue;
                    }

                    if matches!(element.chars().next().unwrap(), '0'..='9') ||
                        (element.len() > 1 && matches!(&element[0..2], "0x" | "0b" | "0o"))
                    {
                        instruction.operands.push(
                            Operand::Number(Self::parse_number(&element).unwrap() as u16)
                        );
                    } else if instruction.opcode == "ARRAY".to_string() {
                        instruction.operands.push(Operand::NamedElement {
                            name: element,
                            value: 0,
                        });
                    } else {
                        instruction.operands.push(Operand::ModeKey(element))
                    }
                },
                Token::Newline | Token::CloseBracket => {
                    if instruction != Instruction::default() { program.push(instruction) };
                    instruction = Instruction::default();
                },
                Token::Error{message, ..} => {
                    let value = message.to_uppercase().to_string();
                    instruction.operands.push(Operand::Error(value));
                },
                Token::OpenBrace => {
                    instruction.operands.push(Operand::CountStart(counter_id));
                    counter_id += 1;
                },
                Token::CloseBrace => {
                    if instruction.opcode.is_empty() {
                        instruction.opcode = "&END_COUNT".to_string();
                    }
                    instruction.operands.push(Operand::CountEnd(counter_id));
                },
                Token::OpenParen => {
                    instruction.operands.push(Operand::ModeStart);
                },
                Token::CloseParen => {
                    instruction.operands.push(Operand::ModeEnd);
                },
                Token::OpenBracket => {
                    instruction.operands.push(Operand::ArrayStart);
                },
                Token::CloseBracket => {
                    instruction.operands.push(Operand::ArrayEnd);
                },
                Token::Directive(id) =>  {
                    let value = id.to_uppercase().to_string();
                    instruction.opcode = "&DIRECTIVE".to_string();
                    instruction.operands.push(Operand::Directive(value))
                }
                Token::EndOfFile => break,
                _ => (),
            }

            position += 1;
        }

        if instruction != Instruction::default() { 
            program.push(instruction) 
        }

        program
    }

    fn parse_number(s: &str) -> Result<usize, std::num::ParseIntError> {
        if let Some(rest) = s.strip_prefix("0X") {
            usize::from_str_radix(rest, 16)
        } else if let Some(rest) = s.strip_prefix("0O") {
            usize::from_str_radix(rest, 8)
        } else if let Some(rest) = s.strip_prefix("0B") {
            usize::from_str_radix(rest, 2)
        } else {
            usize::from_str_radix(s, 10)
        }
    }
}
