use crate::{
    link::ast::{Ast, fun::parse::fun},
    source::{Source, parser::Parser},
};

pub fn parse_ast(source: &'_ Source) -> Ast<'_> {
    Parser::new(source).parse(ast)
}

pub fn ast<'a>(p: &mut Parser<'a>) -> Result<Ast<'a>, ()> {
    let name = p.source_name();
    let _ = fun(p)?;
    p.eof()?;
    Ok(Ast { name })
}
