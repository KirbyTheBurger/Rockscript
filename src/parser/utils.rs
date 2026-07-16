use crate::{error::ParseError, lexer::Token, parser::{Parser, expression::SpannedStatement}};

impl Parser {
    pub fn current(&self) -> Option<Token> {
        self.tokens.get(self.pos)
            .map_or(None, |t| Some(t.token.clone()))
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn current_span(&self) -> std::ops::Range<usize> {
        self.tokens.get(self.pos)
            .map(|t| t.span.clone())
            .unwrap_or_else(|| {
                let end = self.tokens.last().map(|t| t.span.end).unwrap_or(0);
                end..end
            })
    }

    pub fn parse_until(&mut self, stop: &[Token]) -> Result<Vec<SpannedStatement>, ParseError> {
        let mut body = Vec::new();
        while !stop.iter().any(|t| self.current().as_ref() == Some(t)) {
            body.push(self.parse_statement(self.current().unwrap())?);
        }
        Ok(body)
    }
}