use crate::link::{
    ast::{Ast, Expr},
    lex::lexeme::Lexeme::{self, *},
    parse::{Fail, Parse},
};

impl<'a> Parse<'a> {
    pub fn ast(&mut self) -> Result<Ast, Fail> {
        self.expect(Fun)?;
        self.expect(Name("main"))?;
        self.expect(ParL)?;
        self.expect(ParR)?;
        self.expect(CurL)?;
        self.expr()?;
        self.expect(CurR)?;
        Ok(Ast {})
    }

    fn expr(&mut self) -> Result<Expr, Fail> {
        todo!()
    }

    fn expect(&mut self, lexeme: Lexeme) -> Result<(), Fail> {
        if self.tokens[self.cursor].lexeme == lexeme {
            self.cursor += 1;
            Ok(())
        } else {
            self.fail(lexeme.show())
        }
    }
}
