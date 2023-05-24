// mod compiler;
// mod evaluate;
mod analyzer;
mod emitter;
mod errors;
mod evaluate;
mod lexer;
mod parser;
mod pretty_print;
mod tokens;

use std::{env, fs};

use crate::{
    analyzer::analyze_program, emitter::emit_code, parser::Parser, pretty_print::pretty_print,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_count = args.len();
    if arg_count < 2 {
        panic!("Usage keithc program.keith");
    }
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Filed reading file");

    match Parser::parse(&contents) {
        Ok(mut program) => match analyze_program(&mut program) {
            Ok(t) => {
                let code = emit_code(&program).expect("msg");
                println!("{}", pretty_print(&program));
            }
            Err(_error) => println!("Failed!"),
        },
        Err(error) => println!("Failed: {:?}", error),
    }
}
