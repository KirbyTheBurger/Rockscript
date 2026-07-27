use crate::{error::ParseError, lexer::Token, parser::Parser};

impl Parser {
    pub fn expect(&mut self, token: Token, context: &str) -> Result<(), ParseError> {
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

    pub fn expect_some(&mut self, expected: &str, context: &str) -> Result<Token, ParseError> {
        if let Some(t) = self.current() {
            Ok(t)
        } else {
            Err(ParseError {
                span: self.current_span(),
                desc: format!("expected {} {}, found <EOF>", expected, context)
            })
        }
    }

    pub fn expect_identifier(&mut self, context: &str) -> Result<String, ParseError> {
        match self.current() {
            Some(Token::Identifier(s)) => { self.advance(); Ok(s) }
            _ => Err(ParseError {
                span: self.current_span(),
                desc: format!("expected identifier {}, found {:?}", context, self.current()),
            }),
        }
    }
}