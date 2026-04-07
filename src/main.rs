use std::io;

use crate::{interpreter::Interpreter, parser::Parser, tokenizer::Lexer};

mod tokenizer;
mod parser;
mod interpreter;

fn main() {
    let mut lexer = Lexer::new(get_input().as_str());
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let expressions = parser.parse();
    println!("{:?}", expressions);

    let mut interpreter = Interpreter::new(expressions);
    interpreter.run();
    println!("{:?}", interpreter.variables);
    println!("{:?}", interpreter.functions);
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");

    input.trim().to_string()
}
