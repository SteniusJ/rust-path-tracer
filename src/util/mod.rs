use cuda_device::device;

#[device]
pub fn xor_shift(seed: &mut u32) -> u32 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;
    return x;
}

#[device]
pub fn randf(seed: &mut u32) -> f64 {
    let randnr = xor_shift(seed);

    randnr as f64 / 4294967296.0_f64
}
