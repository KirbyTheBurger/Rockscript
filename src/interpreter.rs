use std::collections::HashMap;

use crate::parser::Expression;

#[derive(Debug)]
pub enum Value {
    Number(f64),
    String(String),
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
            let current = self.current();

            if let Some(Expression::EOF) | None = current {
                break;
            }

            self.eval_expression(current.unwrap());
        }
    }

    fn eval_expression(&mut self, expression: Expression) {
        match expression {
            Expression::VarDef { name, value } => {
                self.define_var(name, *value);
            },
            _ => println!("unknown expression"),
        }

        self.advance();
    }

    fn eval_value(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Number(n) => Value::Number(n),
            Expression::String(s) => Value::String(s),
            _ => panic!("unknown value or not yet implemented"),
        }
    }

    fn define_var(&mut self, name: String, value: Expression) {
        let evaluated = self.eval_value(value);
        self.variables.insert(name, evaluated);
    }

    fn current(&self) -> Option<Expression> {
        self.expressions.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}