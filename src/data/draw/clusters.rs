use std::iter::repeat_with;

use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea},
    style::{BLACK, Color, RGBColor, full_palette::GREY_800},
};
use rand::{Rng, rng};

use crate::{death::OrDie, vec2::Point};

pub fn draw(points: Vec<Point>, clusters: Vec<usize>, radius: f32) {
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
    let colors: Vec<RGBColor> = repeat_with(|| {
        RGBColor(
            rng().random_range(80..255),
            rng().random_range(80..255),
            rng().random_range(80..255),
        )
    })
    .take(100)
    .collect();
    chart
        .draw_series(clusters.into_iter().zip(points).map(|(i, p)| {
            Circle::new(
                p.unwrap(),
                2,
                if i == usize::MAX {
                    GREY_800.filled()
                } else {
                    colors[i].filled()
                },
            )
        }))
        .or_die();
    root.present().or_die();
}
