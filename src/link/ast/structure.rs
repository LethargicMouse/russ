use crate::link::{ast::Ast, program::Program};

pub fn structure(ast: Ast) -> Program {
    Program::empty(ast.name)
}
