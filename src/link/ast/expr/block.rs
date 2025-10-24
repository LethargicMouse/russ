use crate::link::ast::expr::Expr;

pub mod parse;

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Expr>,
    pub ret: Expr,
}
