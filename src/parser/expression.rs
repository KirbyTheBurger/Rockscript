use std::ops;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Statement {
    VarDef {
        name: String,
        value: Box<SpannedExpr>,
    },

    FnDef {
        name: String,
        params: Vec<String>,
        body: Vec<SpannedStatement>,
    },

    Return(Box<SpannedExpr>),

    Assign {
        operation: BinaryOp,
        variable: String,
        value: Box<SpannedExpr>,
    },

    If {
        condition: Box<SpannedExpr>,
        body: Vec<SpannedStatement>,
        else_: Option<Vec<SpannedStatement>>,
    },

    While {
        condition: Box<SpannedExpr>,
        body: Vec<SpannedStatement>,
    },

    Break,

    Print(Box<SpannedExpr>),

    Expr(Box<SpannedExpr>),
}

#[derive(Debug, Clone)]
pub struct SpannedStatement {
    pub statement: Statement,
    pub span: ops::Range<usize>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Str(String),
    Boolean(bool),
    Identifier(String),
    Group(Box<SpannedExpr>),

    BinaryOp {
        operation: BinaryOp,
        lhs: Box<SpannedExpr>,
        rhs: Box<SpannedExpr>,
    },

    FnCall {
        name: String,
        args: Vec<SpannedExpr>,
    },

    Weigh {
        left: Box<SpannedExpr>,
        right: Box<SpannedExpr>,
    },
}

#[derive(Debug, Clone)]
pub struct SpannedExpr {
    pub expr: Expression,
    pub span: ops::Range<usize>
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinaryOp {
    pub fn from_token(token: Token) -> BinaryOp {
        match token {
            Token::Smash => BinaryOp::Add,
            Token::Chip => BinaryOp::Sub,
            Token::Mate => BinaryOp::Mul,
            Token::Split => BinaryOp::Div,
            _ => unreachable!(),
        }
    }
}