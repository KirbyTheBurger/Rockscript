use std::{collections::HashMap, ops::{AddAssign, DivAssign, MulAssign, SubAssign}};

use crate::parser::{BinaryOperation, Expression};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

pub struct Interpreter {
    pub variables: Vec<HashMap<String, Value>>,
    pub functions: Vec<HashMap<String, Function>>,
    expressions: Vec<Expression>,
    pos: usize,
}

#[derive(Debug)]
pub struct Function {
    params: Vec<String>,
    body: Vec<Expression>,
}

impl Interpreter {
    pub fn new(expressions: Vec<Expression>) -> Interpreter {
        let mut interpreter = Interpreter {
            variables: Vec::new(),
            functions: Vec::new(),
            expressions,
            pos: 0,
        };

        interpreter.push_scope();

        interpreter
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
            },
            Expression::FnDef {name, params, body} => {
                self.define_function(name, params, body);
            },
            _ => println!("unknown expression"),
        }
    
        self.advance();
    }

    fn push_scope(&mut self) {
        self.variables.push(HashMap::new());
        self.functions.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.variables.pop();
        self.functions.pop();
    }

    fn define_function(&mut self, name: String, params: Vec<String>, body: Vec<Expression>) {
        let function = Function {
            params,
            body,
        };
        self.insert_function(name, function);
    }

    fn get_variable(&self, name: String) -> Option<&Value> {
        for hm in self.variables.iter().rev() {
            if let Some(v) = hm.get(&name) {
                return Some(v);
            }
        }

        None
    }

    fn get_variable_mut(&mut self, name: String) -> Option<&mut Value> {
        for hm in self.variables.iter_mut().rev() {
            if let Some(v) = hm.get_mut(&name) {
                return Some(v);
            }
        }

        None
    }

    fn insert_variable(&mut self, name: String, value: Value) {
        self.variables.last_mut().unwrap().insert(name, value);
    }

    fn insert_function(&mut self, name: String, function: Function) {
        self.functions.last_mut().unwrap().insert(name, function);
    }

    fn eval_binary_op(&mut self, expression: Expression) {
        if let Expression::BinaryOp {
            operation, variable, value
        } = expression {
            let evaluated = self.eval_value(*value);
            match operation {
                BinaryOperation::Add => {
                    *self.get_variable_mut(variable).unwrap() += evaluated;
                },
                BinaryOperation::Sub => {
                    *self.get_variable_mut(variable).unwrap() -= evaluated;
                },
                BinaryOperation::Mul => {
                    *self.get_variable_mut(variable).unwrap() *= evaluated;
                },
                BinaryOperation::Div => {
                    *self.get_variable_mut(variable).unwrap() /= evaluated
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
            Expression::Boolean(b) => Value::Boolean(b),
            Expression::Identifier(s) => {
                let value = self.get_variable(s);
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
        self.insert_variable(name, evaluated);
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
            Value::Boolean(b) => b.to_string(),
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(n1) => match rhs {
                Value::Number(n2) => *n1 += n2,
                Value::String(s) => *n1 += s.parse::<f64>().expect("failed to add string to number"),
                _ => panic!("incompatible types"),
            },
            Value::String(s1) => match rhs {
                Value::Number(n) => s1.push_str(n.to_string().as_str()),
                Value::String(s2) => s1.push_str(s2.as_str()),
                _ => panic!("incompatible types"),
            },
            _ => panic!("incompatible types"),
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(n1) => match rhs {
                Value::Number(n2) => *n1 -= n2,
                Value::String(s) => *n1 -= s.parse::<f64>().expect("failed to add string to number"),
                _ => panic!("incompatible types"),
            },
            _ => panic!("incompatible types"),
        }
    }
}

impl MulAssign for Value {
    fn mul_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(n1) => match rhs {
                Value::Number(n2) => *n1 *= n2,
                Value::String(s) => *n1 *= s.parse::<f64>().expect("failed to add string to number"),
                _ => panic!("incompatible types"),
            },
            Value::String(s) => match rhs {
                Value::Number(n) => *s = s.repeat(n as usize),
                _ => panic!("incompatible types"),
            },
            _ => panic!("incompatible types"),
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, rhs: Self) {
        match self {
            Value::Number(n1) => match rhs {
                Value::Number(n2) => *n1 /= n2,
                Value::String(s) => *n1 /= s.parse::<f64>().expect("failed to add string to number"),
                _ => panic!("incompatible types"),
            },
            _ => panic!("incompatible types"),
        }
    }
}