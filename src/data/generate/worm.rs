use std::iter::successors;

use crate::{data::generate::point::near, vec2::Point};

pub fn worm(point_count: usize, delta_radius: f32) -> Vec<Point> {
    successors(Some(Point::new(0., 0.)), |p| Some(near(*p, delta_radius)))
        .take(point_count)
        .collect()
}
