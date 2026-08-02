mod lexer;

use std::env::args;
use std::fs;

#[allow(non_snake_case)]
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename.sqlng>", args[0]);
        return;
    }

    let program: &String = &args[1];
    let sourceCode = fs::read_to_string(program).expect("Unable to read file: {program}");

    let tokens = lexer::lex(&sourceCode);
    for token in tokens {
        lexer::printToken(&token);
    }
}
