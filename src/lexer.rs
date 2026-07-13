use logos::Logos;

#[derive(Logos, Debug, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),

    #[regex(r"[0-9]+\.?[0-9]*", |lex| lex.slice().parse().ok())]
    Number(f64),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[token("throw")] Throw,
    #[token("rock")] #[token("rocks")] Rock,
    #[token("at")] At,
    #[token("named")] Named,
    #[token("present")] Present,
    #[token("smash")] Smash,
    #[token("into")] Into,
    #[token("chip")] Chip,
    #[token("off")] Off,
    #[token("mate")] Mate,
    #[token("with")] With,
    #[token("split")] Split,
    #[token("from")] From,
    #[token("big")] True,
    #[token("small")] False,
    #[token("carve")] Carve,
    #[token("instruction")] Instruction,
    #[token("retrieve")] Retrieve,
    #[token("enough")] Enough,
    #[token("follow")] Follow,
    #[token("and")] And,
    #[token("engrave")] Engrave,
    #[token("weigh")] Weigh,
    #[token("against")] Against,
    #[token("inspect")] Inspect,
    #[token("refine")] Refine,
    #[token("roll")] Roll,
    #[token("while")] While,
    #[token("destroy")] Destroy,
}