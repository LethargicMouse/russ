use crate::{
    data::generate::{blobs, uniform, worm},
    vec2::Point,
};

pub fn generate(blob_count: usize, radius: f32, kind: Kind) -> Vec<Point> {
    let point_count = 1000;
    let delta_radius = 200.0;
    match kind {
        Kind::Blobs => blobs(blob_count, point_count, radius, delta_radius),
        Kind::Uniform => uniform(point_count, radius),
        Kind::Worm => worm(point_count, delta_radius),
    }
}

pub enum Kind {
    Blobs,
    Uniform,
    Worm,
}
