use crate::link::{ast::Ast, program::Program};

pub fn structure(ast: Ast) -> Program {
    let mut program = Program::empty(ast.name);
    program += ast;
    program
}
