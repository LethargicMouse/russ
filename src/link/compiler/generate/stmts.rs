mod block;
mod expr;

use crate::{
    link::{ast::expr::block::Block, compiler::generate::Generate},
    qbe::ir,
};

pub struct GenStmts<'a, 'b, 's> {
    inner: &'s Generate<'a, 'b>,
    stmts: Vec<ir::Stmt>,
}

impl<'a, 'b, 's> GenStmts<'a, 'b, 's> {
    pub fn new(inner: &'s Generate<'a, 'b>) -> Self {
        let stmts = Vec::new();
        Self { inner, stmts }
    }

    pub fn run(self, body: &Block) -> Vec<ir::Stmt> {
        self.block(body);
        self.stmts
    }
}
