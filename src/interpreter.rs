use std::{collections::HashMap, ops, rc::Rc};

use crate::{error::RuntimeError, parser::expression::{BinaryOp, Expression::{self, *}, SpannedExpr, SpannedStatement, Statement::{self, *}}};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

pub struct Interpreter {
    pub variables: Vec<HashMap<String, Value>>,
    pub functions: Vec<HashMap<String, Function>>,
    program: Vec<Rc<SpannedStatement>>,
    pos: usize,
}

#[derive(Debug, Clone)]
pub struct Function {
    params: Rc<[String]>,
    body: Rc<[SpannedStatement]>,
}

enum ControlFlow {
    None,
    Break,
    Return(Value),
}

impl Interpreter {
    pub fn new(program: Vec<SpannedStatement>) -> Interpreter {
        let mut interpreter = Interpreter {
            variables: Vec::new(),
            functions: Vec::new(),
            program: program.into_iter().map(Rc::new).collect(),
            pos: 0,
        };

        interpreter.push_scope();

        interpreter
    }

    pub fn run(&mut self) -> Result<(), RuntimeError> {
        while let Some(s) = self.current() {
            self.eval_statement(&s)?;
            self.advance();
        }

        Ok(())
    }

    fn eval_statement(&mut self, statement: &SpannedStatement) -> Result<ControlFlow, RuntimeError> {
        match &statement.statement {
            VarDef { name, value } => {
                let evaluated = self.eval_expression(&value)?;
                self.insert_variable(name, evaluated);
            },
            Print(value) => {
                println!("{}", self.eval_expression(&value)?.to_string())
            },
            Statement::BinaryOp { operation, variable, value } => {
                self.eval_binary_op(operation, variable, &value)?;
            },
            FnDef {name, params, body} => {
                self.define_function(name, &params, &body);
            },
            Return(e) => {
                return Ok(ControlFlow::Return(self.eval_expression(e)?));
            },
            If { condition, body, else_ } => {
                return self.eval_if(condition, body, else_);
            },
            While { condition, body } => {
                return self.eval_while(condition, body);
            },
            Break => return Ok(ControlFlow::Break),
            Expr(e) => { self.eval_expression(e)?; }
        }

        Ok(ControlFlow::None)
    }

    fn eval_expression(&mut self, expr: &SpannedExpr) -> Result<Value, RuntimeError> {
        match &expr.expr {
            FnCall {name, args} => {
                self.eval_fncall(name, args, &expr.span)
            },
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::Str(s) => Ok(Value::String(s.clone())),
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),
            Expression::Identifier(s) => {
                let value = self.get_variable(s.clone());
                match value {
                    Some(v) => Ok(v.clone()),
                    None => panic!("unknown variable"),
                }
            },
            Expression::Weigh {left, right} => {
                let left_value = self.eval_expression(left)?;
                let right_value = self.eval_expression(right)?;

                return Ok(Value::Boolean(left_value >= right_value));
            },
        }
    }

    fn eval_while(&mut self, condition: &Box<SpannedExpr>, body: &Vec<SpannedStatement>) -> Result<ControlFlow, RuntimeError> {
        while matches!(self.eval_expression(condition)?, Value::Boolean(true)) {
            self.push_scope();

            let mut result = ControlFlow::None;
            for e in body {
                let cf = self.eval_statement(e)?;
                if !matches!(cf, ControlFlow::None) {
                    result = cf;
                    break;
                }
            }

            self.pop_scope();

            match result {
                ControlFlow::Break => break,
                ControlFlow::None => {},
                other => return Ok(other)
            }
        }

        Ok(ControlFlow::None)
    }

    fn eval_if(&mut self, condition: &Box<SpannedExpr>, body: &Vec<SpannedStatement>, else_: &Option<Vec<SpannedStatement>>) -> Result<ControlFlow, RuntimeError> {
        let cond_val = self.eval_expression(condition)?;

        if matches!(cond_val, Value::Boolean(true)) {
            self.push_scope();

            let mut result = ControlFlow::None;
            for stat in body {
                let cf = self.eval_statement(stat)?;
                if !matches!(cf, ControlFlow::None) {
                    result = cf;
                    break;
                }
            }

            self.pop_scope();

            if !matches!(result, ControlFlow::None) {
                return Ok(result);
            }
        } else {
            if let Some(v) = else_ {
                self.push_scope();

                let mut result = ControlFlow::None;
                for stat in v {
                    let cf = self.eval_statement(stat)?;
                    if !matches!(cf, ControlFlow::None) {
                        result = cf;
                        break;
                    }
                }

                self.pop_scope();

                if !matches!(result, ControlFlow::None) {
                    return Ok(result);
                }
            }
        }

        Ok(ControlFlow::None)
    }

    fn eval_fncall(&mut self, name: &String, args: &Vec<SpannedExpr>, span: &ops::Range<usize>) -> Result<Value, RuntimeError> {
        let function = self.get_function(name.to_string()).cloned();
        if let Some(func) = function {
            self.push_scope();

            for (param, value) in func.params.iter().zip(args.iter()) {
                let evaluated = self.eval_expression(value)?;
                self.insert_variable(param, evaluated);
            }

            let mut result = ControlFlow::None;
            for stat in func.body.iter() {
                let cf = self.eval_statement(stat)?;
                if !matches!(cf, ControlFlow::None) {
                    result = cf;
                    break;
                }
            }

            self.pop_scope();

            match result {
                ControlFlow::Return(v) => return Ok(v),
                ControlFlow::None => {},
                _ => throw("attempted to break form function", span)?,
            }

            return Ok(Value::None)
        } else {
            throw("unknown function", span)
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

    fn define_function(&mut self, name: &String, params: &Vec<String>, body: &Vec<SpannedStatement>) {
        let function = Function {
            params: Rc::from(params.as_slice()),
            body: Rc::from(body.as_slice()),
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

    fn get_variable_mut(&mut self, name: &String) -> Option<&mut Value> {
        for hm in self.variables.iter_mut().rev() {
            if let Some(v) = hm.get_mut(name) {
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

    fn insert_variable(&mut self, name: &String, value: Value) {
        match self.variables.last_mut() {
            Some(h) => h,
            None => unreachable!(),
        }.insert(name.to_string(), value);
    }

    fn insert_function(&mut self, name: &String, function: Function) {
        self.functions.last_mut().unwrap().insert(name.to_string(), function);
    }

    // TODO: implement proper error handling (not feeling like it rn)
    fn eval_binary_op(&mut self, operation: &BinaryOp, variable: &String, value: &SpannedExpr) -> Result<(), RuntimeError> {
        let evaluated = self.eval_expression(value)?;
        let var = self.get_variable_mut(variable).unwrap();
        match operation {
            BinaryOp::Add => {
                match var {
                    Value::Number(n1) => match evaluated {
                        Value::Number(n2) => *n1 += n2,
                        Value::String(s) => *n1 += s.parse::<f64>().expect("failed to add string to number"),
                        _ => panic!("incompatible types"),
                    },
                    Value::String(s1) => match evaluated {
                        Value::Number(n) => s1.push_str(n.to_string().as_str()),
                        Value::String(s2) => s1.push_str(s2.as_str()),
                        _ => panic!("incompatible types"),
                    },
                    _ => panic!("incompatible types"),
                }
            },
            BinaryOp::Sub => {
                match var {
                    Value::Number(n1) => match evaluated {
                        Value::Number(n2) => *n1 -= n2,
                        Value::String(s) => *n1 -= s.parse::<f64>().expect("failed to add string to number"),
                        _ => panic!("incompatible types"),
                    },
                    _ => panic!("incompatible types"),
                }
            },
            BinaryOp::Mul => {
                match var {
                    Value::Number(n1) => match evaluated {
                        Value::Number(n2) => *n1 *= n2,
                        Value::String(s) => *n1 *= s.parse::<f64>().expect("failed to add string to number"),
                        _ => panic!("incompatible types"),
                    },
                    Value::String(s) => match evaluated {
                        Value::Number(n) => *s = s.repeat(n as usize),
                        _ => panic!("incompatible types"),
                    },
                    _ => panic!("incompatible types"),
                }
            },
            BinaryOp::Div => {
                match var {
                    Value::Number(n1) => match evaluated {
                        Value::Number(n2) => *n1 /= n2,
                        Value::String(s) => *n1 /= s.parse::<f64>().expect("failed to add string to number"),
                        _ => panic!("incompatible types"),
                    },
                    _ => panic!("incompatible types"),
                }
            }
        }

        Ok(())
    }

    fn current(&self) -> Option<Rc<SpannedStatement>> {
        self.program.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}

fn throw<T>(msg: &str, span: &ops::Range<usize>) -> Result<T, RuntimeError> {
    Err(RuntimeError {
        desc: msg.to_string(),
        span: span.clone(),
    })
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
            Value::None => "nil".to_string()
        }
    }
}
