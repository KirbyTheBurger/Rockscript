use crate::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),

    VarDef {
        name: String,
        value: Box<Expression>,
    },

    Error,
    EOF,
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

    fn read_var_def(&mut self) -> Expression {
        self.advance();

        let value = Box::new(self.parse_expression());

        if !matches!(self.current(), Some(Token::Rock)) {
            return Expression::Error;
        }

        self.advance();

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