use crate::{
    command,
    link::compiler::{args::Args, build::build},
    source::Source,
};

mod args;
mod build;
mod generate;
mod ir_builder;
pub use generate::generate;

pub fn run() {
    let args = Args::get();
    run_file(args.path);
}

fn run_file(path: String) {
    let source = Source::read(path);
    compile(&source);
    command::run("./out", &[]);
}

fn compile(source: &Source) {
    let ir = build(source);
    ir.dump();
    command::run("qbe", &["-o", "out.s", "out.qbe"]);
    command::run("cc", &["-o", "out", "out.s"]);
}
