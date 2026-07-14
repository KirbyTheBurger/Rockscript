use std::ops;

use ariadne::{Color, Label, Report, Source};

#[derive(Debug)]
pub struct LexError(ops::Range<usize>);

impl From<std::ops::Range<usize>> for LexError {
    fn from(value: std::ops::Range<usize>) -> Self {
        LexError(value)
    }
}

pub struct ParseError {
    pub desc: String,
    pub span: std::ops::Range<usize>,
}

pub trait Diagnostic {
    fn span(&self) -> std::ops::Range<usize>;
    fn desc(&self) -> String;
}

impl Diagnostic for LexError {
    fn span(&self) -> std::ops::Range<usize> { self.0.clone() }
    fn desc(&self) -> String { "unexpected character(s)".to_string() }
}

impl Diagnostic for ParseError {
    fn span(&self) -> std::ops::Range<usize> { self.span.clone() }
    fn desc(&self) -> String { self.desc.clone() }
}

fn byte_to_char(source: &str, byte: usize) -> usize {
    source[..byte].chars().count()
}

pub fn report_error(source: &str, filename: &str, error: Box<dyn Diagnostic>) {
    let char_span = 
        byte_to_char(source, error.span().start)..byte_to_char(source, error.span().end);

    Report::build(
        ariadne::ReportKind::Error, (filename, char_span.clone())
    )
        .with_message(error.desc())
        .with_label(
            Label::new((filename, char_span))
                .with_message("here")
                .with_color(Color::Red)
        )
        .finish()
        .eprint((filename, Source::from(source)))
        .unwrap();
}