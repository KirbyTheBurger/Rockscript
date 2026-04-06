use crate::{interpreter::Interpreter, parser::Parser, tokenizer::Lexer};

mod tokenizer;
mod parser;
mod interpreter;

fn main() {
    let mut lexer = Lexer::new("throw 12 rocks at x");
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let expressions = parser.parse();
    println!("{:?}", expressions);

    let mut interpreter = Interpreter::new(expressions);
    interpreter.run();
    println!("{:?}", interpreter.variables);
}
