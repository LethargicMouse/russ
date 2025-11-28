mod gen_stmts;
use gen_stmts::gen_stmts;

use crate::{link::ast::Ast, qbe::ir::IR};

pub fn analyse(ast: Ast) -> IR {
    let stmts = gen_stmts(ast.expr);
    IR { stmts }
}
