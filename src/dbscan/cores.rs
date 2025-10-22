mod paint;

use crate::{dbscan::options::Options, vec2::Point};

pub struct Cores(pub Vec<Point>);

impl Cores {
    pub fn find(points: &[Point], options: Options) -> Self {
        Self(
            points
                .iter()
                .copied()
                .filter(|p| is_core(*p, points, options))
                .collect(),
        )
    }
}

fn is_core(point: Point, points: &[Point], options: Options) -> bool {
    points
        .iter()
        .filter(|p| p.dist2(point) < options.eps)
        .count()
        >= options.m
}
