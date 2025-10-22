use crate::{
    data::{draw::clusters::draw, generate},
    dbscan::dbscan,
};

pub fn test() {
    let radius = 500.0;
    let points = generate::blobs(10, 1000, radius, 100.0);
    let clusters = dbscan(&points, 45., 10);
    draw(points, clusters, radius);
}
