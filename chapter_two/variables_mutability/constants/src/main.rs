/*
 * This program demonstrates use of constants.
 */
fn main() {
    /*  They are fully immutable(mut not allowed).
     *  Its value should be a constant expression 
     *  and should not be computed at runtime.
     *  Its keyword, naming convention & annotation is as shown below
     */
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    println!("The constant value is: {THREE_HOURS_IN_SECONDS}");
}
