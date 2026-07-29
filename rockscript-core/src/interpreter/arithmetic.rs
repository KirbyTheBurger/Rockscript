use std::ops;

use crate::{error::RuntimeError, interpreter::Value, parser::expression::BinaryOp};

pub(crate) fn calculate(
    operation: &BinaryOp,
    lhs: Value,
    rhs: Value,
    span: &ops::Range<usize>
) -> Result<Value, RuntimeError> {
    let gen_error = || {
        let (op1, op2) = match operation {
            BinaryOp::Add => ("add", "to"),
            BinaryOp::Sub => ("subtract", "from"),
            BinaryOp::Mul => ("multiply", "by"),
            BinaryOp::Div => ("divide", "by"),
        };

        let val_str = |val: &Value| {
            match val {
                Value::Boolean(_) => "boolean",
                Value::Number(_) => "number",
                Value::String(_) => "string",
                Value::None => "nil",
            }
        };

        let msg = format!("attempted to {} {} {} {}", op1, val_str(&rhs), op2, val_str(&lhs));
        Err::<>(RuntimeError {
            desc: msg,
            span: span.clone(),
        })
    };

    let err = gen_error();

    Ok(match operation {
        BinaryOp::Add => {
            match rhs {
                Value::Number(n1) => match lhs {
                    Value::Number(n2) => Value::Number(n1 + n2),
                    _ => err?,
                },
                Value::String(s1) => match lhs {
                    Value::Number(n) => Value::String(format!("{s1}{n}")),
                    Value::String(s2) => Value::String(format!("{s1}{s2}")),
                    Value::Boolean(b) => Value::String(format!("{s1}{b}")),
                    _ => err?,
                },
                _ => err?,
            }
        },
        BinaryOp::Sub => {
            match rhs {
                Value::Number(n1) => match lhs {
                    Value::Number(n2) => Value::Number(n1 - n2),
                    _ => err?,
                },
                _ => err?,
            }
        },
        BinaryOp::Mul => {
            match rhs {
                Value::Number(n1) => match lhs {
                    Value::Number(n2) => Value::Number(n1 * n2),
                    Value::String(s) => Value::String(s.repeat(n1 as usize)),
                    _ => err?,
                },
                Value::String(s) => match lhs {
                    Value::Number(n) => Value::String(s.repeat(n as usize)),
                    _ => err?,
                },
                _ => err?,
            }
        },
        BinaryOp::Div => {
            match rhs {
                Value::Number(n1) => match lhs {
                    Value::Number(n2) => Value::Number(n1 / n2),
                    _ => err?,
                },
                _ => err?,
            }
        }
    })
}