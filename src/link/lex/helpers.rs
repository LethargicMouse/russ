use crate::{
    link::lex::{Error, Lex, Token, lexeme::Lexeme},
    location::Location,
};

impl<'a> Lex<'a> {
    pub fn error(&'_ self) -> Error<'_> {
        Error(self.location(1))
    }

    pub fn location(&self, len: usize) -> Location<'a> {
        let source = self.source;
        let start = self.source.poses[self.cursor];
        let end = self.source.poses[self.cursor + len];
        Location { source, start, end }
    }

    pub fn skip(&mut self) {
        self.cursor += self.source.code[self.cursor..]
            .iter()
            .take_while(|c| c.is_ascii_whitespace())
            .count();
    }

    pub fn token(&mut self, lexeme: Lexeme<'a>, len: usize) -> Token<'a> {
        let location = self.location(len);
        self.cursor += len;
        Token { lexeme, location }
    }
}
