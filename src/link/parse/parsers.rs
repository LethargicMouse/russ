use crate::link::{
    ast::{Ast, Expr},
    lex::lexeme::Lexeme::{self, *},
    parse::{Fail, Parse},
};

pub type Parser<'a, T> = fn(&mut Parse<'a>) -> Result<T, Fail>;

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
        self.either(&[Self::unit, |_| Ok(Expr::Unit)])
    }

    fn unit(&mut self) -> Result<Expr, Fail> {
        self.expect(ParL)?;
        self.expect(ParR)?;
        Ok(Expr::Unit)
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
