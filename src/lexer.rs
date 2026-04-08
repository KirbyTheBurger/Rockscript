#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),

    Throw,
    At,
    Named,
    Rock,

    Present,

    Smash,
    Into,
    Chip,
    Off,
    Mate,
    With,
    Split,
    From,

    Carve,
    Instruction,
    Retrieve,
    Enough,
    Follow,
    And,
    Engrave,

    Weigh,
    Against,

    Inspect,
    Refine,

    Roll,
    While,
    Destroy,

    Error,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(text: &str) -> Lexer {
        Lexer {
            input: text.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current() {
            tokens.push(match c {
                '"' => self.read_string(),
                _ if c.is_numeric() || c == '.' => self.read_number(),
                _ => self.read_identifier(),
            });

            self.skip_whitespace();
        }

        tokens.push(Token::EOF);

        tokens
    }

    fn read_string(&mut self) -> Token {
        self.advance();

        let mut string = String::new();
        while let Some(c) = self.current() {
            if c == '"' {
                break;
            }

            string.push(c);
            self.advance();
        }

        self.advance();

        Token::String(string)
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();

        while let Some(c) = self.current() {
            if !(c.is_numeric() || c == '.') {
                break;
            }

            number.push(c);
            self.advance();
        }

        self.advance();

        match number.parse::<f64>() {
            Ok(n) => Token::Number(n),
            _ => {
                eprintln!("Failed to parse number");
                Token::Error
            },
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();

        while let Some(c) = self.current() {
            if !(c.is_alphanumeric() || c == '_') {
                break;
            }

            identifier.push(c);
                self.advance();
        }

        self.advance();

        match identifier.as_str() {
            "throw" => Token::Throw,
            "rock" | "rocks" => Token::Rock,
            "at" => Token::At,
            "named" => Token::Named,
            "present" => Token::Present,
            "smash" => Token::Smash,
            "into" => Token::Into,
            "chip" => Token::Chip,
            "off" => Token::Off,
            "mate" => Token::Mate,
            "with" => Token::With,
            "split" => Token::Split,
            "from" => Token::From,
            "big" => Token::Boolean(true),
            "small" => Token::Boolean(false),
            "carve" => Token::Carve,
            "instruction" => Token::Instruction,
            "retrieve" => Token::Retrieve,
            "enough" => Token::Enough,
            "follow" => Token::Follow,
            "and" => Token::And,
            "engrave" => Token::Engrave,
            "weigh" => Token::Weigh,
            "against" => Token::Against,
            "inspect" => Token::Inspect,
            "refine" => Token::Refine,
            "roll" => Token::Roll,
            "while" => Token::While,
            "destroy" => Token::Destroy,
            _ => Token::Identifier(identifier),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
}