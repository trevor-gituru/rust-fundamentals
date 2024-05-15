/*
 * This program demonstrates floatint point types
 * which are numbers with decimal points
 */
fn main() {
    // The default is f64
    let x = 6.22;
    let y: f32 = 2.12;
    let result = x*y;
    println!(" The result of x * y is: {}", result);
    // Floating points are signed
    // f64 has approximately 15 decimal points
    let f64_max = f64::MAX;
    let f64_min = f64::MIN;
    //f32 has approxiamtely 7 decimal points
    let f32_max = f32::MAX;
    let f32_min = f32::MIN;

    println!("The range of f64 is {:e} - {:e}", f64_min, f64_max);
    println!("The range of f32 is {:e} - {:e}", f32_min, f32_max);

    }

