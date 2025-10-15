// provides an implementation of `k_means` method of finding a fixed number of clusters for points on a plane
use std::{
    iter::{repeat_n, repeat_with, successors},
    mem::swap,
};

use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea},
    style::{BLACK, Color, GREEN, RED},
};
use rand::{Rng as _, rng};

use crate::{death::OrDie as _, vec2::Vec2};

pub fn test() {
    let blob_count = 2;
    let radius = 500.0;
    let delta_radius = 50.0;
    let points = gen_data(blob_count, radius, delta_radius, DataKind::Worm);
    let clusters = k_means(blob_count, &points, radius);
    draw(points, clusters, radius);
}

fn draw(points: Vec<Point>, clusters: Vec<usize>, radius: f32) {
    let root = BitMapBackend::new("clusters.png", (800, 600)).into_drawing_area();
    root.fill(&BLACK).or_die();
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-radius..radius, -radius..radius)
        .or_die();
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()
        .or_die();
    let colors = [GREEN.filled(), RED.filled()];
    chart
        .draw_series(
            clusters
                .into_iter()
                .zip(points)
                .map(|(i, p)| Circle::new(p.unwrap(), 2, colors[i])),
        )
        .or_die();
    root.present().or_die();
}

pub fn k_means(blob_count: usize, points: &Vec<Point>, radius: f32) -> Vec<usize> {
    KMeans::init(blob_count, points, radius).run()
}

struct KMeans<'a> {
    points: &'a Vec<Point>,
    colors: Vec<usize>,
    centers: Vec<Point>,

    new_centers: Vec<Point>, // stored here to allocate once
    cluster_sizes: Vec<i32>,
}

impl<'a> KMeans<'a> {
    fn init(blob_count: usize, points: &'a Vec<Point>, radius: f32) -> Self {
        let centers: Vec<Point> = repeat_with(|| random_point(radius))
            .take(blob_count)
            .collect();
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
            if self.points[i].dist2(self.centers[res]) < self.points[i].dist2(self.centers[j]) {
                res = j;
            }
        }
        res
    }
}

type Point = Vec2<f32>;

#[allow(unused)]
enum DataKind {
    Blobs,
    Uniform,
    Worm,
}

fn gen_data(blob_count: usize, radius: f32, delta_radius: f32, kind: DataKind) -> Vec<Point> {
    let point_count = 1000;
    match kind {
        DataKind::Blobs => gen_blobs(blob_count, point_count, radius, delta_radius),
        DataKind::Uniform => make_uniform(point_count, radius),
        DataKind::Worm => gen_worm(point_count, delta_radius),
    }
}

fn gen_worm(point_count: usize, delta_radius: f32) -> Vec<Point> {
    successors(Some(Point::new(0., 0.)), |p| Some(near(*p, delta_radius)))
        .take(point_count)
        .collect()
}

fn near(point: Point, radius: f32) -> Point {
    let dx = random_delta(radius);
    let dy = random_delta(radius);
    point + Point::new(dx, dy)
}

fn make_uniform(point_count: usize, radius: f32) -> Vec<Point> {
    repeat_with(|| random_point(radius))
        .take(point_count)
        .collect()
}

fn gen_blobs(blob_count: usize, point_count: usize, radius: f32, delta_radius: f32) -> Vec<Point> {
    let centers: Vec<Point> = repeat_with(|| random_point(radius))
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

fn random_delta(radius: f32) -> f32 {
    rng().random_range(-radius..=radius)
}

fn random_point(radius: f32) -> Point {
    let x = random_delta(radius);
    let y = random_delta(radius);
    Point::new(x, y)
}
