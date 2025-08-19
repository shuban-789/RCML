fn fact(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}

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
    let mut curr_r;
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
    
    return limits[0];
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

// NOTE: this version of nderive is extremely hard coded. As of now, I don't know the math for a dynamic nderive. Until then, the max degree is 5
#[no_mangle]
pub extern "C" fn nderive(n: usize, x: f32, h: f32, f: extern "C" fn(f32) -> f32) -> f32 {
    let h2 = h / 2.0;

    match n {
        0 => { f(x) }

        1 => {
            let d1 = (f(x + h) - f(x - h)) / (2.0 * h);
            let d2 = (f(x + h2) - f(x - h2)) / (2.0 * h2);
            (4.0 * d2 - d1) / 3.0
        }

        2 => {
            let d1 = (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h);
            let d2 = (f(x + h2) - 2.0 * f(x) + f(x - h2)) / (h2 * h2);
            (4.0 * d2 - d1) / 3.0
        }

        3 => {
            let d1 = (f(x + 2.0 * h) - 2.0 * f(x + h) + 2.0 * f(x - h) - f(x - 2.0 * h)) / (2.0 * h.powi(3));
            let d2 = (f(x + 2.0 * h2) - 2.0 * f(x + h2) + 2.0 * f(x - h2) - f(x - 2.0 * h2)) / (2.0 * h2.powi(3));
            (4.0 * d2 - d1) / 3.0
        }

        4 => {
            let d1 = (-f(x + 2.0 * h) + 16.0 * f(x + h) - 30.0 * f(x) + 16.0 * f(x - h) - f(x - 2.0 * h)) / (12.0 * h.powi(4));
            let d2 = (-f(x + 2.0 * h2) + 16.0 * f(x + h2) - 30.0 * f(x) + 16.0 * f(x - h2) - f(x - 2.0 * h2)) / (12.0 * h2.powi(4));
            (4.0 * d2 - d1) / 3.0
        }

        5 => {
            let d1 = (-f(x + 3.0 * h) + 12.0 * f(x + 2.0 * h) - 39.0 * f(x + h)
                + 39.0 * f(x - h) - 12.0 * f(x - 2.0 * h) + f(x - 3.0 * h)) / (6.0 * h.powi(5));
            let d2 = (-f(x + 3.0 * h2) + 12.0 * f(x + 2.0 * h2) - 39.0 * f(x + h2)
                + 39.0 * f(x - h2) - 12.0 * f(x - 2.0 * h2) + f(x - 3.0 * h2)) / (6.0 * h2.powi(5));
            (4.0 * d2 - d1) / 3.0
        }

        6 => {
            let d1 = (f(x + 3.0 * h) - 6.0 * f(x + 2.0 * h) + 15.0 * f(x + h)
                - 20.0 * f(x) + 15.0 * f(x - h) - 6.0 * f(x - 2.0 * h) + f(x - 3.0 * h)) / h.powi(6);
            let d2 = (f(x + 3.0 * h2) - 6.0 * f(x + 2.0 * h2) + 15.0 * f(x + h2)
                - 20.0 * f(x) + 15.0 * f(x - h2) - 6.0 * f(x - 2.0 * h2) + f(x - 3.0 * h2)) / h2.powi(6);
            (4.0 * d2 - d1) / 3.0
        }

        _ => { f32::NAN }
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
pub extern "C" fn taylor(a: f32, x: f32, d: i32, f: extern "C" fn(f32) -> f32) -> f32 {
    let a = a as f64;
    let x = x as f64;
    let mut sum = 0.0;

    for i in 0..=d {
    let i_usize = i as usize;
        let h = 1e-4_f32;
        let derivative = nderive(i_usize, a as f32, h, f) as f64;
        let term = derivative * (x - a).powi(i) / (fact(i_usize as i32) as f64);
        sum += term;
    }

    return sum as f32;
}

#[no_mangle]
pub extern "C" fn addvec(ptr1: *const f32, ptr2: *const f32, len: usize, out: *mut f32) {
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!out.is_null());

    let v1 = unsafe { std::slice::from_raw_parts(ptr1, len) };
    let v2 = unsafe { std::slice::from_raw_parts(ptr2, len) };
    let out = unsafe { std::slice::from_raw_parts_mut(out, len) };

    assert!(v1.len() == v2.len());

    for i in 0..len {
        out[i] = v1[i] + v2[i];
    }
}

#[no_mangle]
pub extern "C" fn subvec(ptr1: *const f32, ptr2: *const f32, len: usize, out: *mut f32) {
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!out.is_null());

    let v1 = unsafe { std::slice::from_raw_parts(ptr1, len) };
    let v2 = unsafe { std::slice::from_raw_parts(ptr2, len) };
    let out = unsafe { std::slice::from_raw_parts_mut(out, len) };

    assert!(v1.len() == v2.len());

    for i in 0..len {
        out[i] = v1[i] - v2[i];
    }
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
pub extern "C" fn cross2(ptr1: *const f32, ptr2: *const f32, out: *mut f32) {
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!out.is_null());   
    
    let v1 = unsafe { std::slice::from_raw_parts(ptr1, 2) };
    let v2 = unsafe { std::slice::from_raw_parts(ptr2, 2) };

    assert!(v1.len() == v2.len());
    assert!(v1.len() == 2);
    
    let cross =  v1[0]*v2[1] - v1[1]*v2[0];

    unsafe {
        *out = cross;
    }
}

#[no_mangle]
pub extern "C" fn cross3(ptr1: *const f32, ptr2: *const f32, out: *mut f32) {
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!out.is_null());

    let v1 = unsafe { std::slice::from_raw_parts(ptr1, 3) };
    let v2 = unsafe { std::slice::from_raw_parts(ptr2, 3) };
    let out = unsafe { std::slice::from_raw_parts_mut(out, 3) };

    assert!(v1.len() == v2.len());
    assert!(v1.len() == 3);

    out[0] = v1[1]*v2[2] - v1[2]*v2[1];
    out[1] = v1[2]*v2[0] - v1[0]*v2[2];
    out[2] = v1[0]*v2[1] - v1[1]*v2[0];
}

#[no_mangle]
pub extern "C" fn project(ptr1: *const f32, ptr2: *const f32, len: usize, proj: *mut f32) {
    assert!(!ptr1.is_null());
    assert!(!ptr2.is_null());
    assert!(!proj.is_null());

    let pv = unsafe { std::slice::from_raw_parts(ptr1, len) };
    let proj = unsafe { std::slice::from_raw_parts_mut(proj, len) };

    let pv_dot_tv_ptr: *mut f32 = &mut 0.0;
    let pv_dot_pv_ptr: *mut f32 = &mut 0.0;
    dot(ptr1, ptr2, len, pv_dot_tv_ptr);
    dot(ptr1, ptr1, len, pv_dot_pv_ptr);

    let pv_dot_tv: f32 = unsafe { *pv_dot_tv_ptr };
    let pv_dot_pv: f32 = unsafe { *pv_dot_pv_ptr };

    let vec_scalar_prod: f32 = pv_dot_tv / pv_dot_pv;

    for i in 0..len {
        proj[i] = vec_scalar_prod * pv[i];
    }
}

#[no_mangle]
pub extern "C" fn curvature(x: f32, y: f32, f: extern "C" fn(f32) -> f32) -> f32 {
    assert!(y == f(x));
    let first = derive(x, f);
    let second = nderive(2, x, 1e-4_f32, f);
    let k = {
        let numerator = second.abs();
        let denominator: f32 = 1.0 + first.powf(2.0);
        numerator / denominator.powf(1.5)
    };
    return k;
}

#[no_mangle]
pub extern "C" fn gcd(a: i32, b: i32) -> i32 {
    // Euclidean Algorithm
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    let r = a % b;

    return gcd(b, r);
}

#[no_mangle]
pub extern "C" fn egcd(p: i32, q: i32, res: *mut i32) {
    // Extended Euclidean Algorithm
    assert!(!res.is_null());
    let res = unsafe { std::slice::from_raw_parts_mut(res, 3) };

    let mut s = 0;
    let mut t = 1;
    let mut old_s = 1;
    let mut old_t = 0;
    let mut r  = p;
    let mut old_r = q;

    while r != 0 {
        let quo = old_r / r;
        old_r = r;
        r = old_r - quo * r;
        old_s = s;
        s = old_s - quo * s;
        old_t = t;
        t = old_t - quo * t;
    }

    res[0] = old_r; // gcd
    res[1] = old_t; // u
    res[2] = old_s; // v
}