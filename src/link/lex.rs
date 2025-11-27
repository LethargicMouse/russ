use std::fmt::Display;

use crate::{die::Mortal, location::Location, source::Source};

pub fn lex(code: &Source) -> Vec<Token> {
    Lex::new(code).run(LIST)
}

struct Lex<'a> {
    source: &'a Source,
    cursor: usize,
}

impl<'a> Lex<'a> {
    fn new(source: &'a Source) -> Self {
        Self { source, cursor: 0 }
    }

    fn run(mut self, list: LexList) -> Vec<Token> {
        let mut res = Vec::new();
        while self.cursor < self.source.code.len() {
            let tok = self.list(list).or_die_with(|_| self.error());
            res.push(tok);
        }
        res.push(self.token(Eof, 1));
        res
    }

    fn error(&'_ self) -> Error<'_> {
        Error(self.location(1))
    }

    fn location(&'_ self, len: usize) -> Location<'_> {
        let source = self.source;
        let start = self.source.poses[self.cursor];
        let end = self.source.poses[self.cursor + len];
        Location { source, start, end }
    }

    fn list(&mut self, list: LexList) -> Option<Token> {
        for (s, lexeme) in list {
            if self.source.code[self.cursor..].starts_with(s) {
                return Some(self.token(*lexeme, s.len()));
            }
        }
        None
    }

    fn token(&mut self, _lexeme: Lexeme, len: usize) -> Token {
        self.cursor += len;
        Token {}
    }
}

struct Error<'a>(Location<'a>);

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error lexing {}\n--! unexpected token", self.0)
    }
}

const LIST: LexList = &[("fn", Fun)];

type LexList<'a> = &'a [(&'a str, Lexeme)];

#[derive(Clone, Copy)]
enum Lexeme {
    Eof,
    Fun,
}

use Lexeme::*;

pub struct Token {}
