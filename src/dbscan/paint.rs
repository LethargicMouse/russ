use std::iter::repeat_n;

use crate::{dbscan::cores::Cores, vec2::Point};

pub fn paint(points: &[Point], cores: Cores, colors: Vec<usize>) -> Vec<usize> {
    if cores.0.is_empty() {
        return repeat_n(usize::MAX, points.len()).collect();
    }
    points
        .iter()
        .map(|p| {
            let i = p.nearest(&cores.0);
            colors[i]
        })
        .collect()
}
