const VARDEF_CONTEXT: &str = "inside of variable definition";
const FNDEF_CONTEXT: &str = "inside of function definition";
const FNBODY_CONTEXT: &str = "inside of function body";
const FNCALL_CONTEXT: &str = "inside of function call";
const ARITHMETIC_CONTEXT: &str = "inside of arithmetic operation";
const RETURN_CONTEXT: &str = "inside of engrave statement";
const CMP_CONTEXT: &str = "inside of weigh statement";
const IF_CONTEXT: &str = "inside of inspect statement";
const WHILE_CONTEXT: &str = "inside of roll statement";
const GROUP_CONTEXT: &str = "inside of `()` group";

use std::ops;

use crate::{error::ParseError, lexer::{SpannedToken, Token}, parser::expression::{BinaryOp, Expression::{self, *}, Statement::{self, *}, SpannedExpr, SpannedStatement}};

pub mod expression;
mod expect;
mod utils;

pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
    debug: bool,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>, debug: bool) -> Parser {
        Parser {
            tokens,
            pos: 0,
            debug,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<SpannedStatement>, Vec<ParseError>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while let Some(t) = self.current() {
            let r = self.parse_statement(t);
            match r {
                Ok(e) => {statements.push(e);},
                Err(e) => errors.push(e),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self, current: Token) -> Result<SpannedStatement, ParseError> {
        let start = self.current_span().start;
        self.advance();

        let stat = match current {
            Token::Throw => self.read_var_def()?,
            Token::Present => self.read_print()?,
            Token::Carve => self.read_fn_def()?,
            Token::Smash | Token::Chip | Token::Mate | Token::Split => self.read_assign(current)?,
            Token::Engrave => self.read_return()?,
            Token::Inspect => self.read_if()?,
            Token::Roll => self.read_while()?,
            Token::Destroy => Break,
            _ => {
                self.pos -= 1;
                Expr(Box::new(self.parse_expression(current)?))
            },
        };

        if self.debug {
            println!("produced statement: {:?}", stat);
        }

        let end = self.current_span().end;
        Ok(SpannedStatement {
            statement: stat,
            span: ops::Range { start, end },
        })
    }

    fn parse_expression(&mut self, current: Token) -> Result<SpannedExpr, ParseError> {
        let start_span = self.current_span();
        self.advance();

        let mut expr = match current {
            Token::Number(n) => Number(n),
            Token::String(s) => Str(s),
            Token::True => Boolean(true),
            Token::False => Boolean(false),
            Token::Identifier(s) => Identifier(s),
            Token::LParen => self.read_group()?,
            Token::Follow => self.read_fn_call()?,
            Token::Weigh => self.read_comparison()?,
            _ => {
                return Err(ParseError {
                    span: start_span.clone(),
                    desc: format!("expected expression, got {:?}", current),
                });
            },
        };

        if matches!(self.current(), Some(Token::Smash | Token::Chip | Token::Mate | Token::Split)) {
            let lhs_span = ops::Range { start: start_span.start, end: self.current_span().end };
            expr = self.read_binaryop(SpannedExpr { expr, span: lhs_span })?;
        }

        if self.debug {
            println!("produced expression: {:?}", expr);
        }

        let end_span = self.current_span();
        Ok(SpannedExpr {
            expr,
            span: ops::Range { start: start_span.start, end: end_span.end },
        })
    }

    fn read_group(&mut self) -> Result<Expression, ParseError> {
        let current = self.expect_some("expression", GROUP_CONTEXT)?;
        let body = self.parse_expression(current)?;
        self.expect(Token::RParen, GROUP_CONTEXT)?;
        Ok(Group(Box::new(body)))
    }

    fn read_binaryop(&mut self, lhs: SpannedExpr) -> Result<Expression, ParseError> {
        let op_token = self.expect_some("operator", ARITHMETIC_CONTEXT)?;
        self.advance();
        let op = BinaryOp::from_token(op_token);
        
        let expected = match op {
            BinaryOp::Add => Token::Into,
            BinaryOp::Sub => Token::Off,
            BinaryOp::Mul => Token::With,
            BinaryOp::Div => Token::From,
        };
        self.expect(expected, ARITHMETIC_CONTEXT)?;

        let current = self.expect_some("value", ARITHMETIC_CONTEXT)?;
        let rhs = self.parse_expression(current)?;

        Ok(Expression::BinaryOp {
            operation: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    fn read_return(&mut self) -> Result<Statement, ParseError> {
        let current = self.expect_some("value", RETURN_CONTEXT)?;
        let val = self.parse_expression(current)?;
        Ok(Return(Box::new(val)))
    }

    fn read_if(&mut self) -> Result<Statement, ParseError> {
        let current = self.expect_some("value", IF_CONTEXT)?;
        let condition = Box::new(self.parse_expression(current)?);

        let body = self.parse_until(&[Token::Refine, Token::Enough])?;

        match self.current() {
            Some(Token::Enough) => {
                self.advance();

                return Ok(If {
                    condition,
                    body,
                    else_: None,
                })
            },
            Some(Token::Refine) => {
                self.advance();

                let mut else_ = Vec::new();
                if let Some(Token::Inspect) = self.current() {
                    self.advance();
                    let start = self.current_span().start;
                    let statement = self.read_if()?;
                    let end = self.current_span().end;

                    else_.push(SpannedStatement {
                        statement,
                        span: ops::Range { start, end }
                    });
                } else {
                    else_.extend(self.parse_until(&[Token::Refine, Token::Enough])?);
                    self.advance();
                }

                return Ok(If {
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

    fn read_while(&mut self) -> Result<Statement, ParseError> {
        self.expect(Token::While, WHILE_CONTEXT)?;

        let current = self.expect_some("value", WHILE_CONTEXT)?;
        let condition = Box::new(self.parse_expression(current)?);

        let body = self.parse_until(&[Token::Enough])?;
        self.advance();

        Ok(While {
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

        Ok(Weigh {
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

        Ok(FnCall {
            name,
            args
        })
    }

    fn read_fn_def(&mut self) -> Result<Statement, ParseError> {
        self.expect(Token::Instruction, FNDEF_CONTEXT)?;
        self.expect(Token::Into, FNDEF_CONTEXT)?;

        let name = self.expect_identifier(FNDEF_CONTEXT)?;

        let mut params = Vec::new();
        while let Some(Token::Retrieve) = self.current() {
            self.advance();
            params.push(self.expect_identifier(FNBODY_CONTEXT)?);
        }

        let body = self.parse_until(&[Token::Enough])?;
        self.advance();

        Ok(FnDef {
            name,
            params,
            body
        })
    }

    fn read_assign(&mut self, op: Token) -> Result<Statement, ParseError> {
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
        Ok(Assign {
            operation: BinaryOp::from_token(op),
            variable: var,
            value: Box::new(value),
        })
    }

    fn read_print(&mut self) -> Result<Statement, ParseError> {
        let current = self.expect_some("value or identifier", "as an argument for `present`")?;
        let value = self.parse_expression(current)?;
        Ok(Print(Box::new(value)))
    }

    fn read_var_def(&mut self) -> Result<Statement, ParseError> {
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

        Ok(Statement::VarDef {
            name,
            value,
        })
    }
}