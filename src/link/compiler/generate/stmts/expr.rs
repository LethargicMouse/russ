mod ret;

use crate::link::{ast::expr::Expr, compiler::generate::stmts::GenStmts};

impl<'a, 'b, 's> GenStmts<'a, 'b, 's> {
    pub fn expr(&self, expr: &Expr) {
        match expr {
            Expr::Unit => {}
        }
    }
}
