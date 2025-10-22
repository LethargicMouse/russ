use std::iter::repeat_n;

use crate::{dbscan::cores::Cores, vec2::Point};

pub struct Paint<'a> {
    cores: &'a Vec<Point>,
    eps: f32,
    colors: Vec<usize>,
    color: usize,
}

impl<'a> Paint<'a> {
    fn new(cores: &'a Cores, eps: f32) -> Self {
        Self {
            cores: &cores.0,
            eps,
            colors: repeat_n(0, cores.0.len()).collect(),
            color: 0,
        }
    }

    fn run(mut self) -> Vec<usize> {
        for i in 0..self.cores.len() {
            if self.colors[i] == 0 {
                self.color += 1;
                self.paint(i);
            }
        }
        self.colors
    }

    fn paint(&mut self, v: usize) {
        self.colors[v] = self.color;
        for i in 0..self.cores.len() {
            if i == v {
                continue;
            }
            if self.colors[i] == 0 && self.are_near(v, i) {
                self.paint(i);
            }
        }
    }

    fn are_near(&self, a: usize, b: usize) -> bool {
        self.cores[a].dist2(self.cores[b]) < self.eps
    }
}

impl Cores {
    pub fn paint(&self, eps: f32) -> Vec<usize> {
        Paint::new(self, eps).run()
    }
}
