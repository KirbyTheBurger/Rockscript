use rockscript_core::{error::report_error, interpreter::{Interpreter, Value}, lexer::tokenize, parser::Parser};

// if an error occurs in the rockscript code, this function returns `None`
pub fn run(source: String, filename: String, debug: bool) -> Option<Value> {
    let tokens = match tokenize(&source, debug) {
        Ok(t) => t,
        Err(errs) => {
            for e in errs {
                report_error(&source, &filename, Box::new(e));
            }
            return None;
        }
    };

    let mut parser = Parser::new(tokens, debug);
    let expressions = match parser.parse() {
        Ok(e) => e,
        Err(errs) => {
            for e in errs {
                report_error(&source, &filename, Box::new(e));
            }
            return None;
        }
    };

    let mut interpreter = Interpreter::new();
    let result = interpreter.run(expressions);
    match result {
        Err(e) => {
            report_error(&source, &filename, Box::new(e));
            None
        },
        Ok(v) => Some(v)
    }
}