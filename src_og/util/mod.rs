use rand::RngExt;
use rand::rngs::SmallRng;

pub fn randf(rng: &mut SmallRng) -> f64 {
    rng.random_range(0.0..1.0)
}
