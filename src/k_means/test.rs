use crate::{
    data::draw::clusters::draw,
    k_means::{
        k_means,
        test::data::{Kind, generate},
    },
};

mod data;

pub fn test() {
    let blob_count = 3;
    let radius = 500.0;
    let points = generate(blob_count, radius, Kind::Blobs);
    let clusters = k_means(blob_count, &points, radius);
    draw(points, clusters, radius);
}
