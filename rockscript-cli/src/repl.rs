use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use rockscript_core::{error::report_error, interpreter::{Interpreter, Value}, lexer::tokenize, parser::Parser};
use rustyline::{Completer, Editor, Helper, Highlighter, Hinter, error::ReadlineError, validate::{ValidationContext, ValidationResult, Validator}};

#[derive(Completer, Helper, Hinter, Highlighter)]
struct RockscriptHelper;

impl Validator for RockscriptHelper {
    fn validate(
        &self,
        ctx: &mut ValidationContext,
    ) -> rustyline::Result<ValidationResult> {
        let input = ctx.input();

        let opens = input.matches("roll while").count()
            + input.matches("carve instruction into").count()
            + input.matches("inspect").count();
        let closes = input.matches("enough").count();

        if opens > closes {
            Ok(ValidationResult::Incomplete)
        } else {
            Ok(ValidationResult::Valid(None))
        }
    }
}

pub fn run_repl() -> rustyline::Result<()> {
    let mut rl = Editor::new()?;
    rl.set_helper(Some(RockscriptHelper));

    let path = match history_path() {
        Ok(p) => p,
        Err(e) => {
            println!("{e}");
            return Ok(());
        }
    };

    let _ = rl.load_history(&path);

    let mut interpreter = Interpreter::new();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                
                let tokens = match tokenize(&line, false) {
                    Ok(t) => t,
                    Err(errs) => {
                        for e in errs {
                            report_error(&line, "repl", Box::new(e));
                        }
                        continue;
                    }
                };

                let program =  match Parser::new(tokens, false).parse() {
                    Ok(p) => p,
                    Err(errs) => {
                        for e in errs {
                            report_error(&line, "repl", Box::new(e));
                        }
                        continue;
                    }
                };

                match interpreter.run(program) {
                    Ok(v) => {
                        if !matches!(v, Value::None) {
                            println!("{v}");
                        }
                    },
                    Err(e) => {
                        report_error(&line, "repl", Box::new(e));
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break;
            },
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                println!("got error: {e}");
                break;
            }
        }
    }

    rl.save_history(&path)?;
    Ok(())
}

fn history_path() -> Result<PathBuf, String> {
    let proj_dirs = match ProjectDirs::from("", "", "rockscript") {
        Some(p) => p,
        None => return Err("could not resolve rockscript data folder".to_string()),
    };

    let data_dir = proj_dirs.data_dir();
    if let Err(e) = fs::create_dir_all(data_dir) {
        return Err(format!("Could not create rockscript data directory: {e}"))
    };
    Ok(data_dir.join("history"))
}
