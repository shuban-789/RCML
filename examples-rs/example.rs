extern "C" {
    fn derive(c: f32, f: extern "C" fn(f32) -> f32) -> f32;
    fn integrate(a: f32, b: f32, f: extern "C" fn(f32) -> f32) -> f32;
    fn euler(x_init: f32, y_init: f32, x_final: f32, step: f32, d: extern "C" fn(f32, f32) -> f32) -> f32;
    fn limit(c: f32, f: extern "C" fn(f32) -> f32) -> f32;
    fn verify_limit(c: f32, l: f32, f: extern "C" fn(f32) -> f32) -> bool;
}

extern "C" fn square(x: f32) -> f32 {
    x * x
}

extern "C" fn dy_dx(x: f32, y: f32) -> f32 {
    x + y
}

fn main() {
    unsafe {
        let d = derive(2.0, square);
        println!("f'(2.0) = {}", d);

        let i = integrate(0.0, 1.0, square);
        println!("Integral of x² from 0 to 1: {}", i);

        let y_final = euler(0.0, 1.0, 1.0, 0.01, dy_dx);
        println!("Euler approx y(1): {}", y_final);

        let l = limit(0.0, square);
        println!("Limit of x^2 as x -> 0: {}", l);

        let is_l = verify_limit(0.0, l, square);
        println!("Is limit verified? {}", is_l);
    }
}
