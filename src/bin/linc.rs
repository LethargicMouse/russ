use std::{
    env::args,
    fs::File,
    io::{Read, Write},
    process::{Command, exit},
};

use russ::{
    die::either::OrDie,
    file::{create, open, read, run, write},
};

fn main() {
    // get path
    let path = {
        let mut args = args();
        args.next().expect("first arg - process name");
        args.next()
            .or_die_with(|_| "! failed to parse args: expected path to file")
    };

    // read file
    let mut code = String::new();
    File::open(&path)
        .or_die_with(|e| open::Error(&path, e))
        .read_to_string(&mut code)
        .or_die_with(|e| read::Error(path, e));

    // generate ir
    let out_path = "out.qbe";
    let mut out = File::create(out_path).or_die_with(|e| create::Error(out_path, e));
    write!(out, "export function w $main() {{\n@start\n  ret 0\n}}")
        .or_die_with(|e| write::Error(out_path, e));

    // run `qbe`
    run("qbe", ["-o", "out.s", "out.qbe"]);
    run("cc", ["-o", "out", "out.s"]);
    let status = Command::new("./out")
        .status()
        .or_die_with(|e| run::Error("./out", e));
    if !status.success() {
        exit(status.code().unwrap())
    }
}
