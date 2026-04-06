use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    String(String),

    Identifier(String),

    VarDef {
        name: String,
        value: Box<Expression>,
    },

    BinaryOp {
        operation: BinaryOperation,
        variable: String,
        value: Box<Expression>,
    },

    Print(Box<Expression>),

    Error,
    EOF,
}

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();

        loop {
            let expr = self.parse_expression();
            if !matches!(expr, Expression::EOF) {
                expressions.push(expr);
            } else { break; }
        }

        expressions
    }

    fn parse_expression(&mut self) -> Expression {
        if let Some(e) = self.current() {
            match e {
                Token::Throw => self.read_var_def(),
                Token::Number(n) => {
                    self.advance();
                    Expression::Number(n)
                },
                Token::String(s) => {
                    self.advance();
                    Expression::String(s)
                },
                Token::Present => self.read_print(),
                Token::Identifier(s) => {
                    self.advance();
                    Expression::Identifier(s)
                },
                Token::Smash | Token::Chip | Token::Mate => self.read_binary_op(),
                Token::EOF => Expression::EOF,
                _ => {
                    self.advance();
                    Expression::Error
                },
            }
        } else {
            self.advance();
            Expression::Error
        }
    }

    fn read_binary_op(&mut self) -> Expression {
        let op_keyword = self.current();
        self.advance();

        let value = self.parse_expression();
        if !matches!(self.current(), Some(Token::Into | Token::Off | Token::With)) {
            return Expression::Error;
        }

        self.advance();

        if let Some(Token::Identifier(s)) = self.current() {
            self.advance();

            Expression::BinaryOp {
                operation: BinaryOperation::from_token(op_keyword.unwrap()),
                variable: s,
                value: Box::new(value),
            }
        } else {
            Expression::Error
        }
    }

    fn read_print(&mut self) -> Expression {
        self.advance();
        let value = Box::new(self.parse_expression());
        Expression::Print(value)
    }

    fn read_var_def(&mut self) -> Expression {
        self.advance();

        let value;
        if !matches!(self.current(), Some(Token::Rock)) {
            value = Box::new(self.parse_expression());

            if !matches!(self.current(), Some(Token::Rock)) {
                return Expression::Error;
            }

            self.advance();
        } else {
            self.advance();

            if !matches!(self.current(), Some(Token::Named)) {
                return Expression::Error;
            }

            self.advance();

            value = Box::new(self.parse_expression());
        }

        if !matches!(self.current(), Some(Token::At)) {
            return Expression::Error;
        }

        self.advance();

        let name;
        match self.current() {
            Some(Token::Identifier(s)) => name = s,
            _ => return Expression::Error,
        }

        self.advance();

        Expression::VarDef {
            name,
            value,
        }
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}

impl BinaryOperation {
    fn from_token(token: Token) -> BinaryOperation {
        match token {
            Token::Smash => BinaryOperation::Add,
            Token::Chip => BinaryOperation::Sub,
            Token::Mate => BinaryOperation::Mul,
            _ => panic!("cant parse token into binary operation, did you forget to implement it?"),
        }
    }
}