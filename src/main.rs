mod chiiko;
mod binary;
mod assembler;
mod mode;
mod operation;

// use chiiko::Chiiko;
use crate::assembler::assembly_error::AssemblyError;
use crate::assembler::lexer::Lexer;
use crate::assembler::parser::Parser;
use crate::assembler::encoder::{symbol_table::SymbolTable, syntax_checker::SyntaxChecker};
// use crate::operation::Operation;

fn main() -> Result<(), AssemblyError> {
    let test_code: String = "
    #Data
    LINK      \"character_rom.ku\"
    STRING    $0000 \"Here is a string\"
    ARRAY     $8000 [
                        first,
                        1, 2, 3, 
                        4, 5, 6, 
                        7, 8, 9,
                        final = 10
                    ]
    VAR       $45 something

    #Logic
    Start:
        LOAD(@r, r)  @HL, A
        COMP I, J
        POS {
            LOAD  0b1001, C
            ADD(m, a) $0o777 
            OR 0b1010000
            DIFF 0xfedc, 0xx1243 ; Invalid number
            JUMP :start, :end
        }
    _Next:
        LOAD 45 I
        LOAD @B J
        COMP I J
        POS {
            OR $final
        }
        ZERO {
            OUT @first
            JUMP :END
        }
        NEG {
            OR C
            NOT C
            SAVE
        }
    END:
        HALT
    ".to_string();

    let mut lexer = Lexer::new(&test_code);
    let tokens = lexer.lex();
    for token in &tokens {
        println!("{:?}", token)
    }

    println!();

    let mut parser = Parser::new(tokens);
    parser.parse();
    for instruction in &parser.instructions {
        println!("{:?}", instruction)
    }

    println!();

    let mut table = SymbolTable::from_ast(&parser.instructions);
    for (label, symbol) in table.table {
        println!("{:?}: {:?}", label, symbol)
    }

    SyntaxChecker::check(parser.instructions)?;

    Ok(())
}
