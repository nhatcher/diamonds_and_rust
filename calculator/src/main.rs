// mod compiler;
// mod evaluate;
mod analyzer;
mod builtins;
mod emitter;
mod errors;
mod evaluate;
mod lexer;
mod opcodes;
mod parser;
mod pretty_print;
mod tokens;
#[cfg(test)]
mod test;

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

    let contents = fs::read_to_string(file_path).expect("Failed reading file");

    match Parser::parse(&contents) {
        Ok(mut ast) => match analyze_program(&mut ast) {
            Ok(symbol_table) => {
                let code = emit_code(&ast, &symbol_table).expect("Error emitting code");
                println!("{}", pretty_print(&ast));
                println!("{:?}", code);
            }
            Err(error) => println!("Failed! {}", error),
        },
        Err(error) => println!("Failed: {}", error),
    }
}
