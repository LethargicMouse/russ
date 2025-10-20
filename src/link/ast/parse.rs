use crate::{
    link::ast::{Ast, fun::parse::fun},
    source::{
        Source,
        parser::{Parser, common::eof, fail::PF},
    },
};

pub fn parse_ast(source: &'_ Source) -> Ast<'_> {
    Parser::new(source).parse(ast)
}

pub fn ast<'a>(p: &mut Parser<'a>) -> Result<Ast<'a>, PF> {
    let name = &p.source.name;
    let _ = fun(p)?;
    eof(p)?;
    Ok(Ast { name })
}
