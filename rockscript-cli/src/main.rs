use std::fs;

use clap::Parser as CliParser;
use rockscript_core::{error::report_error, interpreter::Interpreter, lexer::tokenize, parser::Parser};

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

#[allow(clippy::needless_return)]
fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run {file, debug} => {
            let source = fs::read_to_string(&file);
            match source {
                Ok(s) => {
                    let tokens = match tokenize(&s, debug) {
                        Ok(t) => t,
                        Err(errs) => {
                            for e in errs {
                                report_error(&s, &file, Box::new(e));
                            }
                            return;
                        }
                    };

                    let mut parser = Parser::new(tokens, debug);
                    let expressions = match parser.parse() {
                        Ok(e) => e,
                        Err(errs) => {
                            for e in errs {
                                report_error(&s, &file, Box::new(e));
                            }
                            return;
                        }
                    };

                    let mut interpreter = Interpreter::new();
                    if let Err(e) = interpreter.run(expressions) {
                        report_error(&s, &file, Box::new(e));
                        return;
                    }
                },

                Err(e) => println!("{e}"),
            }
        }
    }
}