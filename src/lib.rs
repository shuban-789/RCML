use core::f32::NAN;

type PathFn = extern "C" fn(f32, *mut f32);

#[no_mangle]
pub extern "C" fn path_x(t: f32, out: *mut f32) {
    unsafe {
        *out.offset(0) = t;
        *out.offset(1) = 0.0;
    }
}

#[no_mangle]
pub extern "C" fn path_diag(t: f32, out: *mut f32) {
    unsafe {
        *out.offset(0) = t;
        *out.offset(1) = t;
    }
}

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
pub extern "C" fn limit2(
    f: extern "C" fn(f32, f32) -> f32, px: f32, py: f32, paths: *const extern "C" fn(f32, *mut f32), num_paths: usize,) -> f32 {
    let mut limits = Vec::new();
    for i in 0..num_paths {
        let path = unsafe { *paths.add(i) };
        let mut t = 1e-3f32;
        let mut prev = f32::NAN;
        let tolerance = 1e-5f32;

        loop {
            let mut coords = [0.0f32, 0.0f32];
            path(t, coords.as_mut_ptr());
            let curr = f(px + coords[0], py + coords[1]);
            if (curr - prev).abs() < tolerance || t < 1e-10 {
                break;
            }
            prev = curr;
            t *= 0.5;
        }
        limits.push(prev);
    }

    let epsilon = 1e-4;
    for i in 1..limits.len() {
        if (limits[i] - limits[0]).abs() > epsilon {
            return f32::NAN;
        }
    }
    limits[0]
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

#[no_mangle]
pub extern "C" fn dot(ptr1: *const f32, ptr2: *const f32, len: usize, out: *mut f32) {
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!out.is_null());

    let v1 = unsafe { std::slice::from_raw_parts(ptr1, len) };
    let v2 = unsafe { std::slice::from_raw_parts(ptr2, len) };

    assert!((v1.len() == v2.len()) && v1.len() == (len as i32).try_into().unwrap());

    let mut dot = 0.0;

    for i in 0..(v1.len()) {
        dot += v1[i]*v2[i];
    }

    unsafe {
        *out = dot;
    }
}


#[no_mangle] 
pub extern "C" fn cross2(ptr1: *const f32, ptr2: *const f32, len: usize, out: *mut f32) {
    assert!(len == 2);
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!out.is_null());   
    
    let v1 = unsafe { std::slice::from_raw_parts(ptr1, len) };
    let v2 = unsafe { std::slice::from_raw_parts(ptr2, len) };

    assert!(v1.len() == v2.len());
    
    let cross =  v1[0]*v2[1] - v1[1]*v2[0];

    unsafe {
        *out = cross;
    }
}

#[no_mangle]
pub extern "C" fn cross3(ptr1: *const f32, ptr2: *const f32, len: usize, out: *mut f32) {
    assert!(len == 3);
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!out.is_null());

    let v1 = unsafe { std::slice::from_raw_parts(ptr1, len) };
    let v2 = unsafe { std::slice::from_raw_parts(ptr2, len) };
    let out = unsafe { std::slice::from_raw_parts_mut(out, 3) };

    assert!(v1.len() == v2.len());

    out[0] = v1[1]*v2[2] - v1[2]*v2[1];
    out[1] = v1[2]*v2[0] - v1[0]*v2[2];
    out[2] = v1[0]*v2[1] - v1[1]*v2[0];
}