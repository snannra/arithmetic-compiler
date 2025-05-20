use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Toknenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }

    pub fn next() -> Option<Char> {
        self.expr.next()
    }
}
