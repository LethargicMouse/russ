mod main;

use crate::link::program::{Program, analyse::main::check_main};

pub fn analyse(program: &Program) {
    check_main(program);
}
