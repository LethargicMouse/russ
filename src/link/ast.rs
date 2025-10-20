use crate::link::ast::fun::Fun;

pub mod expr;
pub mod fun;
pub mod parse;
pub mod structure;

pub struct Ast<'a> {
    name: &'a str,
    pub funs: Vec<Fun<'a>>,
}
