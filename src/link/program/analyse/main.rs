mod error;

use crate::{
    death::die,
    link::program::{Program, analyse::main::error::NoMain},
};

pub fn check_main(program: Program) {
    let _ = program;
    die(NoMain {
        src_name: program.name,
    })
}
