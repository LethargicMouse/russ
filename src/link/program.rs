use std::collections::HashMap;

use crate::link::ast::fun::Fun;

mod add;
pub mod analyse;

pub struct Program<'a> {
    name: &'a str,
    pub funs: HashMap<&'a str, Fun<'a>>,
}

impl<'a> Program<'a> {
    pub fn empty(name: &'a str) -> Self {
        Self {
            name,
            funs: HashMap::new(),
        }
    }
}
