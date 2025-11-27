pub const LIST: LexList = &[
    (b"fn", Fun),
    (b"(", ParL),
    (b")", ParR),
    (b"{", CurL),
    (b"}", CurR),
];

pub type LexList<'a> = &'a [(&'a [u8], Lexeme<'a>)];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lexeme<'a> {
    Eof,
    Fun,
    Name(&'a str),
    ParL,
    ParR,
    CurL,
    CurR,
    Int(i64),
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
            Int(_) => "<int>",
        }
    }
}

use Lexeme::*;
