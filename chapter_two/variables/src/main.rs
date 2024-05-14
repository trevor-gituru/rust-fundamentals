/* 
 * The following program demonstrates mutability of variables.
 */
fn main() {
    //The following statement declares y as an immutable variable
    let y = 2;
    println!("The value of immutable y is {y}");
    //This means that the following is not possible
    // let y = 4;
    // The following statement declares x as a mutuable variable
    let mut x = 6;
    println!("The value of x is: {x}");
    // This means it is possible to change value of x
    x = 5;
    println!("The new value of x is: {x}");
}
