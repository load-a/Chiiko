// use std::fs;

mod emulator;
// mod binary;
// mod assembler;
mod chiiko_error;
mod mode;
mod numeral_parser;
mod operand;
mod operation;
mod register;

// use chiiko::Chiiko;
// use crate::assembler::assembly_error::AssemblyError;
// use crate::assembler::lexer::Lexer;
// use crate::assembler::parser::Parser;
// use crate::assembler::encoder::{symbol_table::SymbolTable, syntax_checker::SyntaxChecker,
// instruction_generator::InstructionGenerator
// };
// use crate::operation::Operation;

fn main() {
    // let test_code: String = fs::read_to_string("test_binaries/micro_test.ku").unwrap();
    // // let test_code: String = fs::read_to_string("test_binaries/short_test.ku").unwrap();

    // let mut lexer = Lexer::new(&test_code);
    // let tokens = lexer.lex();
    // for token in &tokens {
    //     println!("{:?}", token)
    // }

    // println!();

    // let mut parser = Parser::new(tokens);
    // parser.parse();
    // for instruction in &parser.instructions {
    //     println!("{:?}", instruction)
    // }

    // println!();

    // let mut table = SymbolTable::from_ast(&parser.instructions);
    // for (label, symbol) in table.table {
    //     println!("{:?}: {:?}", label, symbol)
    // }

    // let mut generator = InstructionGenerator::new(parser.instructions);
    // generator.generate();

    // Ok(())
}
