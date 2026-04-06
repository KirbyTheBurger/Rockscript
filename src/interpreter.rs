use std::collections::HashMap;

use crate::parser::Expression;

#[derive(Debug)]
pub enum Value {
    Number(f64),
}

pub struct Interpreter {
    pub variables: HashMap<String, Value>,
    expressions: Vec<Expression>,
    pos: usize,
}

impl Interpreter {
    pub fn new(expressions: Vec<Expression>) -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
            expressions,
            pos: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Some(e) = self.current() {
                match e {
                    Expression::VarDef { name, value } => {
                        self.define_var(name, *value);
                    },
                    _ => break,
                }
            } else {
                break;
            }

            self.advance();
        }
    }

    fn define_var(&mut self, name: String, value: Expression) {

        self.variables.insert(name, Value::from(value));
    }

    fn current(&self) -> Option<Expression> {
        self.expressions.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}

impl Value {
    fn from(expression: Expression) -> Value {
        match expression {
            Expression::Number(n) => Value::Number(n),
            _ => panic!("unknown value or not yet implemented"),
        }
    }
}