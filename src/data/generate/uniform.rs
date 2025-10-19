use std::iter::repeat_with;

use crate::{data::generate::point::random_point, vec2::Point};

pub fn uniform(point_count: usize, radius: f32) -> Vec<Point> {
    repeat_with(|| random_point(radius))
        .take(point_count)
        .collect()
}
