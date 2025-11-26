use russ::{
    code::Code,
    file::{self},
    link::{analyse, lex, parse},
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

fn process(code: Code) -> IR {
    let tokens = lex(&code);
    let ast = parse(tokens);
    analyse(ast)
}

fn dump(ir: IR) {
    file::dump(ir, OUT_PATH);
}

const OUT_PATH: &str = "out.qbe";
