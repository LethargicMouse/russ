use std::{fs::File, process::Command};

use russ::{die::either::OrDie, file::create};

fn main() {
    // get path
    // let path = {
    //     let mut args = args();
    //     args.next().expect("first arg - process name");
    //     args.next()
    //         .or_die_with(|_| "! failed to parse args: expected path to file")
    // };

    // read file
    // let mut code = String::new();
    // File::open(&path)
    //     .or_die_with(|e| open::Error(&path, e))
    //     .read_to_string(&mut code)
    //     .or_die_with(|e| read::Error(path, e));

    // generate ir
    let out_path = "out.qbe";
    File::create(out_path).or_die_with(|e| create::Error(out_path, e));

    // run `qbe`
    let output = Command::new("qbe")
        .args(["-o", "out.s", "out.qbe"])
        .output()
        .or_die_with(|_| todo!());
    let _ = output;
    todo!()
}
