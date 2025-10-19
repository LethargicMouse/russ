pub mod near;
pub use near::near;

use crate::{data::generate::delta::random_delta, vec2::Point};

pub fn random_point(radius: f32) -> Point {
    let x = random_delta(radius);
    let y = random_delta(radius);
    Point::new(x, y)
}
