mod chiiko;
mod binary;
mod assembler;
mod mode;
mod operation;

// use chiiko::Chiiko;
use crate::assembler::assembly_error::AssemblyError;
use crate::assembler::lexer::Lexer;
use crate::assembler::parser::Parser;
// use crate::operation::Operation;

fn main() -> Result<(), AssemblyError> {
    let test_code: String = "
    #Data
        LINK      \"character_rom.ku\"
        STRING    $0000 \"Here is a string\"
        ARRAY     $0xabcd [
        1, 2, 3, 
        4, 5, 6, 
        7, 8, 9,
        final = 10
        ]
        VAR       $0b101011 counter

    #Logic
        Start:
            LOAD  (r, _)  @HL, A
            COMP I, J 
            POS {
                LOAD  0b1001, C
                ADD(m, a) 0o777
                DIFF 0xfedc, 0xx1243 ; Invalid number
                JUMP :start 
            }
            HALT
    ".to_string();

    let mut lexer = Lexer::new(&test_code);
    let tokens = lexer.lex();
    for token in &tokens {
        println!("{:?}", token)
    }

    println!();

    let mut parser = Parser::new(tokens);
    // let instructions = parser.parse();
    parser.parse();
    for instruction in parser.instructions {
        println!("{:?}", instruction)
    }

    Ok(())
}
