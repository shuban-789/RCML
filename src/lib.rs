use core::f32::NAN;

#[no_mangle]
pub extern "C" fn verify_limit(c: f32, l: f32, f: extern "C" fn(f32) -> f32) -> bool
{   
    let epsilon = 1e-4;
    let delta_start = 1e-6;

    for i in 1..100 {
        let delta = delta_start * (i as f32);
        let left = c - delta;
        let right = c + delta;

        let fl = f(left);
        let fr = f(right);

        if !fl.is_finite() || !fr.is_finite() {
            return false;
        }

        if (fl - l).abs() > epsilon || (fr - l).abs() > epsilon {
            return false;
        }
    }

    return true;
}

#[no_mangle]
pub extern "C" fn limit(c: f32, f: extern "C" fn(f32) -> f32) -> f32 {
    let mut h = 1e-2;
    let mut prev_r = 0.0;
    let mut curr_r = 0.0;
    let tolerance = 1e-6;

    loop {
        let l1 = (f(c + h) + f(c - h)) / 2.0;
        let l2 = (f(c + h / 2.0) + f(c - h / 2.0)) / 2.0;

        curr_r = (4.0 * l2 - l1) / 3.0;

        if (curr_r - prev_r).abs() < tolerance || h < 1e-10 {
            break;
        }

        prev_r = curr_r;
        h *= 0.5;
    }

    if verify_limit(c, curr_r, f) {
        curr_r
    } else {
        f32::NAN
    }
}

#[no_mangle]
pub extern "C" fn derive(c: f32, f: extern "C" fn(f32) -> f32) -> f32 {
    let h1 = 1e-3;
    let h2 = h1 / 2.0;

    let d1 = (f(c + h1) - f(c - h1)) / (2.0 * h1);
    let d2 = (f(c + h2) - f(c - h2)) / (2.0 * h2);

    let richardson = (4.0 * d2 - d1) / 3.0;

    if richardson.is_finite() {
        richardson
    } else {
        f32::NAN
    }
}



#[no_mangle]
pub extern "C" fn integrate(a: f32, b: f32, f: extern "C" fn(f32) -> f32) -> f32
{
    let n = 1000000;
    let h = (b - a) / (n as f32);
    let mut sum: f64 = (f(a) + f(b)) as f64;
    for i in 1..n {
        let x = a + (i as f32) * h;
        if i % 2 == 0 {
            sum += 2.0 * f(x) as f64;
        } else {
            sum += 4.0 * f(x) as f64;
        };
    }
    return ((h as f64) / 3.0 * sum) as f32;
}

#[no_mangle]
pub extern "C" fn euler(x_init: f32, y_init: f32, x_final: f32, step: f32, d: extern "C" fn(f32, f32) -> f32) -> f32
{
    let mut x_coord = x_init;
    let mut y_coord = y_init;
    let delta_x = step;

    while x_coord < x_final {
        let delta_y = d(x_coord, y_coord) * delta_x;
        y_coord += delta_y;
        x_coord += delta_x;
    }

    return y_coord;
}
