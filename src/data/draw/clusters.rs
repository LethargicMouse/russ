use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea},
    style::{BLACK, BLUE, Color, GREEN, RED},
};

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
    let colors = [GREEN.filled(), RED.filled(), BLUE.filled()];
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
