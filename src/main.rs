use crate::{parser::Parser, tokenizer::Lexer};

mod tokenizer;
mod parser;

fn main() {
    let mut lexer = Lexer::new("throw 12 rocks at x");
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("{:?}", ast);
}
