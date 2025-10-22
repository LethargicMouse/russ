use crate::{
    dbscan::{cores::Cores, options::Options},
    vec2::Point,
};

mod cores;
mod options;
mod paint;
use paint::paint;
pub mod test;
pub use test::test;

pub fn dbscan(points: &[Point], eps: f32, m: usize) -> Vec<usize> {
    let options = Options { eps: eps * eps, m };
    let cores = Cores::find(points, options);
    let colors = cores.paint(options.eps);
    paint(points, cores, colors)
}
