use std::fs;

use clap::Parser as CliParser;
use logos::Logos;
use crate::{interpreter::Interpreter, lexer::Token, parser::Parser};
use Commands::*;

mod lexer;
mod parser;
mod interpreter;

#[derive(CliParser)]
#[command(about, version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run a program (.rock file)
    Run {
        file: String,
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Run {file, debug} => {
            let source = fs::read_to_string(file);
            match source {
                Ok(c) => {
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
                },

                Err(e) => println!("{e}"),
            }
        }
    }
}