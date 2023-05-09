// mod compiler;
// mod evaluate;
mod lexer;
mod parser;
mod tokens;

use std::{env, fs};

use parser::ProgramNode;

use crate::parser::Parser;

// use crate::evaluate::evaluate;

fn emit_code(node: &ProgramNode) -> Result<Vec<u8>, String> {
    Ok(vec![])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_count = args.len();
    if arg_count < 2 {
        panic!("Usage keithc program.keith");
    }
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Filed reading file");

    match Parser::parse(&contents) {
        Ok(node) => {
            let code = emit_code(&node).expect("msg");
            println!("Suck Sex!");
        }
        Err(error) => println!("Failed: {:?}", error)
    }

}
