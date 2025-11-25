use russ::{
    code::Code,
    file::{self},
    process::{call, run},
    qbe::ir::IR,
};

fn main() {
    let code = read_code();
    let ir = process(code);
    dump(ir);
    postcompile();
    run_out();
}

fn postcompile() {
    call("qbe", &["-o", "out.s", OUT_PATH]);
    call("cc", &["-o", "out", "out.s"]);
}

fn run_out() {
    run("./out", &[]);
}

fn read_code() -> Code {
    Code {}
}

fn process(_code: Code) -> IR {
    IR {}
}

fn dump(ir: IR) {
    file::dump(ir, OUT_PATH);
}

const OUT_PATH: &str = "out.qbe";
