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
    let mut total = 0.0;
    let mut count = 0;
    let delta = 1e-6;

    for i in 1..100 {
        let h = delta * (i as f32);
        let left = c - h;
        let right = c + h;

        let fl = f(left);
        let fr = f(right);

        if fl.is_finite() {
            total += fl;
            count += 1;
        }

        if fr.is_finite() {
            total += fr;
            count += 1;
        }
    }

    if count == 0 {
        return NAN;
    }

    let avg = total / count as f32;

    if verify_limit(c, avg, f) {
        return avg;
    } else {
        return NAN;
    }
}

#[no_mangle]
pub extern "C" fn derive(c: f32, f: extern "C" fn(f32) -> f32) -> f32
{
    let delta = 1e-6;
    let left = f(c - delta);
    let right = f(c + delta);

    if left.is_finite() && right.is_finite() {
        return (right - left) / (2.0 * delta);
    } else {
        panic!("Function is not finite around point {}", c);
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
