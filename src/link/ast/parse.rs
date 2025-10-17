use crate::{
    link::ast::Ast,
    source::{Source, parser::Parser},
};

pub fn parse_ast(source: &'_ Source) -> Ast<'_> {
    Parser::new(source).parse(ast)
}

pub fn ast<'a>(p: &mut Parser<'a>) -> Result<Ast<'a>, ()> {
    let _ = p;
    let name = p.source_name();
    Ok(Ast { name })
}
