use crate::link::{ast::expr::block::Block, compiler::generate::stmts::GenStmts};

impl<'a, 'b, 's> GenStmts<'a, 'b, 's> {
    pub fn block(&self, block: &Block) {
        for stmt in &block.stmts {
            self.expr(stmt);
        }
        self.ret(&block.ret);
    }
}
