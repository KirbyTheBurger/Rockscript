use std::{collections::HashMap, ops::{AddAssign, SubAssign}};

use crate::parser::{BinaryOperation, Expression};

#[derive(Debug, Clone)]
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
            Expression::Print(v) => {
                println!("{}", self.eval_value(*v).to_string())
            },
            Expression::BinaryOp{..} => {
                self.eval_binary_op(expression);
            }
            _ => println!("unknown expression"),
        }

        self.advance();
    }

    fn eval_binary_op(&mut self, expression: Expression) {
        if let Expression::BinaryOp {
            operation, variable, value
        } = expression {
            let evaluated = self.eval_value(*value);
            match operation {
                BinaryOperation::Add => {
                    *self.variables.get_mut(&variable).unwrap() += evaluated;
                },
                BinaryOperation::Sub => {
                    *self.variables.get_mut(&variable).unwrap() -= evaluated;
                }
            }
        } else {
            panic!("can only evaluate binary operation")
        }
    }

    fn eval_value(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Number(n) => Value::Number(n),
            Expression::String(s) => Value::String(s),
            Expression::Identifier(s) => {
                let value = self.variables.get(&s);
                match value {
                    Some(v) => v.clone(),
                    None => panic!("unknown variable"),
                }
            },
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

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(n1) => match rhs {
                Value::Number(n2) => *n1 += n2,
                Value::String(s) => *n1 += s.parse::<f64>().expect("failed to add string to number"),
            },
            Value::String(s1) => match rhs {
                Value::Number(n) => s1.push_str(n.to_string().as_str()),
                Value::String(s2) => s1.push_str(s2.as_str()),
            }
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(n1) => match rhs {
                Value::Number(n2) => *n1 -= n2,
                Value::String(s) => *n1 -= s.parse::<f64>().expect("failed to add string to number"),
            },
            Value::String(_) => panic!("cant subtract 2 strings")
        }
    }
}