use rand::{Rng, rng};

pub fn random_delta(radius: f32) -> f32 {
    rng().random_range(-radius..=radius)
}
