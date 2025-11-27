mod error;
mod parsers;

use crate::{
    die::Mortal,
    link::{ast::Ast, lex::Token, parse::error::Fail},
};

pub fn parse(tokens: Vec<Token>) -> Ast {
    Parse::new(tokens).run(Parse::ast)
}

struct Parse<'a> {
    tokens: Vec<Token<'a>>,
    cursor: usize,
    err_cursor: usize,
    msgs: Vec<&'static str>,
}

impl<'a> Parse<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            cursor: 0,
            err_cursor: 0,
            msgs: Vec::new(),
        }
    }

    fn run<T>(mut self, f: impl FnOnce(&mut Self) -> Result<T, Fail>) -> T {
        f(&mut self).or_die_with(|_| self.error())
    }
}
