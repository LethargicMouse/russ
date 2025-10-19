use std::collections::HashMap;

use crate::link::ast::fun::Fun;

pub mod analyse;

pub struct Program<'a> {
    name: &'a str,
    funs: HashMap<&'a str, Fun>,
}

impl<'a> Program<'a> {
    pub fn empty(name: &'a str) -> Self {
        Self {
            name,
            funs: HashMap::new(),
        }
    }
}
