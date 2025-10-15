#![allow(dead_code)]
mod command;
mod death;
mod either;
mod file;
mod k_means;
mod link;
mod pretty;
mod qbe;
mod source;
mod vec2;

fn main() {
    link::compiler::run()
}
