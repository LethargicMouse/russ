use std::{collections::hash_map::Entry, ops::AddAssign};

use crate::{
    death::die,
    link::{
        ast::fun::Fun,
        program::{Program, add::error::AlreadyDeclared},
    },
};

impl<'a> AddAssign<Fun<'a>> for Program<'a> {
    fn add_assign(&mut self, fun: Fun<'a>) {
        match self.funs.entry(fun.name()) {
            Entry::Occupied(entry) => die(AlreadyDeclared {
                view: fun.view(),
                kind: "function",
                name: fun.name(),
                old_view: entry.get().view(),
            }),
            Entry::Vacant(entry) => {
                entry.insert(fun);
            }
        }
    }
}
