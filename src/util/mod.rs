use rand::RngExt;
use rand::rngs::SmallRng;

pub fn randf(rng: &mut SmallRng) -> f64 {
    rng.random_range(0.0..1.0)
}

pub fn get_randnr_vec(len: usize, rng: &mut SmallRng) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::with_capacity(len);
    
    for _ in 0..len {
        vec.push(randf(rng));
    }

    vec
}
