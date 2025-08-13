mod chiiko;
mod binary;
mod assembler;

// use chiiko::Chiiko;

use assembler::assembly_error::AssemblyError;
use assembler::lexer::Lexer;

fn main() -> Result<(), AssemblyError> {
    let example = [
        "ADD    (v,a) 1",
        "MULT   A, B",
        "PUSH   A 0xFF ; Comment is here",
        "LOAD   A, B, C",
        "RTRN"
    ];

    for line in &example {
        println!("{}", line);

        match Lexer::standard_lex(line) {
            Ok(tokens) => {
                for token in tokens {
                    println!("  {:?}", token)
                }
            },
            Err(e) => println!("  Error: {:?}", e),
        }

        println!();
    }

    Ok(())
}