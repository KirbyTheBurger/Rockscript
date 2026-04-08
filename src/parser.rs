use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
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
        operation: BinaryOperation,
        variable: String,
        value: Box<Expression>,
    },
    Comparison {
        operation: CmpOperation,
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

    Error,
    EOF,
}

#[derive(Debug, Clone)]
pub enum CmpOperation {
    Weigh,
}

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
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
                Token::Boolean(b) => {
                    self.advance();
                    Expression::Boolean(b)
                }
                Token::Present => self.read_print(),
                Token::Carve => self.read_fn_def(),
                Token::Identifier(s) => {
                    self.advance();
                    Expression::Identifier(s)
                },
                Token::Follow => self.read_fn_call(),
                Token::Smash | Token::Chip | Token::Mate | Token::Split => self.read_binary_op(),
                Token::Engrave => {
                    self.advance();
                    Expression::Return(Box::new(self.parse_expression()))
                },
                Token::Weigh => self.read_comparison(),
                Token::Inspect => self.read_if(),
                Token::Roll => self.read_while(),
                Token::Destroy => {
                    self.advance();
                    Expression::Break
                },
                Token::EOF => Expression::EOF,
                _ => {
                    self.advance();
                    eprintln!("Tried to parse invalid token");
                    Expression::Error
                },
            }
        } else {
            self.advance();
            eprintln!("Token is `None`");
            Expression::Error
        }
    }

    fn read_if(&mut self) -> Expression {
        self.advance();
        let condition = Box::new(self.parse_expression());

        let mut body = Vec::new();
        while !matches!(self.current(), Some(Token::Refine | Token::Enough)) {
            body.push(self.parse_expression());
        }

        match self.current() {
            Some(Token::Enough) => {
                self.advance();

                return Expression::If {
                    condition,
                    body,
                    else_: None,
                }
            },
            Some(Token::Refine) => {
                self.advance();

                let mut else_ = Vec::new();
                if let Some(Token::Inspect) = self.current() {
                    else_.push(self.read_if());
                } else {
                    while !matches!(self.current(), Some(Token::Enough)) {
                        else_.push(self.parse_expression());
                    }
                    self.advance();
                }

                return Expression::If {
                    condition,
                    body,
                    else_: Some(else_)
                }
            },
            _ => {
                eprintln!("Failed parsing end of if statement. This shouldnt happen and if it does its a bug in the interpreter.");
                return Expression::Error;
            },
        }
    }

    fn read_while(&mut self) -> Expression {
        self.advance();
        if !matches!(self.current(), Some(Token::While)) {
            eprintln!("Missing `while` in loop");
            return Expression::Error;
        }
        self.advance();

        let condition = Box::new(self.parse_expression());

        let mut body = Vec::new();
        while !matches!(self.current(), Some(Token::Enough)) {
            body.push(self.parse_expression());
        }
        self.advance();

        Expression::While {
            condition,
            body,
        }
    }

    fn read_comparison(&mut self) -> Expression {
        let operation = CmpOperation::from_token(self.current().unwrap());
        self.advance();

        let left = Box::new(self.parse_expression());
        
        if !matches!(self.current(), Some(Token::Against)) {
            eprintln!("Incorrect keyword inside of comparison");
            return Expression::Error;
        }
        self.advance();

        let right = Box::new(self.parse_expression());

        Expression::Comparison {
            operation,
            left,
            right
        }
    }

    fn read_fn_call(&mut self) -> Expression {
        self.advance();
        
        let name;
        if let Some(Token::Identifier(s)) = self.current() {
            name = s;
        } else {
            eprintln!("Missing identifier inside of function call");
            return Expression::Error;
        }
        self.advance();

        let mut args = Vec::new();
        while let Some(Token::With | Token::And) = self.current() {
            self.advance();
            args.push(self.parse_expression());
        }

        Expression::FnCall {
            name,
            args
        }
    }

    fn read_fn_def(&mut self) -> Expression {
        self.advance();

        if !matches!(self.current(), Some(Token::Instruction)) {
            eprintln!("Missing `instruction` inside of function definition");
            return Expression::Error;
        }
        self.advance();
        if !matches!(self.current(), Some(Token::Into)) {
            eprintln!("Missing `into` inside of function definition");
            return Expression::Error;
        }
        self.advance();

        let name;
        if let Some(Token::Identifier(s)) = self.current() {
            name = s;
        } else {
            eprintln!("Missing identifier inside of function definition");
            return Expression::Error;
        }
        self.advance();

        let mut params = Vec::new();
        while let Some(Token::Retrieve) = self.current() {
            self.advance();

            if let Some(Token::Identifier(s)) = self.current() {
                params.push(s);
                self.advance();
            } else {
                eprintln!("Missing identifier after `retrieve` inside of function definition");
                return Expression::Error;
            }
        }

        let mut body = Vec::new();
        while !matches!(self.current(), Some(Token::Enough)) {
            body.push(self.parse_expression());
        }
        self.advance();

        Expression::FnDef {
            name,
            params,
            body
        }
    }

    fn read_binary_op(&mut self) -> Expression {
        let op_keyword = self.current();
        self.advance();

        let value = self.parse_expression();
        if !matches!(self.current(), Some(Token::Into | Token::Off | Token::With | Token::From)) {
            eprintln!("Incorrect keyword `{:?}` inside of binary operation `{:?}`", self.current(), op_keyword);
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
            eprintln!("Missing second identifier inside of binary operation `{:?}`", op_keyword);
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
                eprintln!("Missing `Rock(s)` inside variable definition");
                return Expression::Error;
            }

            self.advance();
        } else {
            self.advance();

            if !matches!(self.current(), Some(Token::Named)) {
                eprintln!("Missing `Named` inside string variable definition");
                return Expression::Error;
            }

            self.advance();

            value = Box::new(self.parse_expression());
        }

        if !matches!(self.current(), Some(Token::At)) {
            eprintln!("Missing `at` inside variable definition");
            return Expression::Error;
        }

        self.advance();

        let name;
        match self.current() {
            Some(Token::Identifier(s)) => name = s,
            _ => {
                eprintln!("Missing identifier inside of variable definition");
                return Expression::Error
            },
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
            Token::Split => BinaryOperation::Div,
            _ => panic!("cant parse token into binary operation, this shouldnt panic unless theres a bug in the interpreter"),
        }
    }
}

impl CmpOperation {
    fn from_token(token :Token) -> CmpOperation {
        match token {
            Token::Weigh => CmpOperation::Weigh,
            _ => panic!("cant parse token into binary operation, this shouldnt panic unless theres a bug in the interpreter"),
        }
    }
}