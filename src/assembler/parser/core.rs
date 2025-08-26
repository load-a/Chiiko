use crate::assembler::lexer::token::Token;
use crate::assembler::parser::{assembler_operand::AssemblerOperand, ast_node::ASTNode, 
    opcode::Opcode, ast_node::MacroNode, mode_key::ModeKey,
};
use std::num::ParseIntError;
use crate::mode::Mode;
use crate::mode::mode_group::ModeGroup;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pub instructions: Vec<ASTNode>,
    position: usize,
    counter_id: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens,
            instructions: Vec::new(),
            position: 0,
            counter_id: 0,
        }
    }

    pub fn parse(&mut self) {
        while self.position < self.tokens.len() {
            match self.current_token() {
                Token::Directive(id) => {
                    let mode = Self::normalize_string(id);
                    self.parse_directive(mode);
                },
                Token::LabelHeader(id) => {
                    let label = Self::normalize_string(id);
                    self.parse_label(label);
                }
                Token::Identifier(id) => {
                    let mnemonic = Self::normalize_string(id);
                    self.parse_instruction(mnemonic);
                },
                Token::CloseBrace => {
                    self.instructions.push(
                        ASTNode::Macro(MacroNode::EndCount { id: self.counter_id })
                    );
                    self.counter_id += 1;
                    self.advance();
                },
                Token::Newline => self.advance(),
                Token::EndOfFile => break,
                _ => {
                    self.instructions.push(
                        ASTNode::Error(format!("Cannot parse {:?}", self.current_token()))
                    );
                    self.advance();
                }
            }
        }
    }

    fn parse_directive(&mut self, mode: String) {
        self.instructions.push(ASTNode::Directive(mode));
        self.advance();
    }

    fn parse_label(&mut self, label: String) {
        self.instructions.push(ASTNode::Label(label));
        self.advance();
    }

    fn parse_instruction(&mut self, mnemonic: String) {
        if Opcode::is_macro(&mnemonic) {
            self.parse_macro(mnemonic);
            return;
        }

        self.advance();

        let mut mode: Option<(ModeGroup, ModeGroup)> = self.parse_mode();
        let mut operands: Vec<AssemblerOperand> = Vec::new();

        while !matches!(self.current_token(), Token::Newline | Token::CloseBrace){
            if matches!(self.current_token(), Token::Comma | Token::Quote | Token::Comment(_)) { 
                self.advance();
                continue;
            }

            operands.push(self.lookup_operand());
            self.advance();
        }

        self.advance();
        self.instructions.push(
            ASTNode::Instruction {
                mnemonic: mnemonic,
                mode: mode,
                operands: operands,
            });
    }

    fn parse_mode(&mut self) -> Option<(ModeGroup, ModeGroup)> {
        if self.current_token() == Token::OpenParen {
            self.advance(); // Open Paren
            let left_code = self.lookup_mode_key();
            self.advance(); // Left code
            self.advance(); // Comma
            let right_code = self.lookup_mode_key();
            self.advance(); // Right Code
            self.advance(); // Close Paren
            Some((left_code, right_code))
        } else {
            None
        }
    }

    fn parse_macro(&mut self, mnemonic: String) {
        self.advance();
        let address = self.lookup_operand();
        self.advance();
        
        match mnemonic.as_str() {
            "STRING" => {
                self.advance();

                self.instructions.push(
                    ASTNode::Macro(MacroNode::StringData {
                        address: address,
                        value: self.lookup_operand(),
                    })
                );

                self.advance();
            },
            "ARRAY" => {
                let mut elements = Vec::new();

                self.advance();

                while self.current_token() != Token::CloseBracket {
                    if self.current_token() == Token::Comma {
                        self.advance();
                        continue;
                    }

                    elements.push(self.lookup_operand());
                    self.advance();
                }

                self.instructions.push(
                    ASTNode::Macro(MacroNode::ArrayData {
                        address: address,
                        elements: elements,
                    })
                );
            },
            "VAR" | "NAME" => {
                let label = self.lookup_operand();
                self.advance();

                self.instructions.push(
                    ASTNode::Macro(MacroNode::VariableData {
                        address: address,
                        label: label,
                    })
                );
            },
            "LINK" => {
                let operand = self.lookup_operand();
                if let AssemblerOperand::String(filename) = operand {
                    self.instructions.push(ASTNode::Macro(MacroNode::LinkData(filename.to_string())));
                } else {
                    self.instructions.push(ASTNode::Macro(MacroNode::MacroError(
                        format!("LINK requires String operand; Instead found: {:?}", address)
                    )));
                }

                self.advance();
            }
            _ => ()
        } 

        self.advance();
    }

    fn current_token(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn normalize_string(slice: &str) -> String {
        slice.trim().to_uppercase().to_string()
    }

    pub fn normalize_number(slice: &str) -> Result<usize, std::num::ParseIntError> {
        if let Some(rest) = slice.strip_prefix("0X") {
            usize::from_str_radix(rest, 16)
        } else if let Some(rest) = slice.strip_prefix("0O") {
            usize::from_str_radix(rest, 8)
        } else if let Some(rest) = slice.strip_prefix("0B") {
            usize::from_str_radix(rest, 2)
        } else {
            usize::from_str_radix(slice, 10)
        }
    }

    fn lookup_mode_key(&self) -> ModeGroup {
        if let Token::ModeKey(code) = self.current_token() {
            let key = Self::normalize_string(code);
            Mode::from_key(&key).group
        } else {
            panic!("Key missing from Mode: {:?}", self.current_token())
        }
    }

    fn lookup_operand(&self) -> AssemblerOperand {
        match self.current_token() {
            Token::BinaryNumber(value) => {
                let number = u16::from_str_radix(value, 2).unwrap();
                AssemblerOperand::Number(number)
            },
            Token::OctalNumber(value) => {
                let number = u16::from_str_radix(value, 8).unwrap();
                AssemblerOperand::Number(number)
            },
            Token::DecimalNumber(value) => {
                let number = u16::from_str_radix(value, 10).unwrap();
                AssemblerOperand::Number(number)
            },
            Token::HexNumber(value) => {
                let number = u16::from_str_radix(value, 16).unwrap();
                AssemblerOperand::Number(number)
            },
            Token::Identifier(value) => {
                let id = Self::normalize_string(value);
                if id.len() < 3 {
                    AssemblerOperand::Register(id)
                } else {
                    AssemblerOperand::Identifier(id)
                }
            },
            Token::DirectAddress(value) => {
                let id = Self::normalize_string(value);
                AssemblerOperand::DirectAddress(id)
            },
            Token::IndirectAddress(value) => {
                let id = Self::normalize_string(value);
                AssemblerOperand::IndirectAddress(id)
            },
            Token::JumpLabel(value) => {
                let address = Self::normalize_string(value);
                AssemblerOperand::JumpAddress(address)
            },
            Token::Element(element) => {
                if let Some(index) = element.find('=') {
                    let name = Self::normalize_string(&element[0..index]);
                    let value = &element[index + 1..].trim();
                    let number = Self::normalize_number(&value)
                                    .unwrap_or_else(|_| panic!(
                                    "Initialized Named Element must assign a Value; found: \"{}\"", 
                                    value
                                ));

                    AssemblerOperand::NamedElement { name: name, value: number as u8 }
                } else if let Ok(number) = Self::normalize_number(element) {
                    AssemblerOperand::Number(number as u16)
                } else {
                    AssemblerOperand::Identifier(Self::normalize_string(element))
                }
            },
            Token::OpenBrace => AssemblerOperand::StartCount(self.counter_id),
            Token::String(value) => AssemblerOperand::String(value.to_string()),
            Token::Error {message, line_and_column, snippet} => AssemblerOperand::Error(
                format!("Lexer Error: {} {:?} \"{}\"", message, line_and_column, snippet)
            ),
            token => AssemblerOperand::Placeholder(format!("{:?}", token)),
        }
    }
}
