mod chiiko;
mod binary;
mod assembler;

// use chiiko::Chiiko;
use crate::assembler::assembly_error::AssemblyError;
use crate::assembler::lexer::Lexer;

fn main() -> Result<(), AssemblyError> {
    let test_code: String = "
    #Data
    List [15, 14, 13, abc = 12]
    #Logic
    Start:
    Add abc 0x123
    MOVE (r, r) @hl c
    String >0xabcd \"linelineline line\"
    RAND A B
    JUMP :Start
".to_string();

    let mut lex = Lexer::new(&test_code);

    let tokens = lex.lex();

    for token in tokens {
        println!("{:?}", token)
    }

    Ok(())
}