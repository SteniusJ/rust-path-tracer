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

/* Newton-Raphson sqrt
 */
#[device]
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

#[device]
pub fn pow_f64(base: f64, exp: f64) -> f64 {
    if base == 0.0 {
        return 0.0;
    }
    if exp == 0.0 {
        return 1.0;
    }
    let ln_base = log_f64(base);
    let exp_result = exp_f64(exp * ln_base);
    exp_result
}

#[device]
pub fn log_f64(x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }

    let mut x = x;
    let mut n = 0;
    while x > 1.0 {
        x *= 2.0;
        n += 1;
    }
    while x < 0.5 {
        x *= 2.0;
        n -= 1;
    }

    let z = x - 1.0;
    let mut result = z;
    let mut term = z;
    for i in 0..20 {
        term *= -z;
        result += term / (i as f64 + 1.0);
    }

    result + (n as f64) * 0.6931471805599453
}

#[device]
pub fn exp_f64(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }

    let n = floor_f64(x / 0.6931471805599453) as i32;
    let x_reduced = x - (n as f64) * 0.6931471805599453;

    let mut result = 1.0;
    let mut term = 1.0;
    for i in 0..20 {
        term *= x_reduced / (i as f64);
        result *= term;
    }

    result * powi_f64(2.0, n)
}

#[device]
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

#[device]
pub fn floor_f64(x: f64) -> f64 {
    x as i64 as f64 // lol
}
