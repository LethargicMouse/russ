use std::{cmp::Ordering, fmt::Display};

use crate::{
    die::Mortal,
    link::{
        ast::Ast,
        lex::{
            Token,
            lexeme::Lexeme::{self, *},
        },
    },
    location::Location,
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

    fn error(self) -> Error<'a> {
        let location = self.tokens[self.err_cursor].location;
        let mut msgs = self.msgs;
        msgs.sort();
        msgs.dedup();
        Error { location, msgs }
    }

    fn ast(&mut self) -> Result<Ast, Fail> {
        self.expect(Fun)?;
        self.expect(Name("main"))?;
        self.expect(ParL)?;
        self.expect(ParR)?;
        self.expect(CurL)?;
        self.expect(CurR)?;
        Ok(Ast {})
    }

    fn expect(&mut self, lexeme: Lexeme) -> Result<(), Fail> {
        if self.tokens[self.cursor].lexeme == lexeme {
            self.cursor += 1;
            Ok(())
        } else {
            self.fail(lexeme.show())
        }
    }

    fn fail<T>(&mut self, s: &'static str) -> Result<T, Fail> {
        match self.cursor.cmp(&self.err_cursor) {
            Ordering::Less => {}
            Ordering::Equal => self.msgs.push(s),
            Ordering::Greater => {
                self.err_cursor = self.cursor;
                self.msgs.clear();
                self.msgs.push(s)
            }
        }
        Err(Fail)
    }
}

struct Fail;

struct Error<'a> {
    location: Location<'a>,
    msgs: Vec<&'static str>,
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error parsing {}\n--! expected:", self.location)?;
        for msg in &self.msgs {
            write!(f, "\n    - {msg}")?;
        }
        Ok(())
    }
}
