use crate::{data::generate::delta::random_delta, vec2::Point};

pub fn near(point: Point, radius: f32) -> Point {
    let dx = random_delta(radius);
    let dy = random_delta(radius);
    point + Point::new(dx, dy)
}
