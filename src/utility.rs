use rand::rngs::ThreadRng;
use rand::Rng;
use std::f64::consts::PI;
use std::ops::Range;

pub const CLOSEST_TO_ZERO_TO_ONE_RANGE: Range<f64> = f64::MIN_POSITIVE..1.0;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_from_range(range: Range<f64>) -> f64 {
    let mut rng = ThreadRng::default();
    rng.random_range(range.clone())
}
