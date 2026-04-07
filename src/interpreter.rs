use std::{collections::HashMap, ops::{AddAssign, DivAssign, MulAssign, SubAssign}};

use crate::parser::{BinaryOperation, CmpOperation, Expression};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone)]
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

    fn eval_expression(&mut self, expression: Expression) -> Option<Value> {
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
            Expression::FnCall {name, args} => {
                self.eval_function(name, args);
            },
            Expression::Return(e) => {
                return Some(self.eval_value(*e));
            },
            Expression::If { condition, body, else_ } => {
                self.eval_if(condition, body, else_);
            }
            _ => println!("unknown expression"),
        }
        
        self.advance();
        None
    }

    fn eval_if(&mut self, condition: Box<Expression>, body: Vec<Expression>, else_: Option<Vec<Expression>>) {
        if let Value::Boolean(true) = self.eval_value(*condition) {
            self.push_scope();

            for e in body {
                self.eval_expression(e);
            }

            self.pop_scope();
        } else {
            if let Some(v) = else_ {
                self.push_scope();

                for e in v {
                    self.eval_expression(e);
                }

                self.pop_scope();
            }
        }
    }

    fn eval_function(&mut self, name: String, args: Vec<Expression>) -> Option<Value> {
        let function = self.get_function(name).cloned();
        if let Some(f) = function {
            self.push_scope();

            for (param, value) in f.params.iter().zip(args.iter()) {
                self.define_var(param.clone(), value.clone());
            }

            let mut return_value = None;
            for e in &f.body {
                if let Some(v) = self.eval_expression(e.clone()) {
                    return_value = Some(v);
                    break;
                }
            }

            self.pop_scope();
            return return_value;
        } else {
            panic!("unknown function called")
        }
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

    fn get_function(&mut self, name: String) -> Option<&Function> {
        for hm in self.functions.iter().rev() {
            if let Some(v) = hm.get(&name) {
                return Some(v)
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
            Expression::FnCall {name, args} => {
                match self.eval_function(name, args) {
                    Some(v) => v,
                    None => panic!("expected function to return value"),
                }
            },
            Expression::Comparison {operation, left, right} => {
                match operation {
                    CmpOperation::Weigh => {
                        let left_value = self.eval_value(*left);
                        let right_value = self.eval_value(*right);

                        return Value::Boolean(left_value >= right_value);
                    }
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
            Value::Boolean(b) => match b {
                true => String::from("big"),
                false => String::from("small"),
            },
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