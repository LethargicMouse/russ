mod fun;

use std::ops::AddAssign;

use crate::link::{ast::Ast, program::Program};

impl<'a> AddAssign<Ast<'a>> for Program<'a> {
    fn add_assign(&mut self, ast: Ast<'a>) {
        for fun in ast.funs {
            *self += fun;
        }
    }
}
