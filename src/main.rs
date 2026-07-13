use std::{env, fs};

use logos::Logos;

use crate::{interpreter::Interpreter, lexer::Token, parser::Parser};

mod lexer;
mod parser;
mod interpreter;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) => match s.as_str() {
            "run" => {
                let file_name;
                match args.get(2) {
                    Some(n) => file_name = n.clone(),
                    None => file_name = String::from("main.rock"),
                }

                let source = fs::read_to_string(file_name);
                match source {
                    Ok(c) => {
                        let debug = matches!(args.get(3).map(String::as_str), Some("--debug"));

                        let lexer = Token::lexer(&c);
                        let tokens: Vec<Token> = lexer.filter_map(Result::ok).collect();
                        if debug { println!("Tokens: {:?}", tokens); }

                        let mut parser = Parser::new(tokens);
                        let expressions = parser.parse();
                        if debug { println!("Expressions: {:?}", expressions); }

                        let mut interpreter = Interpreter::new(expressions);
                        interpreter.run();
                        if debug {
                            println!("Variables: {:?}", interpreter.variables);
                            println!("Functions: {:?}", interpreter.functions);
                        }
                    }
                    Err(_) => eprintln!("invalid file name"),
                }
            },
            _ => eprintln!("unknown command"),
        },
        None => {
            println!("Rockscript version {}", VERSION);
            println!("Commands:");
            println!("rockscript run <file>");
        }
    }
}