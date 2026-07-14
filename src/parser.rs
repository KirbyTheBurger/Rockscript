const VARDEF_CONTEXT: &str = "inside of variable definition";
const FNDEF_CONTEXT: &str = "inside of function definition";
const FNBODY_CONTEXT: &str = "inside of function body";
const FNCALL_CONTEXT: &str = "inside of function call";
const ARITHMETIC_CONTEXT: &str = "inside of arithmetic operation";
const RETURN_CONTEXT: &str = "inside of engrave statement";
const CMP_CONTEXT: &str = "inside of weigh statement";
const IF_CONTEXT: &str = "inside of inspect statement";
const WHILE_CONTEXT: &str = "inside of roll statement";

use std::ops;

use crate::{error::ParseError, lexer::{SpannedToken, Token}};
use Expression::*;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Str(String),
    Boolean(bool),

    Identifier(String),

    VarDef {
        name: String,
        value: Box<Expression>,
    },

    FnDef {
        name: String,
        params: Vec<String>,
        body: Vec<Expression>,
    },
    FnCall {
        name: String,
        args: Vec<Expression>,
    },
    Return(Box<Expression>),

    BinaryOp {
        operation: BinaryOp,
        variable: String,
        value: Box<Expression>,
    },
    Weigh {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    If {
        condition: Box<Expression>,
        body: Vec<Expression>,
        else_: Option<Vec<Expression>>,
    },

    While {
        condition: Box<Expression>,
        body: Vec<Expression>,
    },
    Break,

    Print(Box<Expression>),
}

pub struct SpannedExpr {
    expr: Expression,
    span: ops::Range<usize>
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Parser {
        Parser {
            tokens,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, Vec<ParseError>> {
        let mut expressions = vec![];
        let mut errors = vec![];

        while let Some(t) = self.current() {
            let r = self.parse_expression(t);
            match r {
                Ok(e) => expressions.push(e),
                Err(e) => errors.push(e),
            }
            self.advance();
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(expressions)
    }

    fn parse_expression(&mut self, current: Token) -> Result<Expression, ParseError> {
        self.advance();
        match current {
            Token::Throw => self.read_var_def(),
            Token::Number(n) => {
                Ok(Number(n))
            },
            Token::String(s) => {
                Ok(Str(s))
            },
            Token::True => {
                Ok(Boolean(true))
            },
            Token::False => {
                Ok(Boolean(false))
            },
            Token::Present => self.read_print(),
            Token::Carve => self.read_fn_def(),
            Token::Identifier(s) => {
                Ok(Identifier(s))
            },
            Token::Follow => self.read_fn_call(),
            Token::Smash | Token::Chip | Token::Mate | Token::Split => self.read_binary_op(current),
            Token::Engrave => {
                self.read_return()
            },
            Token::Weigh => self.read_comparison(),
            Token::Inspect => self.read_if(),
            Token::Roll => self.read_while(),
            Token::Destroy => {
                Ok(Break)
            },
            _ => {
                self.unexpected_token()
            },
        }
    }

    fn read_return(&mut self) -> Result<Expression, ParseError> {
        let current = self.expect_some("value", RETURN_CONTEXT)?;
        let val = self.parse_expression(current)?;
        Ok(Return(Box::new(val)))
    }

    fn read_if(&mut self) -> Result<Expression, ParseError> {
        let current = self.expect_some("value", IF_CONTEXT)?;
        let condition = Box::new(self.parse_expression(current)?);

        let body = self.parse_until(&[Token::Refine, Token::Enough])?;

        match self.current() {
            Some(Token::Enough) => {
                self.advance();

                return Ok(Expression::If {
                    condition,
                    body,
                    else_: None,
                })
            },
            Some(Token::Refine) => {
                self.advance();

                let mut else_ = Vec::new();
                if let Some(Token::Inspect) = self.current() {
                    else_.push(self.read_if()?);
                } else {
                    else_.extend(self.parse_until(&[Token::Refine, Token::Enough])?);
                }

                return Ok(Expression::If {
                    condition,
                    body,
                    else_: Some(else_)
                })
            },
            _ => {
                unreachable!()
            },
        }
    }

    fn read_while(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::While, WHILE_CONTEXT)?;

        let current = self.expect_some("value", WHILE_CONTEXT)?;
        let condition = Box::new(self.parse_expression(current)?);

        let body = self.parse_until(&[Token::Enough])?;

        Ok(Expression::While {
            condition,
            body,
        })
    }

    fn read_comparison(&mut self) -> Result<Expression, ParseError> {
        let current = self.expect_some("value", CMP_CONTEXT)?;
        let left = Box::new(self.parse_expression(current)?);
        
        self.expect(Token::Against, CMP_CONTEXT)?;

        let current = self.expect_some("value", CMP_CONTEXT)?;
        let right = Box::new(self.parse_expression(current)?);

        Ok(Expression::Weigh {
            left,
            right
        })
    }

    fn read_fn_call(&mut self) -> Result<Expression, ParseError> {
        let name = self.expect_identifier(FNCALL_CONTEXT)?;

        let mut args = Vec::new();
        while let Some(Token::With | Token::And) = self.current() {
            self.advance();
            let current = self.expect_some("value", FNCALL_CONTEXT)?;
            args.push(self.parse_expression(current)?);
        }

        Ok(Expression::FnCall {
            name,
            args
        })
    }

    fn read_fn_def(&mut self) -> Result<Expression, ParseError> {
        self.expect(Token::Instruction, FNDEF_CONTEXT)?;
        self.expect(Token::Into, FNDEF_CONTEXT)?;

        let name = self.expect_identifier(FNDEF_CONTEXT)?;

        let mut params = Vec::new();
        while let Some(Token::Retrieve) = self.current() {
            self.advance();
            params.push(self.expect_identifier(FNBODY_CONTEXT)?);
        }

        let body = self.parse_until(&[Token::Enough])?;

        Ok(Expression::FnDef {
            name,
            params,
            body
        })
    }

    fn read_binary_op(&mut self, op: Token) -> Result<Expression, ParseError> {
        let current = self.expect_some("value", ARITHMETIC_CONTEXT)?;
        let value = self.parse_expression(current)?;
        
        let expected = match op {
            Token::Smash => Token::Into,
            Token::Chip => Token::Off,
            Token::Mate => Token::With,
            Token::Split => Token::From,
            _ => unreachable!(),
        };
        self.expect(expected, ARITHMETIC_CONTEXT)?;

        let var = self.expect_identifier(ARITHMETIC_CONTEXT)?;
        Ok(Expression::BinaryOp {
            operation: BinaryOp::from_token(op),
            variable: var,
            value: Box::new(value),
        })
    }

    fn read_print(&mut self) -> Result<Expression, ParseError> {
        let current = self.expect_some("value or identifier", "as an argument for `present`")?;
        let value = self.parse_expression(current)?;
        Ok(Print(Box::new(value)))
    }

    fn read_var_def(&mut self) -> Result<Expression, ParseError> {
        let value;
        if matches!(self.current(), Some(Token::Rock)) {
            self.advance();
            self.expect(Token::Named, VARDEF_CONTEXT)?;
            let current = self.expect_some("number", VARDEF_CONTEXT)?;
            value = Box::new(self.parse_expression(current)?);
        } else {
            let current = self.expect_some("number", VARDEF_CONTEXT)?;
            value = Box::new(self.parse_expression(current)?);
            self.expect(Token::Rock, "after number value inside variable definition")?;
        }

        self.expect(Token::At, VARDEF_CONTEXT)?;

        let name = self.expect_identifier(VARDEF_CONTEXT)?;

        Ok(Expression::VarDef {
            name,
            value,
        })
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.pos)
            .map_or(None, |t| Some(t.token.clone()))
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current_span(&self) -> std::ops::Range<usize> {
        self.tokens.get(self.pos)
            .map(|t| t.span.clone())
            .unwrap_or_else(|| {
                let end = self.tokens.last().map(|t| t.span.end).unwrap_or(0);
                end..end
            })
    }

    fn expect(&mut self, token: Token, context: &str) -> Result<(), ParseError> {
        if self.current().as_ref() == Some(&token) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                span: self.current_span(),
                desc: format!("expected {:?} {}, found {:?}", token, context, self.current()),
            })
        }
    }

    fn expect_some(&mut self, expected: &str, context: &str) -> Result<Token, ParseError> {
        if let Some(t) = self.current() {
            self.advance();
            Ok(t)
        } else {
            Err(ParseError {
                span: self.current_span(),
                desc: format!("expected {} {}, found <EOF>", expected, context)
            })
        }
    }

    fn expect_identifier(&mut self, context: &str) -> Result<String, ParseError> {
        match self.current() {
            Some(Token::Identifier(s)) => { self.advance(); Ok(s) }
            _ => Err(ParseError {
                span: self.current_span(),
                desc: format!("expected identifier {}, found {:?}", context, self.current()),
            }),
        }
    }

    fn unexpected_token(&mut self) -> Result<Expression, ParseError> {
        let token = self.current().unwrap();
        Err(ParseError {
            span: self.current_span(),
            desc: format!("dit not expect {:?}", token),
        })
    }

    fn parse_until(&mut self, stop: &[Token]) -> Result<Vec<Expression>, ParseError> {
        let mut body = Vec::new();
        while !stop.iter().any(|t| self.current().as_ref() == Some(t)) {
            body.push(self.parse_expression(self.current().unwrap())?);
        }
        Ok(body)
    }
}

impl BinaryOp {
    fn from_token(token: Token) -> BinaryOp {
        match token {
            Token::Smash => BinaryOp::Add,
            Token::Chip => BinaryOp::Sub,
            Token::Mate => BinaryOp::Mul,
            Token::Split => BinaryOp::Div,
            _ => panic!("cant parse token into binary operation, this shouldnt panic unless theres a bug in the interpreter"),
        }
    }
}