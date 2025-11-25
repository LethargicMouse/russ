use std::fmt::Display;

pub struct IR {}

impl Display for IR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "export function w $main() {{\n@start\nret 0\n}}")
    }
}
