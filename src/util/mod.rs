fn xor_shift(seed: &mut u32) -> u32 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;
    return x;
}

pub fn randf(seed: &mut u32) -> f64 {
    let randnr = xor_shift(seed);

    randnr as f64 / 4294967296.0_f64
}

pub fn sqrt_f64(x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
        return x;
    }
    let mut guess = x;
    for _ in 0..20 {
        guess = 0.5 * (guess + x / guess);
    }
    guess
}

pub fn powi_f64(base: f64, exp: i32) -> f64 {
    if exp == 0 {
        return 1.0;
    }
    if exp < 0 {
        return 1.0 / powi_f64(base, -exp);
    }
    let mut result = 1.0;
    let mut base = base;
    let mut exp = exp as u32;
    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }
    result
}

pub fn floor_f64(x: f64) -> f64 {
    x as i64 as f64 // lol
}
