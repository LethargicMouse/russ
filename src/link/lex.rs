use std::fmt::Display;

use crate::{die::Mortal, location::Location, source::Source};

pub fn lex(code: &'_ Source) -> Vec<Token<'_>> {
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

    fn run(mut self, list: LexList<'a>) -> Vec<Token<'a>> {
        let mut res = Vec::new();
        self.skip();
        while self.cursor < self.source.code.len() {
            let tok = self
                .list(list)
                .or_else(|| self.name())
                .or_die_with(|_| self.error());
            res.push(tok);
            self.skip();
        }
        self.cursor -= 1;
        res.push(self.token(Eof, 1));
        res
    }

    fn name(&mut self) -> Option<Token<'a>> {
        if self.cursor == self.source.code.len()
            || !is_name_first_char(self.source.code[self.cursor])
        {
            return None;
        }
        let res_len = self.source.code[self.cursor..]
            .iter()
            .take_while(|c| is_name_char(**c))
            .count();
        let res = &self.source.code[self.cursor..self.cursor + res_len];
        let lexeme = Name(str::from_utf8(res).unwrap());
        Some(self.token(lexeme, res_len))
    }

    fn error(&'_ self) -> Error<'_> {
        Error(self.location(1))
    }

    fn location(&self, len: usize) -> Location<'a> {
        let source = self.source;
        let start = self.source.poses[self.cursor];
        let end = self.source.poses[self.cursor + len];
        Location { source, start, end }
    }

    fn list(&mut self, list: LexList<'a>) -> Option<Token<'a>> {
        for (s, lexeme) in list {
            if self.source.code[self.cursor..].starts_with(s) {
                return Some(self.token(*lexeme, s.len()));
            }
        }
        None
    }

    fn skip(&mut self) {
        self.cursor += self.source.code[self.cursor..]
            .iter()
            .take_while(|c| c.is_ascii_whitespace())
            .count();
    }

    fn token(&mut self, lexeme: Lexeme<'a>, len: usize) -> Token<'a> {
        let location = self.location(len);
        self.cursor += len;
        Token { lexeme, location }
    }
}

fn is_name_first_char(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_name_char(c: u8) -> bool {
    is_name_first_char(c) || c.is_ascii_digit()
}

struct Error<'a>(Location<'a>);

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error lexing {}\n--! unexpected token", self.0)
    }
}

const LIST: LexList = &[
    (b"fn", Fun),
    (b"(", ParL),
    (b")", ParR),
    (b"{", CurL),
    (b"}", CurR),
];

type LexList<'a> = &'a [(&'a [u8], Lexeme<'a>)];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lexeme<'a> {
    Eof,
    Fun,
    Name(&'a str),
    ParL,
    ParR,
    CurL,
    CurR,
}

impl Lexeme<'_> {
    pub fn show(self) -> &'static str {
        match self {
            Eof => "<eof>",
            Fun => "`fn`",
            Name(_) => "<name>",
            ParL => "`(`",
            ParR => "`)`",
            CurL => "`{`",
            CurR => "`}`",
        }
    }
}

use Lexeme::*;

pub struct Token<'a> {
    pub lexeme: Lexeme<'a>,
    pub location: Location<'a>,
}
