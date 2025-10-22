pub mod test;
use rand::{rng, seq::SliceRandom};
pub use test::test;

use std::{
    iter::{repeat_n, successors},
    mem::swap,
};

use crate::vec2::Point;

pub fn k_means(blob_count: usize, points: &[Point]) -> Vec<usize> {
    KMeans::init(blob_count, points).run()
}

struct KMeans<'a> {
    points: &'a [Point],
    colors: Vec<usize>,
    centers: Vec<Point>,

    new_centers: Vec<Point>, // stored here to allocate once
    cluster_sizes: Vec<i32>,
}

impl<'a> KMeans<'a> {
    fn init(blob_count: usize, points: &'a [Point]) -> Self {
        let centers = choose(blob_count, points);
        let new_centers = centers.clone();
        let colors = repeat_n(0, points.len()).collect();
        let cluster_sizes = repeat_n(0, blob_count).collect();
        Self {
            points,
            centers,
            new_centers,
            colors,
            cluster_sizes,
        }
    }

    fn run(mut self) -> Vec<usize> {
        for _ in 0..1000 {
            self.paint();
            self.find_centers();
            if self.new_centers == self.centers {
                break;
            }
            swap(&mut self.new_centers, &mut self.centers);
        }
        self.colors
    }

    fn paint(&mut self) {
        for i in 0..self.points.len() {
            self.paint_point(i);
        }
    }

    fn find_centers(&mut self) {
        self.cluster_sizes.fill(0);
        self.new_centers.fill(Point::new(0., 0.));
        self.add_points_to_clusters();
        self.norm_new_centers();
    }

    fn norm_new_centers(&mut self) {
        for i in 0..self.centers.len() {
            self.new_centers[i] /= self.cluster_sizes[i] as f32;
        }
    }

    fn add_points_to_clusters(&mut self) {
        for i in 0..self.points.len() {
            self.add_point_to_cluster(i);
        }
    }

    fn add_point_to_cluster(&mut self, i: usize) {
        let color = self.colors[i];
        self.new_centers[color] += self.points[i];
        self.cluster_sizes[color] += 1;
    }

    fn paint_point(&mut self, i: usize) {
        self.colors[i] = self.nearest_to_point(i);
    }

    fn nearest_to_point(&self, i: usize) -> usize {
        let mut res = 0;
        for j in 1..self.centers.len() {
            if self.points[i].dist2(self.centers[res]) > self.points[i].dist2(self.centers[j]) {
                res = j;
            }
        }
        res
    }
}

fn choose(blob_count: usize, points: &[Point]) -> Vec<Point> {
    let mut p: Vec<usize> = successors(Some(0), |n| Some(n + 1))
        .take(points.len())
        .collect();
    p.shuffle(&mut rng());
    p.into_iter().map(|i| points[i]).take(blob_count).collect()
}
