use crate::link::{
    ast::{Ast, Call, Expr},
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
        self.either(&[
            Self::unit,
            |p| Ok(Expr::Call(p.call()?)),
            |_| Ok(Expr::Unit),
        ])
    }

    fn call(&mut self) -> Result<Call, Fail> {
        self.name()?;
        self.expect(ParL)?;
        self.expr()?;
        self.expect(ParR)?;
        Ok(Call {})
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

    fn name(&mut self) -> Result<&'a str, Fail> {
        if let Name(n) = self.tokens[self.cursor].lexeme {
            self.cursor += 1;
            Ok(n)
        } else {
            self.fail("name")
        }
    }
}
