use std::fs;

use clap::Parser as CliParser;
use crate::{error::report_error, interpreter::Interpreter, lexer::tokenize, parser::Parser};
use Commands::*;

mod lexer;
mod parser;
mod interpreter;
mod error;

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
            let source = fs::read_to_string(&file);
            match source {
                Ok(s) => {
                    let tokens = match tokenize(&s) {
                        Ok(t) => t,
                        Err(errs) => {
                            for e in errs {
                                report_error(&s, &file, Box::new(e));
                            }
                            return;
                        }
                    };
                    if debug { println!("Tokens: {:?}", tokens); }

                    let mut parser = Parser::new(tokens);
                    let expressions = match parser.parse() {
                        Ok(e) => e,
                        Err(errs) => {
                            for e in errs {
                                report_error(&s, &file, Box::new(e));
                            }
                            return;
                        }
                    };
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