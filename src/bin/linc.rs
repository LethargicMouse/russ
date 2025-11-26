use std::{
    env::{self, args},
    fmt::Display,
};

use russ::{
    code::{Code, read_code},
    die::{Mortal, die},
    file::{self},
    link::{analyse, lex, parse},
    process::{self, call},
    qbe::ir::IR,
};

fn main() {
    let args = get_args();
    match args.command {
        Command::Run(path) => run(path),
    }
}

fn run(path: String) {
    compile(path);
    run_out();
}

fn compile(path: String) {
    let code = read_code(path);
    let ir = process(code);
    dump(ir);
    postcompile();
}

fn get_args() -> Args {
    let mut args = args();
    args.next();
    let command = get_command(&mut args);
    if let Some(arg) = args.next() {
        die(Unexpected("argument", arg))
    }
    Args { command }
}

fn get_command(args: &mut env::Args) -> Command {
    match args.next() {
        Some(s) if &s == "run" => Command::Run(get_path(args)),
        Some(command) => die(Unexpected("command", command)),
        None => die(Expected("command")),
    }
}

fn get_path(args: &mut env::Args) -> String {
    args.next().or_die_with(|_| Expected("path"))
}

struct Expected(&'static str);

impl Display for Expected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} expected {}", ARGS_ERROR, self.0)
    }
}

struct Unexpected(&'static str, String);

impl Display for Unexpected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} unexpected {}: {}", ARGS_ERROR, self.0, self.1)
    }
}

const ARGS_ERROR: &str = "! error reading args:";

struct Args {
    command: Command,
}

enum Command {
    Run(String),
}

fn postcompile() {
    call("qbe", &["-o", "out.s", OUT_PATH]);
    call("cc", &["-o", "out", "out.s"]);
}

fn run_out() {
    process::run("./out", &[]);
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
