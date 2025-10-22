use std::iter::repeat_with;

use rand::{Rng, rng};

use crate::{
    data::generate::{delta::random_delta, point::random_point},
    vec2::Point,
};

pub fn blobs(blob_count: usize, point_count: usize, radius: f32, delta_radius: f32) -> Vec<Point> {
    let centers: Vec<Point> = repeat_with(|| random_point(radius - delta_radius))
        .take(blob_count)
        .collect();
    repeat_with(|| rng().random_range(0..blob_count))
        .map(|i| {
            let dx = random_delta(delta_radius);
            let dy = random_delta(delta_radius);
            Point::new(centers[i].x + dx, centers[i].y + dy)
        })
        .take(point_count)
        .collect()
}
