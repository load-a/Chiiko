mod chiiko;
mod binary;
mod assembler;

// use chiiko::Chiiko;
use crate::assembler::assembly_error::AssemblyError;
use crate::assembler::lexer::Lexer;

fn main() -> Result<(), AssemblyError> {
    let test_code: String = "
    #Data
      LINK      \"character_rom.ku\"
      STRING    $0000 \"Here is a string\"
      ARRAY     $0xabcd 16[
            1, 2, 3, 
            4, 5, 6, 
            7, 8, 9
        ]
      VAR       $0b101011 counter

    #Logic
      LOAD  (r, r)  @HL, A
      COMP I, J 
      POS {
        LOAD  0b1001, C
        ADD(m, a) 0o777
        DIFF 0xfedc, 0xx1243 ; Fix this invalid number format
      }
      HALT
".to_string();

    let mut lex = Lexer::new(&test_code);

    let tokens = lex.lex();

    for token in tokens {
        println!("{:?}", token)
    }

    Ok(())
}